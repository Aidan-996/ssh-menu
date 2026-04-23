# ssh-menu

> 交互式 TUI SSH 连接管理器，告别反复敲 `ssh -i ... -p ... user@host`。

[English](README.md) | **简体中文**

[![crates.io](https://img.shields.io/crates/v/ssh-menu.svg)](https://crates.io/crates/ssh-menu)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

```
┌─ SSH Menu ────────────────────────────────────────┐
│ > prod-db       root@db.example.com     [prod]    │
│   staging-web   root@web.staging.lan    [staging] │
│   cloud-vps     ubuntu@vps.example.net  [cloud]   │
│   bastion-jump  admin@10.0.0.1:2222     [infra]   │
└────────────────────────────────────────────────────┘
 Enter=连接  a=添加  e=编辑  d=删除  /=搜索  q=退出
```

## 特性

- 📋 **TUI 主机列表**，支持 vim 风格快捷键（`j/k`、`g/G`、方向键）
- 🔍 **实时搜索**：按名称、主机、用户、分组、标签过滤
- ➕ **内置增删改**：TUI 里直接加/改/删，无需手动编辑配置文件
- 🗂️ **分组 + 标签**，把堡垒机/生产/测试/云主机一眼分开
- 🔀 **ProxyJump / 跳板机**支持，自动拼 `-J` 参数
- 📥 **一键导入 `~/.ssh/config`**，零迁移成本
- 🔑 每主机独立的**密钥、端口、额外参数**
- 🚀 Rust **单二进制**，无运行时依赖
- 🌐 调用系统 `ssh`，你的 agent / known_hosts / 密钥完全不受影响

## 安装

### Cargo

```bash
cargo install ssh-menu
```

### 下载预编译二进制

去 [Releases](https://github.com/Aidan-996/ssh-menu/releases) 页面下载对应系统版本（Linux / macOS / Windows）。

### 从源码编译

```bash
git clone https://github.com/Aidan-996/ssh-menu
cd ssh-menu
cargo build --release
# 二进制在 target/release/ssh-menu
```

## 快速上手

```bash
# 首次使用 —— 导入现有的 SSH 配置
ssh-menu import

# 启动 TUI
ssh-menu

# 直接按名称连接，跳过 TUI
ssh-menu connect prod-db
```

## 配置文件

默认位置 `~/.ssh-menu.toml`（可用 `--config` 或环境变量 `SSH_MENU_CONFIG` 覆盖）。

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

[[hosts]]
name  = "cloud-vps"
host  = "vps.example.net"
user  = "ubuntu"
group = "cloud"
tags  = ["public"]
```

你可以直接编辑这个文件，也可以用 TUI（`a` 添加 / `e` 编辑 / `d` 删除），工具会自动写回。

## 命令

| 命令                     | 说明                                         |
| ------------------------ | -------------------------------------------- |
| `ssh-menu`               | 启动交互式 TUI（默认）                       |
| `ssh-menu list`          | 列出所有主机                                 |
| `ssh-menu connect NAME`  | 不开 TUI，直接连接                           |
| `ssh-menu import`        | 从 `~/.ssh/config` 合并导入                  |
| `ssh-menu import --from` | 从指定路径导入                               |
| `ssh-menu path`          | 打印当前配置文件路径                         |

## 快捷键

**普通模式**

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

**搜索模式**

- 输入任意字符即时过滤
- `Esc` — 清空过滤条件
- `↑`/`↓` — 在过滤结果中移动光标

**表单模式**（添加/编辑）

- `Tab` / `↑` / `↓` — 切换字段
- `Enter` 或 `Ctrl-S` — 保存
- `Esc` — 取消

## FAQ

**Q：它会替换 `ssh` 吗？**
不会。ssh-menu 只是帮你拼好 `ssh` 的参数（`-i`、`-p`、`-J` 以及额外选项），然后直接 exec 系统 `ssh`。你现有的 agent、密钥、known_hosts 设置都不受影响。

**Q：我还能用 `~/.ssh/config` 吗？**
可以。两种方式：(a) `ssh-menu import` 把条目拉进来；(b) 在 `jump` 字段里直接写 ssh_config 里的别名，ssh-menu 会原样传给 `-J`。

**Q：Windows 支持吗？**
支持。Windows 10+ 自带 OpenSSH（`ssh.exe` 在 `PATH` 中）即可。终端渲染用 `crossterm`，跨平台。

## Roadmap

- [ ] SSH 密钥选择器 / 生成器
- [ ] 每主机 pre/post 钩子（连接时本地执行命令、通知等）
- [ ] 导出回 OpenSSH config 格式
- [ ] 主题自定义
- [ ] 模糊匹配（当前是子串匹配）

## License

MIT © 2026 [Aidan-996](https://github.com/Aidan-996)
