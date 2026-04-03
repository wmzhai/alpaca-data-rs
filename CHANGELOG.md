# CHANGELOG.md

本文件记录每个版本提交对应的主要变化。

规则：

- 每次新版本提交都必须更新本文件
- 不只记录结构变化，也记录对外接口、文档、测试、工程配置和内部实现上的重要变化
- 版本号使用三段格式：`MAJOR.MINOR.PATCH`

## v0.0.1

### Added

- 初始化 `alpaca-data` Rust library crate，包名为 `alpaca-data`
- 建立根入口 `alpaca_data::Client`
- 建立 `stocks`、`options`、`crypto`、`news`、`corporate_actions` 五个资源模块骨架
- 建立 `transport` 和 `common` 最小公共层
- 建立 `tests/public_api.rs`，验证公开 API 形状
- 建立 `AGENTS.md` 和 `memory/` 项目记忆体系
- 建立 `docs/superpowers/plans/2026-04-03-crate-bootstrap.md`

### Changed

- 将 crate 版本设为 `0.0.1`
- 明确 crates.io 包名使用 `alpaca-data`，代码导入名使用 `alpaca_data`
- 明确 commit message 采用英文 Conventional Commits 风格

### Docs

- 写入项目最终设计方案到 `README.md`
- 写入版本提交必须同步更新 `CHANGELOG.md` 的仓库规则

### Notes

- 当前代码仍为最小公开骨架，真实 Alpaca HTTP API 逻辑尚未开始实现
