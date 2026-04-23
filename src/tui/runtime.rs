use super::app::{App, ExitAction};
use super::events::handle_key;
use super::view::draw;
use crate::config::{Config, Host};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::path::PathBuf;

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
