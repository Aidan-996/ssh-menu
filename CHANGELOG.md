# Changelog

All notable changes to **ssh-menu** are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Planned
- Per-host pre/post hooks (notify on connect, run local command)
- Export back to OpenSSH `~/.ssh/config`
- Theme customization (color schemes)
- Fuzzy matching (currently substring only)
- Multi-select + batch operations (bulk delete / bulk re-group)
- sshfs / scp quick actions
- Unit tests for parser and argv builder

## [0.3.0] - 2026-04-24

Major UI redesign: dark tech theme, card-based details panel, status dots,
and key-pill shortcut bar.

### Added
- **Dark tech theme** with neon cyan (`#7FDBFF`) + purple (`#AA82FF`)
  accents on a near-black blue background (`#0F111A`).
- **Status dots** on every host row: green (used within 24h), blue (within
  a week), grey (older / never), yellow when a jump host is present.
- **Card-based details panel** split into `CONNECTION`, `META`, `USAGE`,
  `SSH COMMAND`, `SHORTCUTS`, `ABOUT` sections.
- **Iconified fields** in details: 🌐 host · 👤 user · 🔌 port · 🔑 key ·
  ↪ jump · 📝 note · ⚙ extra.
- **Group pills** (colored background) and **tag chips** in details.
- **Shortcut key-pills** rendered as keycap-style badges with
  action-semantic colors (connect=green, delete=red, help=yellow, etc.).
- **About / copyright block** fixed at the bottom of details: version,
  author, repo URL, MIT license.
- **ASCII banner + about card** when no host is selected (empty filter
  state still shows something useful).
- **Split header**: left panel shows app title + search state, right
  panel shows live stats `⚡ N hosts · ⇅ sort · ◉ details`.
- **Key-pill footer** in normal mode replaces the plain text hints.

### Changed
- `user@host:port` rendered with distinct colors per component (purple /
  green / blue) instead of a single green run.
- Empty-state prompts upgraded to keycap-styled buttons (`a`, `ssh-menu
  import` as highlighted pills).
- Form editor title becomes a pill badge.
- Help overlay shortcuts all rendered in keycap style.

## [0.2.1] - 2026-04-24

Visual polish release.

### Changed
- Rounded borders and a soft cyan RGB accent throughout the UI.
- List rows gain a `│` separator between columns; index uses `·` beyond 9.
- Groups auto-colorize via a stable hash → 9-color palette for instant
  visual grouping.
- Usage counter rendered as a dim `×N` suffix.
- Tags rendered as `#tag` in italic blue.
- Jump marker spelled out as `↪ name`.
- Details panel shows the ssh command with `$ ssh ...` prefix and a
  bottom hint line.
- Form hints prefixed with a 💡 glyph.
- Help overlay reorganized with section dividers and Chinese labels.
- Empty-state and no-match states get emoji guidance (✨, 😶).
- Header decorates with `╼ ╾` bracket glyphs.
- Footer status text fully localized to Chinese.

### Docs
- Release notes under `.github/release-notes/` trimmed to concise
  per-version summaries.

## [0.2.0] - 2026-04-24

Major TUI overhaul with rich visuals, usage tracking, sorting, details pane,
help overlay, and quick-jump. Config model is backward-compatible.

### Added
- **Colored host list** with semantic colors: index (dim), group (magenta),
  name (cyan, bold), connection string (green), jump marker `↪` (yellow),
  tags (blue). Selected row uses a bold blue highlight with a `▶` cursor.
- **Details panel** (right 45% of body, toggle with `i`): shows every field
  of the selected host, usage stats (`connects`, `last`), and the **full
  equivalent `ssh` command** that would be executed.
- **Usage tracking**:
  - New `last_used` (RFC3339) and `use_count` fields on each host.
  - Automatically incremented on every successful connect (both via TUI
    Enter and `ssh-menu connect NAME`).
  - Rendered as human-friendly relative time: `3h ago`, `2d ago`, `5mo ago`.
  - Preserved across host edits.
- **Four sort modes** cycled with `s`: name → group → recent → most-used.
- **Help overlay** (`?`): full keyboard reference, rendered as a centered
  popup with sections (Navigation / Actions / View / Exit / Search / Form).
- **Quick-jump**:
  - Digits `1`–`9` jump to the Nth visible host.
  - Letters jump to the next host whose name starts with that letter
    (case-insensitive, wraps).
- **Form field hints**: active field shows a one-line description below it
  (required / format / defaults).
- **View equivalent ssh command**: press `y` in normal mode to display the
  `ssh` invocation for the selected host in the status bar.
- **PageUp/PageDown** scroll 10 items at a time.
- **Home/End** as aliases for `g`/`G`.
- **Ctrl-U** clears the current input (both search and form fields).
- **Smart search Enter**: if exactly one entry matches, pressing Enter in
  search mode connects directly.
- **Header chip** shows total host count, current sort mode, and whether
  the details panel is on.
- **Empty-state guidance**: friendly messages when the host list is empty
  or a filter has no matches.

### Changed
- **Delete key**: now `D` (Shift+d) instead of `d`, to avoid accidental
  deletes. The confirmation prompt now starts with a `⚠` glyph to emphasize
  the irreversible action.
- **Status bar** colorizes by context: cyan for search/form/help, red for
  confirm, yellow for general status.
- **Title bar** shows app name + version + live stats.

### Dependencies
- Added `time = "0.3"` for RFC3339 timestamps and relative time rendering.

## [0.1.1] - 2026-04-24

### Fixed
- **Duplicate host entries on import.** When `~/.ssh/config` contained the same
  `Host` alias multiple times, every occurrence was imported. Now dedup by name
  keeping the **last** occurrence, matching OpenSSH's "later overrides"
  semantics.

### Added
- **Robust `ssh` executable discovery.** Previously `Command::new("ssh")` would
  fail with an opaque "program not found" in terminals where `ssh` was not on
  `PATH` (typical in Windows CMD / PowerShell without OpenSSH Client installed).
  The new fallback chain is:
  1. `$SSH_MENU_SSH` environment variable (manual override).
  2. `ssh` on `PATH` (probed via `ssh -V`).
  3. Well-known Windows paths: `System32\OpenSSH\ssh.exe`, Git for Windows, etc.
  4. Well-known Unix paths: `/usr/bin/ssh`, `/usr/local/bin/ssh`, homebrew.
- Clear install guidance printed when no `ssh` binary can be located.
- Bilingual README (Chinese first, English second) with a release-history
  section.

### Changed
- **Modular code structure.** The flat `src/*.rs` layout was refactored into
  three self-contained modules:
  - `config/` — `model` (structs + helpers) and `store` (load/save/path).
  - `ssh/` — `connect` (argv + ssh spawn) and `import` (openssh config parser).
  - `tui/` — `app` (state machine), `form` (add/edit state), `events`
    (keyboard dispatch), `view` (rendering), `runtime` (terminal lifecycle +
    event loop).
  No behavior change; internal only.

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

[Unreleased]: https://github.com/Aidan-996/ssh-menu/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/Aidan-996/ssh-menu/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/Aidan-996/ssh-menu/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Aidan-996/ssh-menu/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Aidan-996/ssh-menu/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Aidan-996/ssh-menu/releases/tag/v0.1.0
