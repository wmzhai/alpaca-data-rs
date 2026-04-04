# Full Project Roadmap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `alpaca-data` 从当前 `v0.1.0` 的 shared-core 基线起点，逐步推进到可发布到 crates.io 的高性能 Alpaca Market Data API Rust 客户端。

**Architecture:** 开发主线按共享基础层和资源域拆成独立 phase，先做 transport、错误模型、分页、真实 API 测试基线，再按 `stocks -> options -> crypto -> news/corporate_actions -> release hardening` 推进。benchmark、真实 API 测试和文档/CHANGELOG 作为横向轨道持续贯穿所有 phase，而不是只在最后补。

**Tech Stack:** Rust 2024 edition, Cargo library crate, reqwest transport, real Alpaca Market Data HTTP API integration tests, limited mock fault injection for exceptional paths only

---

## Phase Summary

- **Phase 0: Bootstrap**，已完成
- **Phase 1: Shared Core**，已完成（`v0.1.0`）
- **Phase 2: Stocks**，已完成（`v0.2.0`）
- **Phase 3: Options**，进行中（`v0.2.4`，Task 3 已完成）
- **Phase 4: Crypto**
- **Phase 5: News + Corporate Actions**
- **Phase 6: Release Hardening**

横向持续轨道：

- 真实 API 成功路径测试
- 异常路径 mock 故障注入测试
- benchmark 基线与回归对比
- README / CHANGELOG / memory 文档同步

## Phase 0: Bootstrap

**Status:** Done in `v0.0.1`

**Delivered:**

- `Cargo.toml` 与最小 crate 骨架
- `README.md` 最终设计方案
- `AGENTS.md`
- `memory/`
- `CHANGELOG.md`
- `alpaca_data::Client` 和五个资源模块壳
- `tests/public_api.rs` 公开 API 形状测试

**Exit Criteria:**

- `cargo test` 通过
- `cargo fmt --check` 通过
- 首次版本提交完成
- 代码已推到远端仓库

## Phase 1: Shared Core

**Status:** Done in `v0.1.0`

**Goal:** 把所有资源域都会依赖的基础能力做扎实，避免后续重复返工。

**Delivered:**

- `ClientBuilder` 已具备 `base_url`、`timeout`、`max_retries`、`max_in_flight` 最小运行时配置
- `Auth::new(...)` 已强制 `api_key` / `secret_key` 成对校验
- 顶层 `Error` 已具备 `InvalidConfiguration`、`RateLimited`、`HttpStatus`、`Deserialize` 等共享错误变体
- `tests/client_builder.rs` 已覆盖 builder/runtime config 与认证校验
- `src/common/query.rs` 已提供最小 query 参数构造能力
- `src/transport/endpoint.rs` 已提供 crypto latest quotes 的官方路径路由
- `src/transport/http.rs`、`src/transport/retry.rs`、`src/transport/rate_limit.rs` 已具备最小实际行为
- `tests/mock_transport_errors.rs` 已覆盖共享 transport 的异常路径
- `src/transport/pagination.rs` 已提供最小分页聚合与按页 stream helper
- `tests/live_crypto_latest_quotes_smoke.rs` 已验证 `crypto.latest_quotes` 的真实 API happy path
- `benches/shared_core.rs` 已建立本地 benchmark baseline

**Primary Scope:**

- `ClientBuilder` 的真实配置项
- auth 模型与需要认证/无需认证的判定
- endpoint version routing
- `reqwest` transport
- typed error
- query 参数序列化
- enum 到官方字符串映射
- 分页聚合框架
- `*_all` / `*_stream` 通用支撑
- 真实 API 测试基线
- 异常路径 mock 故障注入测试基线

**Files Likely Touched:**

- Modify: `Cargo.toml`
- Modify: `src/client.rs`
- Modify: `src/error.rs`
- Modify: `src/auth.rs`
- Modify: `src/common/enums.rs`
- Modify: `src/common/response.rs`
- Modify: `src/transport/http.rs`
- Modify: `src/transport/retry.rs`
- Modify: `src/transport/pagination.rs`
- Modify: `src/transport/rate_limit.rs`
- Create: `tests/live/` 下的最小真实 API 测试文件
- Create: `tests/mock/` 下的异常路径测试文件

**Deliverables:**

