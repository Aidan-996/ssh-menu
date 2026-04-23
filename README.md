# ssh-menu

> 交互式 TUI SSH 连接管理器，告别反复敲 `ssh -i ... -p ... user@host`

[![crates.io](https://img.shields.io/crates/v/ssh-menu.svg)](https://crates.io/crates/ssh-menu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI](https://github.com/Aidan-996/ssh-menu/actions/workflows/ci.yml/badge.svg)](https://github.com/Aidan-996/ssh-menu/actions/workflows/ci.yml)

Rust 单二进制，跨平台，零运行时依赖。

```
┌─ SSH Menu v0.2.0 ──────────── 4 hosts • sort:name • details:on ──┐
│  🔌 ssh-menu                                                      │
└───────────────────────────────────────────────────────────────────┘
┌─ Hosts (4) ─────────────────────┬─ Details ────────────────────┐
│ ▶  1 prod       prod-db         │ name       prod-db            │
│    2 prod       web-server      │ host       db.example.com     │
│    3 cloud      cloud-vps  ↪pd  │ user       root               │
│    4 internal   win-gw     [rdp]│ port       22                 │
│                                 │ group      prod               │
│                                 │ tags       mysql, linux       │
│                                 │                               │
│                                 │ ── usage ──                   │
│                                 │ connects   12                 │
│                                 │ last       3h ago             │
│                                 │                               │
│                                 │ ── ssh command ──             │
│                                 │ ssh root@db.example.com       │
└─────────────────────────────────┴───────────────────────────────┘
│ Press ? for help • Enter=connect • a=add • /=search • q=quit    │
```

---

## ✨ 特性

- 📋 **TUI 主机列表**，vim 风格快捷键（`j/k`、`g/G`、方向键、PgUp/PgDn）
- 🔍 **实时搜索**，按名称/主机/用户/分组/标签过滤
- 🎨 **彩色列表 + 详情面板**，一眼看清所有字段
- 📊 **使用统计**，自动记录连接次数与最近使用时间（`3h ago` / `2d ago`）
- 🔀 **4 种排序**：按名称 / 分组 / 最近使用 / 最多使用，`s` 键切换
- ⚡ **快速跳转**：按 `1-9` 跳到第 N 个、按字母跳到首字母匹配项
- ➕ **内置增删改 + 字段提示**，表单实时显示每个字段的说明
- 🔁 **ProxyJump / 跳板机**支持，自动拼 `-J`
- 📥 **一键导入 `~/.ssh/config`**，零迁移成本，导入自动去重
- 🔑 每主机独立的密钥、端口、额外参数、备注
- 🚀 Rust 单二进制，无运行时依赖
- 🌐 调用系统 `ssh`，多路径兜底查找；agent / known_hosts / 密钥完全复用
- ❓ **内置 `?` 帮助蒙层**，随时查快捷键不用翻文档
- 👀 查看等效的 `ssh` 命令（在详情面板，或按 `y`）

---

## 📦 安装

### Cargo

```bash
cargo install ssh-menu
```

### 预编译二进制

到 [Releases](https://github.com/Aidan-996/ssh-menu/releases) 页面下载对应系统的版本（Linux / macOS / Windows）。

### 从源码编译

```bash
git clone https://github.com/Aidan-996/ssh-menu
cd ssh-menu
cargo build --release
# 二进制在 target/release/ssh-menu
```

---

## 🚀 快速上手

```bash
# 首次：导入现有 SSH 配置
ssh-menu import

# 打开 TUI
ssh-menu

# 直接按名称连接（跳过 TUI）
ssh-menu connect prod-db
```

---

## ⌨️ 快捷键

在 TUI 里随时按 **`?`** 查看完整快捷键说明。

### 普通模式

| 按键                  | 动作                              |
| --------------------- | --------------------------------- |
| `↑`/`↓` 或 `j`/`k`    | 移动光标                          |
| `PgUp` / `PgDn`       | 翻 10 行                          |
| `g` / `G` 或 `Home`/`End` | 跳到首项 / 末项               |
| `1`–`9`               | 跳到当前第 N 个                   |
| 任意字母              | 跳到下一个**首字母匹配**的主机    |
| `Enter`               | 连接当前选中                      |
| `/`                   | 进入搜索                          |
| `a`                   | 添加新主机                        |
| `e`                   | 编辑当前选中                      |
| `D`（Shift+d）        | 删除（二次确认 `y/N`）            |
| `y`                   | 状态栏显示等效 `ssh` 命令         |
| `s`                   | 切换排序（名称 → 分组 → 最近 → 最多）|
| `i`                   | 显示/隐藏详情面板                 |
| `r`                   | 刷新过滤                          |
| `?`                   | 打开帮助蒙层                      |
| `q` / `Esc`           | 退出                              |
| `Ctrl-C`              | 强制退出                          |

### 搜索模式（`/` 进入）

- 任意字符即时过滤
- `Backspace` 删字符，`Ctrl-U` 清空
- `↑`/`↓` 在过滤结果中移动
- `Enter`：若只剩 1 条结果直接连接
- `Esc` 清空并回到普通模式

### 表单模式（添加 / 编辑）

- `Tab` / `↑` / `↓`：切换字段
- `Ctrl-U`：清空当前字段
- `Enter` 或 `Ctrl-S`：保存
- `Esc`：取消
- 当前字段下方会显示**字段说明**（哪些必填、格式要求等）

---

## 📝 配置文件

默认路径 `~/.ssh-menu.toml`（可用 `--config` 或环境变量 `SSH_MENU_CONFIG` 覆盖）。

```toml
[[hosts]]
name  = "prod-db"                    # 必填：别名（中文也行）
host  = "db.example.com"             # 必填：IP 或域名
user  = "root"                       # 默认 root
port  = 22                           # 默认 22
key   = "~/.ssh/id_rsa"              # 可选：私钥路径（支持 ~/）
group = "prod"                       # 可选：分组
tags  = ["mysql", "linux"]           # 可选：标签
note  = "主库，小心改写操作"         # 可选：备注

[[hosts]]
name  = "win-rdp-gateway"
host  = "10.0.0.5"
user  = "admin"
port  = 3389
group = "internal"
jump  = "prod-db"                    # 通过另一台已命名主机跳板
extra = ["-o", "ServerAliveInterval=30"]  # 额外 ssh 参数
```

使用统计字段由工具自动维护，无需手填：

```toml
last_used = "2026-04-24T15:30:12+08:00"
use_count = 12
```

---

## 📟 命令行

| 命令                     | 说明                          |
| ------------------------ | ----------------------------- |
| `ssh-menu`               | 启动交互式 TUI（默认）        |
| `ssh-menu list`          | 列出所有主机                  |
| `ssh-menu connect NAME`  | 不开 TUI，直接连接            |
| `ssh-menu import`        | 从 `~/.ssh/config` 合并导入   |
| `ssh-menu import --from` | 从指定路径导入                |
| `ssh-menu path`          | 打印配置文件路径              |

---

## 🌱 环境变量

| 变量                | 作用                                           |
| ------------------- | ---------------------------------------------- |
| `SSH_MENU_CONFIG`   | 覆盖默认配置文件路径                           |
| `SSH_MENU_SSH`      | 指定 `ssh` 可执行文件路径（PATH 找不到时用）   |

---

## ❓ FAQ

**Q：它会替换 `ssh` 吗？**
不会。ssh-menu 只是帮你拼好 `ssh` 的参数（`-i`、`-p`、`-J`、`extra`），然后直接 exec 系统 `ssh`。你现有的 agent、密钥、known_hosts 全部不受影响。

**Q：Windows 支持吗？**
支持。需要系统装有 OpenSSH Client（`ssh.exe`）。ssh-menu 会自动按顺序查找：`$SSH_MENU_SSH` → `PATH` → `C:\Windows\System32\OpenSSH\` → `C:\Program Files\Git\usr\bin\` 等常见位置。

**Q：我还能用 `~/.ssh/config` 吗？**
可以。两种方式：(a) `ssh-menu import` 把条目拉进来；(b) 在 `jump` 字段里直接写 ssh_config 里的别名，ssh-menu 会原样传给 `-J`。

**Q：使用统计会影响原有行为吗？**
不会。`last_used` 和 `use_count` 只在连接成功时自动写入配置，不会触发任何网络行为。如果不想要，删除字段即可。

---

## 📦 版本迭代

详细变更日志见 [CHANGELOG.md](CHANGELOG.md)。

### v0.2.0 — 2026-04-24  🎨 TUI 大升级

**🎨 界面美化**
- 彩色列表：序号灰色、分组洋红、名称青色加粗、连接串绿色、跳板黄色、标签蓝色
- 全新页头：显示主机总数、当前排序、详情面板开关
- 视觉标记：`▶` 选中指示、`↪` 跳板、连接状态符号
- 支持 `🔌` emoji 标题与状态栏 `✓/✗` 反馈

**✨ 新增功能**
- 📊 **使用统计**：自动记录每台主机的连接次数（`use_count`）和最近使用时间（`last_used`）
- 🔀 **4 种排序**：按名称 / 分组 / 最近使用 / 最多使用，`s` 键循环切换
- 📱 **详情面板**：右侧显示选中主机所有字段 + 等效 `ssh` 命令，`i` 键开关
- ❓ **帮助蒙层**：按 `?` 弹出完整快捷键说明
- ⚡ **快速跳转**：`1`–`9` 跳到第 N 个、字母键跳到首字母匹配项
- 📖 **表单字段提示**：添加/编辑时当前字段下方显示格式说明
- 👀 **查看 ssh 命令**：按 `y` 在状态栏显示当前选中的等效命令
- `PgUp/PgDn` 翻 10 行
- `Ctrl-U` 在搜索和表单中清空当前输入
- 搜索模式按 `Enter`：若只剩 1 条结果直接连接
- 删除键改为 `D`（Shift+d），避免误触

**🛡️ 增强**
- 空列表友好提示（"按 a 添加" / "运行 ssh-menu import"）
- 无匹配时的引导文案
- 删除使用 ⚠ 图标强调不可逆
- 编辑主机时保留已有的使用统计

### v0.1.1 — 2026-04-24

**🐛 修复**
- 修复导入 `~/.ssh/config` 时同名 Host 重复出现的问题（按 OpenSSH "later overrides" 语义保留最后一次）

**✨ 增强**
- 健壮的 `ssh` 可执行文件查找：`$SSH_MENU_SSH` → PATH → Windows/Unix 常见路径
- 找不到 `ssh` 时给出清晰的安装指引

**🏗️ 代码结构**
- 扁平 `src/*.rs` 重组为模块化目录：`config/`、`ssh/`、`tui/`
- TUI 拆分为 `app` / `events` / `view` / `form` / `runtime`

### v0.1.0 — 2026-04-24  🎉 首次发布

- TUI 主机列表 + vim 风格快捷键
- 实时搜索（子串匹配）
- TUI 内 CRUD（添加/编辑/删除 + 二次确认）
- TOML 配置 `~/.ssh-menu.toml`
- ProxyJump 支持
- 从 `~/.ssh/config` 导入
- 子命令：`tui` / `list` / `connect` / `import` / `path`
- exec 系统 `ssh`
- GitHub Actions 跨平台 Release 流水线

---

## 🏗️ 代码架构

```
ssh-menu/
├── Cargo.toml
├── README.md                 # 本文件
├── CHANGELOG.md              # 详细变更日志
├── LICENSE                   # MIT
├── config.example.toml       # 配置示例
├── .github/workflows/
│   ├── ci.yml                # CI：三平台 build + test
│   └── release.yml           # tag v* → 交叉编译 4 平台上传 Release
└── src/
    ├── main.rs               # CLI 入口、子命令分发、调用 ssh::connect
    ├── config/               # 配置层（纯数据 + I/O，零 UI/ssh 依赖）
    │   ├── mod.rs
    │   ├── model.rs          #   Config / Host 结构 + 显示 / 搜索辅助
    │   └── store.rs          #   加载 / 保存 / 路径解析
    ├── ssh/                  # SSH 层（只依赖 config）
    │   ├── mod.rs
    │   ├── connect.rs        #   拼 argv + 查找 ssh + 启动连接 + 时间工具
    │   └── import.rs         #   ~/.ssh/config 解析 + 合并去重
    └── tui/                  # 终端 UI 层（组合 config + ssh）
        ├── mod.rs
        ├── app.rs            #   应用状态机（Mode / 过滤列表 / 排序 / 详情开关）
        ├── form.rs           #   添加 / 编辑表单状态
        ├── events.rs         #   键盘事件派发（五种模式）
        ├── view.rs           #   渲染（头部 / 列表 / 详情 / 页脚 / 表单 / 帮助蒙层）
        └── runtime.rs        #   终端生命周期 + 事件循环
```

**分层职责**

- `config::` — 纯数据与 I/O，不依赖任何 UI 或 ssh 逻辑
- `ssh::` — 仅依赖 `config::`，不涉及 UI
- `tui::` — 组合 `config::` 和 `ssh::`，封装终端交互
- `main.rs` — CLI 解析 + 子命令到各模块的桥接

---

## 🗺️ Roadmap

- [ ] SSH 密钥选择器 / 生成器
- [ ] 每主机 pre/post 钩子（连接时本地命令、通知等）
- [ ] 导出回 OpenSSH config 格式
- [ ] 主题自定义（多种配色方案）
- [ ] 模糊匹配（当前为子串匹配）
- [ ] 批量操作（多选 + 批量删除 / 批量改分组）
- [ ] sshfs / scp 快捷入口
- [ ] 单元测试

---

## 📄 License

MIT © 2026 [Aidan-996](https://github.com/Aidan-996)
