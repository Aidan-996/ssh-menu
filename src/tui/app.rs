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
    Help,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortBy {
    Name,
    Group,
    Recent,
    MostUsed,
}

impl SortBy {
    pub fn label(&self) -> &'static str {
        match self {
            SortBy::Name => "name",
            SortBy::Group => "group",
            SortBy::Recent => "recent",
            SortBy::MostUsed => "most-used",
        }
    }
    pub fn next(&self) -> Self {
        match self {
            SortBy::Name => SortBy::Group,
            SortBy::Group => SortBy::Recent,
            SortBy::Recent => SortBy::MostUsed,
            SortBy::MostUsed => SortBy::Name,
        }
    }
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
    pub show_details: bool,
    pub sort_by: SortBy,
}

impl App {
    pub fn new(cfg: Config, cfg_path: PathBuf) -> Self {
        let mut s = Self {
            cfg, cfg_path,
            query: String::new(),
            list_state: ListState::default(),
            mode: Mode::Normal,
            status: "按 ? 查看帮助 · Enter 连接 · a 添加 · / 搜索 · q 退出".into(),
            filtered: vec![],
            exit: None,
            show_details: true,
            sort_by: SortBy::Name,
        };
        s.refilter();
        if !s.filtered.is_empty() { s.list_state.select(Some(0)); }
        s
    }

    pub fn refilter(&mut self) {
        let mut indices: Vec<usize> = self.cfg.hosts.iter().enumerate()
            .filter(|(_, h)| h.matches(&self.query))
            .map(|(i, _)| i)
            .collect();
        let hosts = &self.cfg.hosts;
        indices.sort_by(|&a, &b| {
            let ha = &hosts[a];
            let hb = &hosts[b];
            match self.sort_by {
                SortBy::Name => ha.name.to_lowercase().cmp(&hb.name.to_lowercase()),
                SortBy::Group => {
                    let ga = ha.group.as_deref().unwrap_or("");
                    let gb = hb.group.as_deref().unwrap_or("");
                    ga.cmp(gb).then_with(|| ha.name.cmp(&hb.name))
                }
                SortBy::Recent => hb.last_used.cmp(&ha.last_used),
                SortBy::MostUsed => hb.use_count.cmp(&ha.use_count),
            }
        });
        self.filtered = indices;
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
            Ok(_) => self.status = format!("✓ saved to {}", self.cfg_path.display()),
            Err(e) => self.status = format!("✗ save failed: {}", e),
        }
    }

    pub fn move_sel(&mut self, delta: i32) {
        if self.filtered.is_empty() { return; }
        let len = self.filtered.len() as i32;
        let cur = self.list_state.selected().unwrap_or(0) as i32;
        let n = ((cur + delta) % len + len) % len;
        self.list_state.select(Some(n as usize));
    }

    pub fn toggle_sort(&mut self) {
        self.sort_by = self.sort_by.next();
        self.refilter();
        self.status = format!("✓ sort: {}", self.sort_by.label());
    }

    pub fn toggle_details(&mut self) {
        self.show_details = !self.show_details;
        self.status = format!("✓ details panel: {}", if self.show_details { "on" } else { "off" });
    }

    /// Jump to the first host whose name starts with the given char (case-insensitive).
    pub fn jump_to_letter(&mut self, c: char) {
        let lc = c.to_ascii_lowercase();
        let cur = self.list_state.selected().unwrap_or(0);
        // search starting from cur+1, wrap around
        let len = self.filtered.len();
        for step in 1..=len {
            let i = (cur + step) % len;
            let idx = self.filtered[i];
            if let Some(first) = self.cfg.hosts[idx].name.chars().next() {
                if first.to_ascii_lowercase() == lc {
                    self.list_state.select(Some(i));
                    self.status = format!("→ jumped to '{}'", self.cfg.hosts[idx].name);
                    return;
                }
            }
        }
        self.status = format!("no host starting with '{}'", c);
    }
}
