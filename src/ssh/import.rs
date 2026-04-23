use crate::config::{Config, Host};
use anyhow::Result;
use std::path::Path;

/// Parse ~/.ssh/config into hosts.
/// Supports: Host, HostName, User, Port, IdentityFile, ProxyJump.
/// Skips wildcard entries (contains '*' or '?').
pub fn parse_ssh_config(path: &Path) -> Result<Vec<Host>> {
    if !path.exists() {
        return Ok(vec![]);
    }
    let text = std::fs::read_to_string(path)?;
    let mut hosts: Vec<Host> = Vec::new();
    let mut cur: Option<Host> = None;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let mut parts = line.splitn(2, |c: char| c.is_whitespace() || c == '=');
        let key = parts.next().unwrap_or("").trim().to_lowercase();
        let val = parts.next().unwrap_or("").trim().trim_matches('"').to_string();
        if val.is_empty() { continue; }

        match key.as_str() {
            "host" => {
                if let Some(h) = cur.take() {
                    if !h.host.is_empty() { hosts.push(h); }
                }
                let name = val.split_whitespace().next().unwrap_or("").to_string();
                if name.contains('*') || name.contains('?') {
                    cur = None;
                } else {
                    cur = Some(Host {
                        name: name.clone(),
                        host: String::new(),
                        user: "root".into(),
                        port: 22,
                        key: None,
                        group: Some("ssh_config".into()),
                        tags: vec![],
                        jump: None,
                        note: None,
                        extra: vec![],
                    });
                }
            }
            _ => {
                let Some(h) = cur.as_mut() else { continue; };
                match key.as_str() {
                    "hostname"     => h.host = val,
                    "user"         => h.user = val,
                    "port"         => h.port = val.parse().unwrap_or(22),
                    "identityfile" => h.key = Some(val),
                    "proxyjump"    => h.jump = Some(val),
                    _ => {}
                }
            }
        }
    }
    if let Some(h) = cur {
        if !h.host.is_empty() { hosts.push(h); }
    }
    for h in hosts.iter_mut() {
        if h.host.is_empty() { h.host = h.name.clone(); }
    }
    // Dedup by name: keep last occurrence (OpenSSH "later overrides" semantics).
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut deduped: Vec<Host> = Vec::with_capacity(hosts.len());
    for h in hosts.into_iter().rev() {
        if seen.insert(h.name.clone()) {
            deduped.push(h);
        }
    }
    deduped.reverse();
    Ok(deduped)
}

pub fn merge_into(cfg: &mut Config, incoming: Vec<Host>) -> (usize, usize) {
    let mut added = 0;
    let mut skipped = 0;
    let existing: std::collections::HashSet<String> =
        cfg.hosts.iter().map(|h| h.name.clone()).collect();
    for h in incoming {
        if existing.contains(&h.name) {
            skipped += 1;
        } else {
            cfg.hosts.push(h);
            added += 1;
        }
    }
    (added, skipped)
}
