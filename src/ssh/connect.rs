use crate::config::{Config, Host};
use anyhow::{Context, Result};
use std::path::PathBuf;
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

/// Locate the `ssh` executable with a multi-step fallback chain.
fn find_ssh() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("SSH_MENU_SSH") {
        let pb = PathBuf::from(p);
        if pb.exists() { return Ok(pb); }
    }

    if Command::new("ssh").arg("-V").output().is_ok() {
        return Ok(PathBuf::from("ssh"));
    }

    #[cfg(windows)]
    {
        let candidates = [
            r"C:\Windows\System32\OpenSSH\ssh.exe",
            r"C:\Program Files\OpenSSH\ssh.exe",
            r"C:\Program Files (x86)\OpenSSH\ssh.exe",
            r"C:\Program Files\Git\usr\bin\ssh.exe",
            r"C:\Program Files\Git\mingw64\bin\ssh.exe",
        ];
        for c in candidates {
            let pb = PathBuf::from(c);
            if pb.exists() { return Ok(pb); }
        }
    }

    #[cfg(unix)]
    {
        for c in ["/usr/bin/ssh", "/usr/local/bin/ssh", "/opt/homebrew/bin/ssh"] {
            let pb = PathBuf::from(c);
            if pb.exists() { return Ok(pb); }
        }
    }

    anyhow::bail!(
        "`ssh` executable not found.\n\n\
         Install OpenSSH client, or set $SSH_MENU_SSH to its full path.\n\
         - Windows 10+:  Settings -> Apps -> Optional features -> add 'OpenSSH Client'\n\
         - macOS:        built-in, usually /usr/bin/ssh\n\
         - Linux:        install the openssh-client package"
    )
}

pub fn connect(cfg: &Config, target: &Host) -> Result<i32> {
    let args = build_ssh_args(cfg, target);
    let ssh = find_ssh()?;
    let status = Command::new(&ssh)
        .args(&args)
        .status()
        .with_context(|| format!("failed to launch {}", ssh.display()))?;
    Ok(status.code().unwrap_or(0))
}

/// Current timestamp as RFC3339 in local time, e.g. "2026-04-24T15:30:12+08:00".
pub fn now_rfc3339() -> String {
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};
    let t = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    t.format(&Rfc3339).unwrap_or_default()
}

/// Human-friendly "time ago" from an RFC3339 string.
pub fn time_ago(rfc3339: &str) -> String {
    use time::{format_description::well_known::Rfc3339, OffsetDateTime};
    let Ok(past) = OffsetDateTime::parse(rfc3339, &Rfc3339) else { return rfc3339.to_string(); };
    let now = OffsetDateTime::now_utc();
    let secs = (now - past).whole_seconds();
    if secs < 0 { return "just now".into(); }
    match secs {
        0..=59            => format!("{}s ago", secs),
        60..=3599         => format!("{}m ago", secs / 60),
        3600..=86399      => format!("{}h ago", secs / 3600),
        86400..=2591999   => format!("{}d ago", secs / 86400),
        _                 => format!("{}mo ago", secs / 2592000),
    }
}
