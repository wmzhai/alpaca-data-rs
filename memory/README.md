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

- 这是一个新项目，当前已经完成 `Phase 1: Shared Core`、`Phase 2: Stocks` 与 `Phase 3: Options`，`Phase 4: Crypto` 正在推进，当前工作版本是 `v0.3.2`。
- 当前已经落下最小 Rust crate 骨架：`Cargo.toml`、`src/`、`tests/`、`.gitignore`。
- 已完成的真实代码落地包括：共享 `ClientBuilder` 运行时配置、认证配对校验、共享 `QueryWriter` / `Endpoint`、共享 `HttpClient`、异常路径 mock 测试、共享分页 helper、真实 API 的 `crypto` historical `bars` / `quotes` / `trades` 与 `bars_all` / `bars_stream`、`quotes_all` / `quotes_stream`、`trades_all` / `trades_stream`，以及 latest `latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks`；完整的 `stocks` 历史 batch / single、latest、snapshot、metadata 端点、batch + single 的 `*_all` / `*_stream` 便利层；`options` 现已完整打通 historical `bars` / `trades`、latest `latest_quotes` / `latest_trades`、snapshot family `snapshots` / `chain`、metadata `exchange_codes`，以及全部 `*_all` / `*_stream` 便利层。
- 当前下一步继续完成 `Phase 4: Crypto` 的 `snapshots`、故障注入测试与 benchmark，继续沿用 `stocks` / `options` 已经定型的模块分层、live test、mock fault 与 benchmark 收尾结构。
- `memory/` 现在既记录设计约束，也要记录已经落地的真实代码入口，后续随着 transport、分页和资源实现补齐继续同步更新。
