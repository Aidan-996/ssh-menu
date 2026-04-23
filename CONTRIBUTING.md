# 贡献指南

感谢考虑为 `ssh-menu` 做贡献！这是一个轻量的开源项目，欢迎 issue、PR、文档修正、翻译。

---

## 🌿 分支模型

```
feature/xxx  ──PR──▶  dev  ──PR──▶  main  ──tag──▶  Release
  (开发)              (集成/测试)     (稳定)
```

- **`main`** — 稳定分支，只从 `dev` 合入。所有 tag（`v*`）都打在 `main` 上，触发 GitHub Actions 自动发布 Release
- **`dev`** — 集成分支，日常合并点。所有外部 PR、新功能都提到这里
- **`feature/xxx`** — 每个新功能或 bugfix 用独立分支，合回 `dev`

**原则**：
- ❌ 不要直接 push 到 `main`
- ❌ 不要直接 push 到 `dev`（即使你是协作者，也请走 PR）
- ✅ 从 `dev` 拉新分支开发，开发完提 PR 回 `dev`

---

## 🚀 快速开始（Fork 流程）

```bash
# 1. Fork 本仓库到你自己的 GitHub 账号
# 2. Clone 到本地
git clone https://github.com/<你的用户名>/ssh-menu
cd ssh-menu

# 3. 添加上游
git remote add upstream https://github.com/Aidan-996/ssh-menu

# 4. 从 dev 拉新分支
git fetch upstream
git checkout -b feature/my-thing upstream/dev

# 5. 改代码、跑测试
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check

# 6. 提交 + 推送
git add .
git commit -m "feat: add my thing"
git push origin feature/my-thing

# 7. 去 GitHub 开 PR，base 选 Aidan-996/ssh-menu:dev
```

---

## 🧑‍💻 代码规范

### Rust 风格
- 通过 `cargo fmt` 格式化
- `cargo clippy -- -D warnings` 零警告
- 公开 API 加文档注释 `///`
- 错误用 `anyhow::Result`，内部错误可用 `?` 传播

### Commit Message 约定
使用 [Conventional Commits](https://www.conventionalcommits.org/) 风格：

```
<类型>(<可选范围>): <简短描述>

<可选正文>
```

常用类型：
- `feat` — 新功能
- `fix` — 修 bug
- `docs` — 只改文档
- `refactor` — 重构（无功能变化）
- `perf` — 性能优化
- `test` — 加测试
- `chore` — 构建 / 依赖 / 杂项

示例：
```
feat(tui): add column sort by last-used time
fix(import): handle empty HostName line
docs: clarify jump field in README
```

---

## ✅ PR 检查清单

提交 PR 前请确认：

- [ ] 从最新的 `dev` 分支派生
- [ ] `cargo build --release` 通过
- [ ] `cargo test` 通过
- [ ] `cargo clippy -- -D warnings` 无警告
- [ ] `cargo fmt --check` 通过
- [ ] 如果影响 UI / 行为，README 或截图已更新
- [ ] Commit message 符合 Conventional Commits
- [ ] PR 标题简洁，正文说明 **动机**、**改动点**、**如何测试**

---

## 🐛 报 Bug

在 Issues 里选 **Bug report**，包含：
- 操作系统 + 版本（`uname -a` / Windows 版本号）
- `ssh-menu --version`
- 复现步骤（最小化）
- 期望行为 vs 实际行为
- 必要时贴终端输出 / 截图

---

## 💡 提 Feature

先在 Issues 里开个讨论，确认方向后再动手写代码，避免白干。尤其是涉及：
- TUI 行为 / 快捷键调整
- 配置文件 schema 变更
- 新子命令
- 跨平台差异

---

## 🏗️ 本地开发

```bash
# 运行未安装版本
cargo run -- tui
cargo run -- list
cargo run -- connect <name>

# 用独立配置，不污染你自己的 ~/.ssh-menu.toml
SSH_MENU_CONFIG=./test.toml cargo run
# 或
cargo run -- --config ./test.toml
```

### 调试 TUI
- 用独立终端窗口测，不要嵌在 IDE 终端里
- TUI 崩溃时终端可能卡死，执行 `reset` 或 `stty sane` 恢复

---

## 📦 发版流程（维护者）

1. 所有改动在 `dev` 通过 CI
2. `dev` 合回 `main`：`git checkout main && git merge --no-ff dev`
3. 更新 `Cargo.toml` 的 `version`、`CHANGELOG.md`
4. 提 commit：`chore: release v0.x.y`
5. 打 tag：`git tag v0.x.y && git push origin main --tags`
6. GitHub Actions 自动交叉编译 4 目标（linux-gnu / darwin x64+arm64 / windows-msvc）上传 Release

---

## 📄 License

本项目 MIT 协议。贡献的代码默认以相同协议发布。

---

## 💬 联系

- Issues: https://github.com/Aidan-996/ssh-menu/issues
- Discussions: https://github.com/Aidan-996/ssh-menu/discussions（如已开启）

感谢你的贡献 🙏
