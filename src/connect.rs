use crate::config::{Config, Host};
use anyhow::Result;
use std::process::Command;

/// Build the `ssh` argv for a given host (including ProxyJump resolution).
pub fn build_ssh_args(cfg: &Config, target: &Host) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();

    if let Some(k) = &target.key {
        let expanded = expand_tilde(k);
        args.push("-i".into());
        args.push(expanded);
    }
    if target.port != 22 {
        args.push("-p".into());
        args.push(target.port.to_string());
    }
    if let Some(jname) = &target.jump {
        if let Some(jump_host) = cfg.hosts.iter().find(|h| &h.name == jname) {
            let jstr = if jump_host.port == 22 {
                format!("{}@{}", jump_host.user, jump_host.host)
            } else {
                format!("{}@{}:{}", jump_host.user, jump_host.host, jump_host.port)
            };
            args.push("-J".into());
            args.push(jstr);
        } else {
            // Pass raw (might be a host in ~/.ssh/config)
            args.push("-J".into());
            args.push(jname.clone());
        }
    }
    for e in &target.extra {
        args.push(e.clone());
    }
    args.push(format!("{}@{}", target.user, target.host));
    args
}

fn expand_tilde(p: &str) -> String {
    if let Some(stripped) = p.strip_prefix("~/") {
        if let Some(h) = dirs::home_dir() {
            return h.join(stripped).to_string_lossy().into_owned();
        }
    }
    if p == "~" {
        if let Some(h) = dirs::home_dir() {
            return h.to_string_lossy().into_owned();
        }
    }
    p.to_string()
}

pub fn connect(cfg: &Config, target: &Host) -> Result<i32> {
    let args = build_ssh_args(cfg, target);
    let status = Command::new("ssh").args(&args).status()?;
    Ok(status.code().unwrap_or(0))
}
