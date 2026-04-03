# CHANGELOG.md

本文件记录每个版本提交对应的主要变化。

规则：

- 每次新版本提交都必须更新本文件
- 不只记录结构变化，也记录对外接口、文档、测试、工程配置和内部实现上的重要变化
- 版本号使用三段格式：`MAJOR.MINOR.PATCH`

## v0.0.6

### Added

- 新建 `tests/live_crypto_latest_quotes_smoke.rs`，用真实 Alpaca API 验证 `crypto.latest_quotes`
- 为 `tests/public_api.rs` 增加 `crypto.latest_quotes` 的公开 API 覆盖

### Changed

- `Phase 1 / Task 5` 的 canary endpoint 现在已通过真实 API smoke test 验证
- live test 现在支持通过 `ALPACA_LIVE_TESTS=1` 开关启用，并支持 `APCA_API_DATA_URL` 覆盖 base URL

### Docs

- 更新 `AGENTS.md` 当前项目状态
- 更新 `memory/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.5

### Added

- 为分页框架新增 `PaginatedRequest`、`PaginatedResponse`、`collect_all` 和 `stream_pages`
- 为分页 helper 新增单元测试，覆盖聚合和按页流

### Changed

- `common::response::ResponseStream` 现在升级为 boxed async stream 类型
- `empty_stream()` 现在返回 boxed 空 stream，以兼容后续共享分页流接口
- Phase 1 计划和 roadmap 已同步到 Task 4 完成后的当前状态

### Docs

- 更新 `AGENTS.md` 当前项目状态
- 更新 `memory/README.md` 与 `memory/core/system-map.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.4

### Added

- 接入 `reqwest`、`serde`、`serde_json`、`tokio` 与 `wiremock` 作为共享 transport 和异常测试基础依赖
- 新建 `tests/mock_transport_errors.rs`，覆盖 429 retry-after 和损坏 JSON 的异常映射
- 为 `Auth` 增加 request header 注入能力
- 为 `HttpClient` 增加 async JSON GET 通路

### Changed

- `Client` 现在持有共享 `HttpClient`，并根据 builder runtime config 初始化 timeout、retry 和 rate limiting
- `RetryPolicy` 和 `RateLimiter` 现在具备最小实际行为，而不再只是占位结构
- `Error::RateLimited` 现在同时保留 `retry_after` 与 `body`
- `crypto.latest_quotes` 现在已经接到共享 transport，可用于 mock 异常测试链路

### Docs

- 更新 `AGENTS.md` 当前项目状态
- 更新 `memory/README.md` 与 `memory/core/system-map.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.3

### Added

- 新建 `src/common/query.rs`，提供 `QueryWriter` 以统一构造 query 参数
- 新建 `src/transport/endpoint.rs`，提供 `Endpoint` 以统一官方 Market Data 路径路由
- 为 query 序列化和 endpoint 路由新增对应的单元测试

### Changed

- `src/common/mod.rs` 现在导出 `query` 模块
- `src/transport/mod.rs` 现在导出 `endpoint` 模块
- Phase 1 计划和 roadmap 已同步到 Task 2 完成后的当前状态

### Docs

- 更新 `memory/core/system-map.md` 记录新的共享基础层文件
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.2

### Added

- 建立 `tests/client_builder.rs`，覆盖 public crypto client、partial credentials 校验和显式 runtime config 构建
- 为 `ClientBuilder` 增加 `timeout`、`max_retries`、`max_in_flight` 三个最小共享运行时配置入口
- 为 `Error` 增加 `InvalidConfiguration`，用于配置层错误建模

### Changed

- `ClientBuilder::build()` 现在默认使用 `https://data.alpaca.markets`
- `Auth` 现在强制 `api_key` 和 `secret_key` 成对出现，不允许半配置状态通过构建
- 项目文档与 memory 已更新到“共享基础层 Task 1 已落地”的当前事实
- 提交流程规则已明确：提交前必须全面检查代码、测试和文档是否彼此对齐，发现问题先修正再提交

### Docs

- 更新 `AGENTS.md` 的任务提交、分支使用和提交流程校验规则
- 更新 `memory/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`、`memory/core/invariants.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 和 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md` 以反映当前进度

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
