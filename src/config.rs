use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub hosts: Vec<Host>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub name: String,
    pub host: String,
    #[serde(default = "default_user")]
    pub user: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra: Vec<String>,
}

fn default_user() -> String { "root".into() }
fn default_port() -> u16 { 22 }

impl Host {
    pub fn display_line(&self) -> String {
        let grp = self.group.as_deref().unwrap_or("");
        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", self.tags.join(","))
        };
        let port = if self.port == 22 { String::new() } else { format!(":{}", self.port) };
        let jump = self.jump.as_deref().map(|j| format!(" via {}", j)).unwrap_or_default();
        format!("{:<12} {:<22} {}@{}{}{}{}", grp, self.name, self.user, self.host, port, jump, tags)
    }

    pub fn matches(&self, q: &str) -> bool {
        if q.is_empty() { return true; }
        let q = q.to_lowercase();
        let hay = format!("{} {} {} {} {}",
            self.name, self.host, self.user,
            self.group.as_deref().unwrap_or(""),
            self.tags.join(" ")
        ).to_lowercase();
        hay.contains(&q)
    }
}

pub fn default_config_path() -> PathBuf {
    if let Ok(p) = std::env::var("SSH_MENU_CONFIG") {
        return PathBuf::from(p);
    }
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".ssh-menu.toml")
}

pub fn load(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    let s = std::fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let cfg: Config = toml::from_str(&s)
        .with_context(|| format!("parse {}", path.display()))?;
    Ok(cfg)
}

pub fn save(path: &Path, cfg: &Config) -> Result<()> {
    let s = toml::to_string_pretty(cfg)?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).ok();
    }
    std::fs::write(path, s).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
