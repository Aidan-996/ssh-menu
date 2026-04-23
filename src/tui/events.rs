use super::app::{App, ExitAction, Mode};
use super::form::FormState;
use crossterm::event::{KeyCode, KeyModifiers};

pub fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    let mode = std::mem::replace(&mut app.mode, Mode::Normal);
    match mode {
        Mode::Normal => handle_normal(app, code, mods),
        Mode::Search => handle_search(app, code, mods),
        Mode::Form(fs) => handle_form(app, fs, code, mods),
        Mode::Confirm(msg, cb) => handle_confirm(app, msg, cb, code),
    }
}

fn handle_normal(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.exit = Some(ExitAction::Quit),
        KeyCode::Char('c') if mods.contains(KeyModifiers::CONTROL) => app.exit = Some(ExitAction::Quit),
        KeyCode::Down | KeyCode::Char('j') => app.move_sel(1),
        KeyCode::Up | KeyCode::Char('k') => app.move_sel(-1),
        KeyCode::Char('g') => { if !app.filtered.is_empty() { app.list_state.select(Some(0)); } }
        KeyCode::Char('G') => {
            if !app.filtered.is_empty() {
                app.list_state.select(Some(app.filtered.len() - 1));
            }
        }
        KeyCode::Enter => {
            if let Some(h) = app.selected_host().cloned() {
                app.exit = Some(ExitAction::Connect(h));
            }
        }
        KeyCode::Char('/') => {
            app.query.clear();
            app.refilter();
            app.mode = Mode::Search;
            return;
        }
        KeyCode::Char('a') => {
            app.mode = Mode::Form(FormState::new(None));
            return;
        }
        KeyCode::Char('e') => {
            if let Some(h) = app.selected_host().cloned() {
                let mut f = FormState::new(Some(&h));
                f.editing_index = app.selected_index();
                app.mode = Mode::Form(f);
                return;
            }
        }
        KeyCode::Char('d') => {
            if let Some(h) = app.selected_host().cloned() {
                let idx = app.selected_index().unwrap();
                let name = h.name.clone();
                app.mode = Mode::Confirm(
                    format!("Delete '{}'? (y/N)", name),
                    Box::new(move |a: &mut App| {
                        a.cfg.hosts.remove(idx);
                        a.refilter();
                        a.save();
                    }),
                );
                return;
            }
        }
        _ => {}
    }
    app.mode = Mode::Normal;
}

fn handle_search(app: &mut App, code: KeyCode, _mods: KeyModifiers) {
    match code {
        KeyCode::Esc => { app.query.clear(); app.refilter(); }
        KeyCode::Enter => {}
        KeyCode::Backspace => { app.query.pop(); app.refilter(); app.mode = Mode::Search; return; }
        KeyCode::Char(c) => { app.query.push(c); app.refilter(); app.mode = Mode::Search; return; }
        KeyCode::Down => { app.mode = Mode::Search; app.move_sel(1); return; }
        KeyCode::Up => { app.mode = Mode::Search; app.move_sel(-1); return; }
        _ => { app.mode = Mode::Search; return; }
    }
    app.mode = Mode::Normal;
}

fn handle_form(app: &mut App, mut fs: FormState, code: KeyCode, mods: KeyModifiers) {
    match code {
        KeyCode::Esc => { app.mode = Mode::Normal; return; }
        KeyCode::Tab | KeyCode::Down => {
            fs.cursor = (fs.cursor + 1) % fs.fields.len();
        }
        KeyCode::BackTab | KeyCode::Up => {
            fs.cursor = if fs.cursor == 0 { fs.fields.len() - 1 } else { fs.cursor - 1 };
        }
        KeyCode::Backspace => { fs.fields[fs.cursor].1.pop(); }
        KeyCode::Char(c) if mods.contains(KeyModifiers::CONTROL) && c == 's' => {
            if save_form(app, &fs) { return; }
        }
        KeyCode::Enter => {
            if save_form(app, &fs) { return; }
        }
        KeyCode::Char(c) => { fs.fields[fs.cursor].1.push(c); }
        _ => {}
    }
    app.mode = Mode::Form(fs);
}

fn save_form(app: &mut App, fs: &FormState) -> bool {
    if let Some(h) = fs.to_host() {
        match fs.editing_index {
            Some(i) => app.cfg.hosts[i] = h,
            None => app.cfg.hosts.push(h),
        }
        app.refilter();
        app.save();
        app.mode = Mode::Normal;
        true
    } else {
        app.status = "name and host are required".into();
        false
    }
}

fn handle_confirm(app: &mut App, _msg: String, cb: Box<dyn FnOnce(&mut App)>, code: KeyCode) {
    if matches!(code, KeyCode::Char('y') | KeyCode::Char('Y')) {
        cb(app);
    }
    app.mode = Mode::Normal;
}
