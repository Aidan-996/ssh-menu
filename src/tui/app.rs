use crate::config::{self, Config, Host};
use super::form::FormState;
use ratatui::widgets::ListState;
use std::path::PathBuf;

pub enum ExitAction {
    Quit,
    Connect(Host),
}

pub enum Mode {
    Normal,
    Search,
    Form(FormState),
    Confirm(String, Box<dyn FnOnce(&mut App)>),
}

pub struct App {
    pub cfg: Config,
    pub cfg_path: PathBuf,
    pub query: String,
    pub list_state: ListState,
    pub mode: Mode,
    pub status: String,
    pub filtered: Vec<usize>,
    pub exit: Option<ExitAction>,
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

    pub fn refilter(&mut self) {
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

    pub fn selected_host(&self) -> Option<&Host> {
        let sel = self.list_state.selected()?;
        let idx = *self.filtered.get(sel)?;
        self.cfg.hosts.get(idx)
    }

    pub fn selected_index(&self) -> Option<usize> {
        let sel = self.list_state.selected()?;
        self.filtered.get(sel).copied()
    }

    pub fn save(&mut self) {
        match config::save(&self.cfg_path, &self.cfg) {
            Ok(_) => self.status = format!("saved {}", self.cfg_path.display()),
            Err(e) => self.status = format!("save failed: {}", e),
        }
    }

    pub fn move_sel(&mut self, delta: i32) {
        if self.filtered.is_empty() { return; }
        let len = self.filtered.len() as i32;
        let cur = self.list_state.selected().unwrap_or(0) as i32;
        let n = ((cur + delta) % len + len) % len;
        self.list_state.select(Some(n as usize));
    }
}