- 可复用的 HTTP 请求执行通路
- 统一的错误模型
- 统一的分页与 stream 支撑
- 至少一个最小真实 endpoint 能打通

**Exit Criteria:**

- 可以对真实 Alpaca API 成功发起最小请求
- 正常成功路径测试以真实 API 为主
- mock 只用于 429、5xx、timeout 等异常场景
- benchmark 最小基线建立完成

## Phase 2: Stocks

**Status:** Done in `v0.2.0`

**Goal:** 把 `stocks` 做成第一个完整模板模块，作为后续资源域的实现标准。

**Primary Scope:**

- `bars`
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `condition_codes`
- `exchange_codes`
- 对应的 `*_all` / `*_stream`

**Files Likely Touched:**

- Modify: `src/stocks/client.rs`
- Modify: `src/stocks/request.rs`
- Modify: `src/stocks/response.rs`
- Modify: `src/stocks/model.rs`
- Modify: `src/stocks/enums.rs`
- Create: `tests/live/stocks_*.rs`
- Create: `tests/mock/stocks_*.rs`

**Deliverables:**

- 第一个真实可用资源模块
- 多 symbol 分页行为验证
- latest / snapshot / metadata 端点形状定型
- 历史 batch + single `*_all` / `*_stream` 便利层完成
- `benches/stocks.rs` 本地 benchmark baseline 建立完成

**Exit Criteria:**

- `stocks` 所有已设计方法具备真实 HTTP 行为
- 对外请求/响应字段仍严格使用官方原词
- `bars_all` / `bars_stream` 行为已通过真实 API 验证

## Phase 3: Options

**Status:** In progress in `v0.2.4` (Task 3 complete)

**Delivered so far:**

- `options.bars` 与 `options.trades` 已具备真实 HTTP 行为
- `options.bars_all` / `options.bars_stream` 与 `options.trades_all` / `options.trades_stream` 已接通共享分页 helper
- `src/options/request.rs`、`src/options/response.rs`、`src/options/model.rs` 已具备 historical batch 的 typed query / wrapper / model
- `tests/live_options_historical.rs` 已用真实 Alpaca API 验证 `options` historical happy path
- `docs/superpowers/specs/2026-04-03-phase-3-options-design.md` 与 `docs/superpowers/plans/2026-04-03-phase-3-options.md` 已建立 phase 设计与执行文档
- `options.latest_quotes`、`options.latest_trades` 与 `options.exchange_codes` 已具备真实 HTTP 行为
- `tests/live_options_latest_metadata.rs` 已用真实 Alpaca API 验证 `options` latest + metadata happy path
- `options.snapshots` 与 `options.chain` 已具备真实 HTTP 行为，并已接通 `snapshots_all` / `snapshots_stream` / `chain_all` / `chain_stream`
- `options::Snapshot` 现已包含官方 `greeks` 与 `impliedVolatility` 字段，`SnapshotsResponse` / `ChainResponse` 现已具备 serde 与分页 merge 行为
- `tests/live_options_snapshots_chain.rs` 与 `tests/mock_options_errors.rs` 已分别覆盖 `options` snapshot/chain happy path 和异常分页/损坏 JSON 故障场景

**Goal:** 完成最复杂的资源域，锁定 option chain 和 snapshots 的正式实现方式。

**Primary Scope:**

- `bars`
- `trades`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `chain`
- `exchange_codes`
- 对应的 `*_all` / `*_stream`

**Files Likely Touched:**

- Modify: `src/options/client.rs`
- Modify: `src/options/request.rs`
- Modify: `src/options/response.rs`
- Modify: `src/options/model.rs`
- Modify: `src/options/enums.rs`
- Create: `tests/live/options_*.rs`
- Create: `tests/mock/options_*.rs`

**Deliverables:**

- `ChainRequest` / `ChainResponse` 最终定型
- option 分页与过滤参数实现完成
- 不引入额外 analytics 逻辑，只忠实封装官方 HTTP API

**Exit Criteria:**

- `options::chain` 与 `options::snapshots` 的分页行为在真实 API 下验证通过
- `r#type` 到 `type` 的最小 Rust 适配验证通过
- 不引入官方 HTTP API 之外的自定义字段

## Phase 4: Crypto

**Goal:** 完成 crypto 资源域，并处理其认证和路径差异。

**Primary Scope:**

