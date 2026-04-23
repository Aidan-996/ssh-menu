# ssh-menu

> 交互式 TUI SSH 连接管理器｜Interactive TUI SSH connection manager

[![crates.io](https://img.shields.io/crates/v/ssh-menu.svg)](https://crates.io/crates/ssh-menu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/Aidan-996/ssh-menu/actions/workflows/ci.yml/badge.svg)](https://github.com/Aidan-996/ssh-menu/actions/workflows/ci.yml)

```
┌─ SSH Menu ────────────────────────────────────────┐
│ > prod-db       root@db.example.com     [prod]    │
│   staging-web   root@web.staging.lan    [staging] │
│   cloud-vps     ubuntu@vps.example.net  [cloud]   │
│   bastion-jump  admin@10.0.0.1:2222     [infra]   │
└────────────────────────────────────────────────────┘
 Enter=连接/connect  a=添加/add  e=编辑/edit  d=删除/delete  /=搜索/search  q=退出/quit
```

---

## 🇨🇳 中文

告别反复敲 `ssh -i ... -p ... user@host`。单二进制 Rust TUI，跨平台，零依赖。

### 特性

- 📋 **TUI 主机列表**，vim 风格快捷键（`j/k`、`g/G`、方向键）
- 🔍 **实时搜索**：按名称、主机、用户、分组、标签过滤
- ➕ **内置增删改**：TUI 里直接加/改/删，自动写回配置
- 🗂️ **分组 + 标签**，一眼看清生产/测试/云主机
- 🔀 **ProxyJump / 跳板机**支持，自动拼 `-J`
- 📥 **一键导入 `~/.ssh/config`**，零迁移成本
- 🔑 每主机独立的密钥、端口、额外参数
- 🚀 Rust 单二进制，无运行时依赖
- 🌐 调用系统 `ssh`，agent / known_hosts / 密钥完全复用

### 安装

```bash
cargo install ssh-menu
```

或去 [Releases](https://github.com/Aidan-996/ssh-menu/releases) 下载预编译二进制（Linux / macOS / Windows）。

### 快速上手

```bash
# 首次：导入现有 SSH 配置
ssh-menu import

# 打开 TUI
ssh-menu

# 直接连接（不开 TUI）
ssh-menu connect prod-db
```

### 配置文件

默认 `~/.ssh-menu.toml`（可用 `--config` 或 `$SSH_MENU_CONFIG` 覆盖）。

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
jump  = "prod-db"                    # 通过另一台已命名主机做跳板
extra = ["-o", "ServerAliveInterval=30"]
```

### 命令

| 命令                     | 说明                             |
| ------------------------ | -------------------------------- |
| `ssh-menu`               | 启动交互式 TUI（默认）           |
| `ssh-menu list`          | 列出所有主机                     |
| `ssh-menu connect NAME`  | 不开 TUI，直接连接               |
| `ssh-menu import`        | 从 `~/.ssh/config` 合并导入      |
| `ssh-menu import --from` | 从指定路径导入                   |
| `ssh-menu path`          | 打印配置文件路径                 |

### 快捷键

| 按键             | 动作                    |
| ---------------- | ----------------------- |
| `↑`/`↓` 或 `j/k` | 移动光标                |
| `g` / `G`        | 跳到首项 / 末项         |
| `Enter`          | 连接当前选中            |
| `/`              | 进入搜索模式            |
| `a`              | 添加新主机              |
| `e`              | 编辑当前选中            |
| `d`              | 删除当前选中            |
| `q` / `Esc`      | 退出                    |

**表单模式**：`Tab` 切字段，`Enter` 或 `Ctrl-S` 保存，`Esc` 取消。

### 环境变量

| 变量                | 作用                                              |
| ------------------- | ------------------------------------------------- |
| `SSH_MENU_CONFIG`   | 覆盖默认配置文件路径                              |
| `SSH_MENU_SSH`      | 指定 `ssh` 可执行文件路径（PATH 找不到时用）      |

### FAQ

**Q：它会替换 `ssh` 吗？**
不会。ssh-menu 只负责拼参数然后 exec 系统 `ssh`，你现有的 agent、密钥、known_hosts 全部不受影响。

**Q：Windows 支持吗？**
支持。需要系统装有 OpenSSH Client（`ssh.exe`）。ssh-menu 会自动查找常见位置：`C:\Windows\System32\OpenSSH\`、`C:\Program Files\Git\usr\bin\` 等。

---

## 🇬🇧 English

Skip the long `ssh -i ... -p ... user@host`. Single-binary Rust TUI, cross-platform, no runtime deps.

### Features

- 📋 **TUI host list** with vim-style navigation (`j/k`, `g/G`, arrows)
- 🔍 **Instant fuzzy search** — name, host, user, group, or tag
- ➕ **Built-in CRUD** — add / edit / delete hosts without leaving the TUI
- 🗂️ **Groups & tags** — organize bastions, prod, staging, clouds
- 🔀 **ProxyJump / bastion** support — `-J` is built for you
- 📥 **Import from `~/.ssh/config`** — zero migration cost
- 🔑 Per-host **identity file, port, extra flags**
- 🚀 **Single Rust binary** — no runtime, no deps
- 🌐 Uses your system `ssh`, so agent forwarding / known_hosts / keys just work

### Install

```bash
cargo install ssh-menu
```

Or grab pre-built binaries from the [Releases](https://github.com/Aidan-996/ssh-menu/releases) page (Linux / macOS / Windows).

### Quick start

```bash
ssh-menu import            # merge ~/.ssh/config
ssh-menu                   # launch TUI
ssh-menu connect prod-db   # skip TUI
```

### Config (`~/.ssh-menu.toml`)

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
jump  = "prod-db"
extra = ["-o", "ServerAliveInterval=30"]
```

### Commands

| Command                  | What it does                                 |
| ------------------------ | -------------------------------------------- |
| `ssh-menu`               | Launch interactive TUI (default)             |
| `ssh-menu list`          | Print all hosts                              |
| `ssh-menu connect NAME`  | Connect directly, no TUI                     |
| `ssh-menu import`        | Merge entries from `~/.ssh/config`           |
| `ssh-menu import --from` | Import from a custom ssh config path         |
| `ssh-menu path`          | Print resolved config file path              |

### Key bindings

| Key              | Action                  |
| ---------------- | ----------------------- |
| `↑`/`↓` or `j/k` | Move selection          |
| `g` / `G`        | Jump top / bottom       |
| `Enter`          | Connect to selected     |
| `/`              | Search mode             |
| `a`              | Add host                |
| `e`              | Edit selected           |
| `d`              | Delete selected         |
| `q` / `Esc`      | Quit                    |

**Form mode**: `Tab` — next field, `Enter` or `Ctrl-S` — save, `Esc` — cancel.

### Environment variables

| Var                 | Purpose                                            |
| ------------------- | -------------------------------------------------- |
| `SSH_MENU_CONFIG`   | Override config file path                          |
| `SSH_MENU_SSH`      | Point to the `ssh` binary if not on `PATH`         |

### FAQ

**Q: Does it replace `ssh`?**
No — it builds argv (including `-i`, `-p`, `-J`, extras) and execs the system `ssh`. All existing agent / keys / known_hosts remain respected.

**Q: Windows?**
Works on Windows 10+ with OpenSSH Client installed. ssh-menu auto-probes common locations (`System32\OpenSSH\`, `Git\usr\bin\`).

---

## 📦 版本迭代 / Release history

查看完整变更日志：[CHANGELOG.md](CHANGELOG.md)｜Full changelog: [CHANGELOG.md](CHANGELOG.md)

### v0.1.1 — 2026-04-24

**🐛 修复 / Fixes**
- 修复导入 `~/.ssh/config` 时同名 Host 重复出现的问题，按 OpenSSH "later overrides" 语义保留最后一次出现的条目
- Fix duplicate entries when importing `~/.ssh/config` with repeated `Host` aliases (OpenSSH "later overrides" semantics).

**✨ 增强 / Enhancements**
- 健壮的 `ssh` 可执行文件查找：`$SSH_MENU_SSH` 环境变量 → PATH → Windows/Unix 常见路径的多级兜底
- Robust `ssh` executable discovery: `$SSH_MENU_SSH` → PATH → well-known Windows/Unix paths.
- 找不到 `ssh` 时给出清晰的安装指引
- Clear install guidance when `ssh` is missing.

**🏗️ 代码结构 / Code structure**
- 将扁平的单层 `src/*.rs` 重组为模块化目录：`config/`、`ssh/`、`tui/`
- Refactor flat `src/*.rs` into modular directories: `config/`, `ssh/`, `tui/`.
- TUI 拆分为 `app`（状态）/ `events`（按键）/ `view`（渲染）/ `form`（表单）/ `runtime`（事件循环）
- TUI split into `app` (state) / `events` (keys) / `view` (render) / `form` / `runtime` (loop).

**📚 文档 / Docs**
- 新增 `CHANGELOG.md`（Keep a Changelog 格式）
- Added `CHANGELOG.md` (Keep a Changelog format).
- README 改为双语（中文在上、英文在下）
- Bilingual README (Chinese first, English below).

### v0.1.0 — 2026-04-24

**🎉 首次发布 / Initial release**

- TUI 主机列表 + vim 风格快捷键
- 实时搜索（子串匹配 name/host/user/group/tags）
- TUI 内 CRUD（添加/编辑/删除 + 二次确认）
- TOML 配置 `~/.ssh-menu.toml`
- ProxyJump / 跳板机支持
- 从 `~/.ssh/config` 导入合并
- 子命令：`tui` / `list` / `connect` / `import` / `path`
- exec 系统 `ssh`，完全复用 agent / 密钥 / known_hosts
- GitHub Actions 跨平台 Release 工作流（Linux / macOS x64+ARM / Windows）

---

## 🏗️ 代码架构 / Project layout

```
ssh-menu/
├── Cargo.toml
├── README.md                 # 本文件 / this file
├── CHANGELOG.md              # 版本迭代 / release history
├── LICENSE                   # MIT
├── config.example.toml       # 配置示例 / config template
├── .github/workflows/
│   ├── ci.yml                # CI: build + test on Linux/macOS/Windows
│   └── release.yml           # tag v* → 交叉编译上传 Release
└── src/
    ├── main.rs               # CLI 入口、子命令分发 / CLI entry & subcommand dispatch
    ├── config/               # 配置 / configuration
    │   ├── mod.rs            #   re-exports
    │   ├── model.rs          #   Config / Host 结构 + 显示/搜索辅助 / structs + helpers
    │   └── store.rs          #   load / save / path 解析 / I/O
    ├── ssh/                  # SSH 相关 / SSH integration
    │   ├── mod.rs            #   re-exports
    │   ├── connect.rs        #   构造 argv + 查找 ssh + 启动连接 / argv + ssh discovery + spawn
    │   └── import.rs         #   解析 & 合并 ~/.ssh/config / parse & merge openssh config
    └── tui/                  # 终端 UI / terminal UI
        ├── mod.rs            #   re-exports
        ├── app.rs            #   应用状态机 / App state (Mode, filtered list, exit)
        ├── form.rs           #   添加/编辑表单状态 / add/edit form state
        ├── events.rs         #   键盘事件派发 / keyboard dispatcher
        ├── view.rs           #   渲染（列表/页头/页脚/表单蒙层）/ rendering
        └── runtime.rs        #   终端 setup/teardown + 事件循环 / terminal lifecycle + loop
```

**分层职责 / Layered responsibilities**

- `config::` — 纯数据与 I/O，不依赖任何 UI 或 ssh 逻辑
- `config::` — pure data + I/O, no UI or ssh knowledge
- `ssh::` — 仅依赖 `config::`，不涉及 UI
- `ssh::` — depends only on `config::`, no UI
- `tui::` — 组合 `config::` 和 `ssh::`，封装终端交互
- `tui::` — composes `config::` and `ssh::`, owns terminal interaction
- `main.rs` — 命令行解析 + 子命令到各模块的桥接
- `main.rs` — CLI parsing + bridging subcommands to modules

---

## 🗺️ Roadmap

- [ ] SSH 密钥选择器/生成器 | SSH key picker / generator
- [ ] 每主机 pre/post 钩子 | per-host pre/post hooks
- [ ] 导出回 OpenSSH config | export back to OpenSSH config
- [ ] 主题自定义 | theme customization
- [ ] 模糊匹配 | fuzzy matching (currently substring)
- [ ] 单元测试 | unit tests for parser / argv builder

---

## License

MIT © 2026 [Aidan-996](https://github.com/Aidan-996)
