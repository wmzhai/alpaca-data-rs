# memory/README.md

`memory/` 用来帮助新 session 快速定位项目事实、约束和后续落点。

## 热路径

- 想知道项目做什么：先读 `README.md`
- 想知道每个版本具体改了什么：先读 `CHANGELOG.md`
- 想知道必须遵守什么：先读 `memory/core/invariants.md`
- 想知道仓库当前有哪些目录和它们负责什么：读 `memory/core/system-map.md`
- 想知道默认工作顺序和验证基线：读 `memory/core/workflows.md`
- 想知道公开 API 设计和命名约束：读 `memory/api/README.md`

## 冷路径

- 遇到“该先看哪份文档”的问题：读 `memory/core/symptom-routing.md`
- 代码真正开始落地后，再按实际模块补充新的领域文档

## 当前状态说明

- 这是一个新项目，当前已经完成 `Phase 1: Shared Core`，并已进入 `Phase 2: Stocks` 的真实接口落地阶段，当前工作版本是 `v0.1.5`。
- 当前已经落下最小 Rust crate 骨架：`Cargo.toml`、`src/`、`tests/`、`.gitignore`。
- 已完成的真实代码落地包括：共享 `ClientBuilder` 运行时配置、认证配对校验、共享 `QueryWriter` / `Endpoint`、共享 `HttpClient`、异常路径 mock 测试、共享分页 helper、真实 API 的 `crypto.latest_quotes` smoke test，以及 `stocks` 历史 batch / single、latest、snapshot、metadata 端点与对应 live / mock 覆盖。
- 当前下一步默认进入 `Phase 2 / Task 6`，补齐 `stocks` 历史 batch `*_all` / `*_stream` 便利层，再进入 phase 收尾。
- `memory/` 现在既记录设计约束，也要记录已经落地的真实代码入口，后续随着 transport、分页和资源实现补齐继续同步更新。