- `bars`
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `latest_orderbooks`
- `snapshots`
- `loc` 建模与路由
- 无凭证可用行为

**Files Likely Touched:**

- Modify: `src/crypto/client.rs`
- Modify: `src/crypto/request.rs`
- Modify: `src/crypto/response.rs`
- Modify: `src/crypto/model.rs`
- Modify: `src/crypto/enums.rs`
- Create: `tests/live/crypto_*.rs`
- Create: `tests/mock/crypto_*.rs`

**Deliverables:**

- 无凭证 crypto 请求通路
- `loc` 参数序列化与 endpoint 路由完成
- `latest_orderbooks` 真实行为完成

**Exit Criteria:**

- `crypto()` 在无凭证配置下可运行成功路径测试
- `loc` 的默认值和显式值都完成验证
- crypto 真实 API 测试覆盖核心 happy path

## Phase 5: News + Corporate Actions

**Goal:** 完成两个 list/filter/pagination 型资源域。

**Primary Scope:**

- `news().list`
- `news().list_all`
- `news().list_stream`
- `corporate_actions().list`
- `corporate_actions().list_all`
- `corporate_actions().list_stream`

**Files Likely Touched:**

- Modify: `src/news/client.rs`
- Modify: `src/news/request.rs`
- Modify: `src/news/response.rs`
- Modify: `src/news/model.rs`
- Modify: `src/corporate_actions/client.rs`
- Modify: `src/corporate_actions/request.rs`
- Modify: `src/corporate_actions/response.rs`
- Modify: `src/corporate_actions/model.rs`
- Create: `tests/live/news_*.rs`
- Create: `tests/live/corporate_actions_*.rs`
- Create: `tests/mock/news_*.rs`
- Create: `tests/mock/corporate_actions_*.rs`

**Deliverables:**

- 两个资源域的官方字段映射完成
- list / list_all / list_stream 的统一行为完成

**Exit Criteria:**

- `news` 和 `corporate_actions` 成功路径真实 API 测试通过
- 分页聚合和按页 stream 行为与共享框架一致

## Phase 6: Release Hardening

**Goal:** 把项目从“功能可用”推进到“可发布到 crates.io”。

**Primary Scope:**

- benchmark 收口
- README 示例校对
- rustdoc 示例补齐
- package metadata 完整化
- 发布前检查脚本或文档
- `cargo package`
- `cargo publish --dry-run`
- CHANGELOG 与版本发布流程定型

**Files Likely Touched:**

- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `memory/` 中相关文档
- Create: `benches/` 下 benchmark 文件
- Create: 需要的发布辅助文档或脚本

**Deliverables:**

- crates.io 可发布状态
- 公开文档与代码一致
- benchmark 与验证流程成型

**Exit Criteria:**

- `cargo fmt --check` 通过
- `cargo test` 通过
- `cargo package` 通过
- `cargo publish --dry-run` 通过
- `CHANGELOG.md`、版本号、README 示例保持一致

## Cross-Cutting Tracks

### Real API Testing Track

- 正常成功路径始终优先使用真实 Alpaca API
- 每完成一个资源域，就补对应 live tests
- mock 只保留给异常路径

### Benchmark Track

- 从 Phase 1 开始建立最小 benchmark 基线
- Phase 2 以后每完成一个资源域就补对应 benchmark
- Phase 6 做最终收口和对比

### Documentation Track

- 每个 phase 结束后更新 `README.md`、`CHANGELOG.md`、`memory/`
- 新版本提交前必须更新 `CHANGELOG.md`
- `CHANGELOG.md` 记录各种新变化，不只记录结构变化
- phase 收尾时要完成完整验证、自动做一次 MINOR 版本升级，并在合并 `main` 推送后删除开发分支

## Recommended Execution Order From Now

- [x] Execute **Phase 1: Shared Core** first
- [x] Then execute **Phase 2: Stocks** as the first real resource implementation
- [ ] Then execute **Phase 3: Options**
- [ ] Then execute **Phase 4: Crypto**
- [ ] Then execute **Phase 5: News + Corporate Actions**
- [ ] Finish with **Phase 6: Release Hardening**

## Next Planning Step

This roadmap is the master plan. Each phase should now get its own focused implementation plan before coding starts. The recommended next detailed plan is:

- `Phase 3: Options`
