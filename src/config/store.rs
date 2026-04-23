use super::model::Config;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

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
