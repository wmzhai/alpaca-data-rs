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

- 这是一个新项目，当前仍以设计约束和路线文档为主，但已经开始落地共享基础层代码。
- 当前已经落下最小 Rust crate 骨架：`Cargo.toml`、`src/`、`tests/`、`.gitignore`。
- 已完成的最小真实代码落地包括：`ClientBuilder` 运行时配置、认证配对校验、`InvalidConfiguration` 错误类型、`tests/client_builder.rs`、共享 `QueryWriter` / `Endpoint`，以及共享 `HttpClient` 与异常路径 mock 测试。
- `memory/` 现在既记录设计约束，也要记录已经落地的真实代码入口，后续随着 transport、分页和资源实现补齐继续同步更新。
