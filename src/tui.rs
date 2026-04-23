use crate::config::{self, Config, Host};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Clear},
    Terminal,
};
use std::io;
use std::path::PathBuf;

pub enum ExitAction {
    Quit,
    Connect(Host),
}

enum Mode {
    Normal,
    Search,
    Form(FormState),
    Confirm(String, Box<dyn FnOnce(&mut App)>),
}

struct FormState {
    editing_index: Option<usize>,
    fields: Vec<(String, String)>, // (label, value)
    cursor: usize,
}

impl FormState {
    fn new(h: Option<&Host>) -> Self {
        let def = |v: &str| v.to_string();
        let h_def = Host {
            name: String::new(), host: String::new(),
            user: "root".into(), port: 22,
            key: None, group: None, tags: vec![],
            jump: None, note: None, extra: vec![],
        };
        let src = h.unwrap_or(&h_def);
        Self {
            editing_index: None,
            fields: vec![
                ("name".into(), def(&src.name)),
                ("host".into(), def(&src.host)),
                ("user".into(), def(&src.user)),
                ("port".into(), src.port.to_string()),
                ("key".into(), src.key.clone().unwrap_or_default()),
                ("group".into(), src.group.clone().unwrap_or_default()),
                ("tags".into(), src.tags.join(",")),
                ("jump".into(), src.jump.clone().unwrap_or_default()),
                ("note".into(), src.note.clone().unwrap_or_default()),
            ],
            cursor: 0,
        }
    }

    fn to_host(&self) -> Option<Host> {
        let get = |i: usize| self.fields[i].1.clone();
        let name = get(0);
        let host = get(1);
        if name.is_empty() || host.is_empty() { return None; }
        let port: u16 = get(3).parse().unwrap_or(22);
        let key = { let v = get(4); if v.is_empty() { None } else { Some(v) } };
        let group = { let v = get(5); if v.is_empty() { None } else { Some(v) } };
        let tags: Vec<String> = get(6).split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        let jump = { let v = get(7); if v.is_empty() { None } else { Some(v) } };
        let note = { let v = get(8); if v.is_empty() { None } else { Some(v) } };
        Some(Host {
            name, host,
            user: { let v = get(2); if v.is_empty() { "root".into() } else { v } },
            port, key, group, tags, jump, note, extra: vec![],
        })
    }
}

pub struct App {
    pub cfg: Config,
    pub cfg_path: PathBuf,
    pub query: String,
    pub list_state: ListState,
    mode: Mode,
    status: String,
    filtered: Vec<usize>,
    exit: Option<ExitAction>,
}

impl App {
    pub fn new(cfg: Config, cfg_path: PathBuf) -> Self {
        let mut s = Self {
            cfg, cfg_path,
            query: String::new(),
            list_state: ListState::default(),
            mode: Mode::Normal,
            status: "Enter=connect  a=add  e=edit  d=delete  /=search  q=quit".into(),
            filtered: vec![],
            exit: None,
        };
        s.refilter();
        if !s.filtered.is_empty() { s.list_state.select(Some(0)); }
        s
    }

    fn refilter(&mut self) {
        self.filtered = self.cfg.hosts.iter().enumerate()
            .filter(|(_, h)| h.matches(&self.query))
            .map(|(i, _)| i)
            .collect();
        if self.filtered.is_empty() {
            self.list_state.select(None);
        } else if self.list_state.selected().map_or(true, |s| s >= self.filtered.len()) {
            self.list_state.select(Some(0));
        }
    }

    fn selected_host(&self) -> Option<&Host> {
        let sel = self.list_state.selected()?;
        let idx = *self.filtered.get(sel)?;
        self.cfg.hosts.get(idx)
    }

    fn selected_index(&self) -> Option<usize> {
        let sel = self.list_state.selected()?;
        self.filtered.get(sel).copied()
    }

    fn save(&mut self) {
        match config::save(&self.cfg_path, &self.cfg) {
            Ok(_) => self.status = format!("saved {}", self.cfg_path.display()),
            Err(e) => self.status = format!("save failed: {}", e),
        }
    }
}

pub fn run(cfg: Config, cfg_path: PathBuf) -> Result<Option<Host>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(cfg, cfg_path);
    let result = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result?;
    match app.exit {
        Some(ExitAction::Connect(h)) => Ok(Some(h)),
        _ => Ok(None),
    }
}

