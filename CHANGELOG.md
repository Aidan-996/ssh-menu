# Changelog

All notable changes to **ssh-menu** are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Planned
- SSH key picker / generator inside the TUI
- Per-host pre/post hooks (notify on connect, run local command)
- Export back to OpenSSH `~/.ssh/config`
- Theme customization (color schemes)
- Fuzzy matching (currently substring only)
- Unit tests for `ssh_config` parser and `build_ssh_args`

## [0.1.0] - 2026-04-24

### Added
- **Interactive TUI** (`ratatui` + `crossterm`) with a full-screen host list.
- **Vim-style navigation**: `j`/`k`/arrows, `g`/`G` jump to top/bottom.
- **Live search** (`/`) across name, host, user, group, and tags.
- **CRUD in TUI**: `a` add, `e` edit, `d` delete (with `y/N` confirmation).
- **TOML configuration** at `~/.ssh-menu.toml`
  - Overridable via `--config` flag or `$SSH_MENU_CONFIG` env var.
  - Fields: `name`, `host`, `user`, `port`, `key`, `group`, `tags`, `jump`, `note`, `extra`.
- **ProxyJump / bastion support** — `jump` field references another host's name; ssh-menu builds the correct `-J user@host:port` argument.
- **Import from `~/.ssh/config`**: `ssh-menu import` parses `Host`/`HostName`/`User`/`Port`/`IdentityFile`/`ProxyJump`, skips wildcard entries, merges without duplicates.
- **Dedup on import**: when the same `Host` alias appears multiple times in `~/.ssh/config`, only the last occurrence is kept (matches OpenSSH "later overrides" semantics).
- **CLI subcommands**:
  - `ssh-menu` / `ssh-menu tui` — launch TUI (default)
  - `ssh-menu list` — print all hosts
  - `ssh-menu connect <name>` — connect directly without TUI
  - `ssh-menu import [--from <path>]` — import from ssh config
  - `ssh-menu path` — print resolved config file path
- **System `ssh` execution** — ssh-menu builds argv and execs the system `ssh`, so your existing keys, agent, `known_hosts`, and config all continue to work.
- **Cross-platform release workflow** — GitHub Actions builds binaries for:
  - `x86_64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `aarch64-apple-darwin`
  - `x86_64-pc-windows-msvc`
- **MIT License**.

[Unreleased]: https://github.com/Aidan-996/ssh-menu/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Aidan-996/ssh-menu/releases/tag/v0.1.0
