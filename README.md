# ssh-menu

> Interactive TUI SSH connection manager. Never type a long `ssh -i ... -p ... user@host` again.

[![crates.io](https://img.shields.io/crates/v/ssh-menu.svg)](https://crates.io/crates/ssh-menu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

```
┌─ SSH Menu ────────────────────────────────────────┐
│ > prod-db       root@db.example.com     [prod]    │
│   staging-web   root@web.staging.lan    [staging] │
│   cloud-vps     ubuntu@vps.example.net  [cloud]   │
│   bastion-jump  admin@10.0.0.1:2222     [infra]   │
└────────────────────────────────────────────────────┘
 Enter=connect  a=add  e=edit  d=delete  /=search  q=quit
```

## Features

- 📋 **TUI list** with vim-style navigation (`j/k`, `g/G`, arrows)
- 🔍 **Instant fuzzy search** by name, host, user, group, or tag
- ➕ **Built-in CRUD** — add / edit / delete hosts without leaving the TUI
- 🗂️ **Groups & tags** — organize bastions, prod, staging, clouds
- 🔀 **ProxyJump / bastion** support (`-J` is built for you)
- 📥 **Import from `~/.ssh/config`** — zero migration cost
- 🔑 **Identity files, custom ports, extra flags** per host
- 🚀 **Single binary** in Rust — no runtime, no deps
- 🌐 Uses your system `ssh`, so agent forwarding, known_hosts, and keys just work

## Install

### From source

```bash
cargo install ssh-menu
```

### Pre-built binaries

Grab the latest from the [Releases](https://github.com/Aidan-996/ssh-menu/releases) page
(Linux / macOS / Windows).

### Build locally

```bash
git clone https://github.com/Aidan-996/ssh-menu
cd ssh-menu
cargo build --release
# binary at target/release/ssh-menu
```

## Quick start

```bash
# First run — import your existing SSH config
ssh-menu import

# Launch the TUI
ssh-menu

# Or jump straight to a host by name
ssh-menu connect prod-db
```

## Configuration

Config lives at `~/.ssh-menu.toml` (override with `--config` or `$SSH_MENU_CONFIG`).

```toml
[[hosts]]
name  = "prod-db"
host  = "db.example.com"
user  = "root"
port  = 22
key   = "~/.ssh/id_rsa"
group = "prod"
tags  = ["mysql", "linux"]

[[hosts]]
name  = "win-rdp-gateway"
host  = "10.0.0.5"
user  = "admin"
port  = 3389
group = "internal"
jump  = "prod-db"        # ProxyJump through another named host
extra = ["-o", "ServerAliveInterval=30"]

[[hosts]]
name  = "cloud-vps"
host  = "vps.example.net"
user  = "ubuntu"
group = "cloud"
tags  = ["public"]
```

You can edit the file directly, or use the TUI (`a` add / `e` edit / `d` delete)
which writes it back for you.

## Commands

| Command                  | What it does                                       |
| ------------------------ | -------------------------------------------------- |
| `ssh-menu`               | Launch interactive TUI (default)                   |
| `ssh-menu list`          | Print all hosts, one per line                      |
| `ssh-menu connect NAME`  | Connect directly, skipping the TUI                 |
| `ssh-menu import`        | Merge entries from `~/.ssh/config`                 |
| `ssh-menu import --from` | Import from a custom ssh config path               |
| `ssh-menu path`          | Print the resolved config file path                |

## Key bindings

**Normal mode**

| Key              | Action                  |
| ---------------- | ----------------------- |
| `↑`/`↓` or `j/k` | Move selection          |
| `g` / `G`        | Jump to top / bottom    |
| `Enter`          | Connect to selected     |
| `/`              | Enter search mode       |
| `a`              | Add new host            |
| `e`              | Edit selected           |
| `d`              | Delete selected         |
| `q` / `Esc`      | Quit                    |

**Search mode**

- Type any character to filter live
- `Esc` — clear filter
- `↑`/`↓` — move selection while filter stays

**Form mode** (add/edit)

- `Tab` / `↑` / `↓` — change field
- `Enter` or `Ctrl-S` — save
- `Esc` — cancel

## FAQ

**Q: Does it replace `ssh`?**
No — it just builds the right argv (including `-i`, `-p`, `-J`, and any extras)
and execs your system `ssh`. All your existing agent / key / known_hosts config is respected.

**Q: Can I still use `~/.ssh/config`?**
Yes. Either (a) run `ssh-menu import` to pull entries in, or (b) use names from your
`ssh_config` as a `jump` target — ssh-menu will pass them to `-J` unchanged.

**Q: Windows?**
Works on Windows 10+ with OpenSSH installed (`ssh.exe` in `PATH`). Terminal rendering
uses `crossterm`, which is cross-platform.

## Roadmap

- [ ] SSH key picker / generator
- [ ] Per-host pre/post commands (e.g. notify on connect)
- [ ] Import/export to OpenSSH config
- [ ] Theme customization
- [ ] Fuzzy matching (currently substring)

## License

MIT © 2026 [Aidan-996](https://github.com/Aidan-996)
