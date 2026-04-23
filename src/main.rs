//! ssh-menu: interactive TUI SSH connection manager.

mod config;
mod ssh;
mod tui;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "ssh-menu",
    version,
    about = "Interactive TUI SSH connection manager",
    long_about = None
)]
struct Cli {
    /// Path to config file (default: ~/.ssh-menu.toml, or $SSH_MENU_CONFIG)
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Launch the interactive TUI (default when no subcommand given)
    Tui,
    /// List all hosts in the config
    List,
    /// Import entries from ~/.ssh/config (merges, skips duplicates)
    Import {
        /// Override path to ssh config (default: ~/.ssh/config)
        #[arg(long)]
        from: Option<PathBuf>,
    },
    /// Connect directly to a host by name, no TUI
    Connect { name: String },
    /// Print the resolved config file path
    Path,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg_path = cli.config.unwrap_or_else(config::default_config_path);

    match cli.cmd.unwrap_or(Cmd::Tui) {
        Cmd::Path => {
            println!("{}", cfg_path.display());
        }
        Cmd::List => {
            let cfg = config::load(&cfg_path)?;
            if cfg.hosts.is_empty() {
                eprintln!("no hosts. edit {} or run `ssh-menu import`", cfg_path.display());
            }
            for h in &cfg.hosts {
                println!("{}", h.display_line());
            }
        }
        Cmd::Import { from } => {
            let src = from.unwrap_or_else(|| {
                dirs::home_dir().unwrap_or_default().join(".ssh/config")
            });
            let incoming = ssh::parse_ssh_config(&src)
                .with_context(|| format!("parse {}", src.display()))?;
            let n_in = incoming.len();
            let mut cfg = config::load(&cfg_path)?;
            let (added, skipped) = ssh::merge_into(&mut cfg, incoming);
            config::save(&cfg_path, &cfg)?;
            println!("parsed {} entries from {} — added {}, skipped {} (already present)",
                n_in, src.display(), added, skipped);
            println!("saved: {}", cfg_path.display());
        }
        Cmd::Connect { name } => {
            let cfg = config::load(&cfg_path)?;
            let Some(h) = cfg.hosts.iter().find(|h| h.name == name).cloned() else {
                anyhow::bail!("host '{}' not found", name);
            };
            let code = ssh::connect(&cfg, &h)?;
            std::process::exit(code);
        }
        Cmd::Tui => {
            let cfg = config::load(&cfg_path)?;
            if cfg.hosts.is_empty() {
                eprintln!("No hosts yet. Either:");
                eprintln!("  - press 'a' in the TUI to add one");
                eprintln!("  - run: ssh-menu import");
                eprintln!("  - edit {}", cfg_path.display());
                eprintln!();
            }
            let picked = tui::run(cfg.clone(), cfg_path.clone())?;
            if let Some(h) = picked {
                let args = ssh::build_ssh_args(&cfg, &h);
                eprintln!("$ ssh {}", args.join(" "));
                let code = ssh::connect(&cfg, &h)?;
                std::process::exit(code);
            }
        }
    }
    Ok(())
}