fn event_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| draw(f, app))?;
        if app.exit.is_some() { break; }
        if let Event::Key(k) = event::read()? {
            if k.kind != KeyEventKind::Press { continue; }
            handle_key(app, k.code, k.modifiers);
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    // Extract mode temporarily so we can match without borrow issues.
    let mode = std::mem::replace(&mut app.mode, Mode::Normal);
    match mode {
        Mode::Normal => {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => app.exit = Some(ExitAction::Quit),
                KeyCode::Char('c') if mods.contains(KeyModifiers::CONTROL) => app.exit = Some(ExitAction::Quit),
                KeyCode::Down | KeyCode::Char('j') => move_sel(app, 1),
                KeyCode::Up | KeyCode::Char('k') => move_sel(app, -1),
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
        Mode::Search => {
            match code {
                KeyCode::Esc => { app.query.clear(); app.refilter(); }
                KeyCode::Enter => {}
                KeyCode::Backspace => { app.query.pop(); app.refilter(); }
                KeyCode::Char(c) => { app.query.push(c); app.refilter(); app.mode = Mode::Search; return; }
                KeyCode::Down => { app.mode = Mode::Search; move_sel(app, 1); return; }
                KeyCode::Up => { app.mode = Mode::Search; move_sel(app, -1); return; }
                _ => { app.mode = Mode::Search; return; }
            }
            app.mode = Mode::Normal;
        }
        Mode::Form(mut fs) => {
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
                    if let Some(h) = fs.to_host() {
                        match fs.editing_index {
                            Some(i) => app.cfg.hosts[i] = h,
                            None => app.cfg.hosts.push(h),
                        }
                        app.refilter();
                        app.save();
                        app.mode = Mode::Normal;
                        return;
                    } else {
                        app.status = "name and host are required".into();
                    }
                }
                KeyCode::Enter => {
                    if let Some(h) = fs.to_host() {
                        match fs.editing_index {
                            Some(i) => app.cfg.hosts[i] = h,
                            None => app.cfg.hosts.push(h),
                        }
                        app.refilter();
                        app.save();
                        app.mode = Mode::Normal;
                        return;
                    } else {
                        app.status = "name and host are required".into();
                    }
                }
                KeyCode::Char(c) => { fs.fields[fs.cursor].1.push(c); }
                _ => {}
            }
            app.mode = Mode::Form(fs);
        }
        Mode::Confirm(msg, cb) => {
            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') => { cb(app); }
                _ => {}
            }
            let _ = msg;
            app.mode = Mode::Normal;
        }
    }
}

fn move_sel(app: &mut App, delta: i32) {
    if app.filtered.is_empty() { return; }
    let len = app.filtered.len() as i32;
    let cur = app.list_state.selected().unwrap_or(0) as i32;
    let n = ((cur + delta) % len + len) % len;
    app.list_state.select(Some(n as usize));
}

fn draw(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Length(3)])
        .split(f.area());

    // Search bar / title
    let title = match &app.mode {
        Mode::Search => format!("/ {}_", app.query),
        _ => if app.query.is_empty() { "ssh-menu".into() } else { format!("filter: {}", app.query) },
    };
    let header = Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL).title("SSH Menu"));
    f.render_widget(header, chunks[0]);

    // List
    let items: Vec<ListItem> = app.filtered.iter().filter_map(|i| {
        app.cfg.hosts.get(*i).map(|h| ListItem::new(h.display_line()))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(" Hosts ({}) ", app.filtered.len())))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    // Footer
    let footer_text = match &app.mode {
        Mode::Search => "Type to filter  Esc=clear  Up/Down=move  Enter=back to list".to_string(),
        Mode::Form(_) => "Tab/Up/Down=field  Enter or Ctrl-S=save  Esc=cancel".to_string(),
        Mode::Confirm(m, _) => m.clone(),
        Mode::Normal => app.status.clone(),
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);

    // Form overlay
    if let Mode::Form(fs) = &app.mode {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);
        let mut lines: Vec<Line> = vec![
            Line::from(Span::styled(
                if fs.editing_index.is_some() { "Edit host" } else { "Add host" },
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];
        for (i, (k, v)) in fs.fields.iter().enumerate() {
            let marker = if i == fs.cursor { "> " } else { "  " };
            let style = if i == fs.cursor {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{}{:<8}: ", marker, k), style),
                Span::raw(v.clone()),
                Span::styled(if i == fs.cursor { "_" } else { "" }, style),
            ]));
        }
        let p = Paragraph::new(lines).block(
            Block::default().borders(Borders::ALL).title(" Host editor ")
        );
        f.render_widget(p, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
