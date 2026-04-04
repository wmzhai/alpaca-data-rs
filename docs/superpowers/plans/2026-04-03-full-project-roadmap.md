# Full Project Roadmap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `alpaca-data` 从当前 `v0.1.0` 的 shared-core 基线起点，逐步推进到完成 release preparation，并在后续 `Phase 7` 决定是否执行最终发布的高性能 Alpaca Market Data API Rust 客户端。

**Architecture:** 开发主线按共享基础层和资源域拆成独立 phase，先做 transport、错误模型、分页、真实 API 测试基线，再按 `stocks -> options -> crypto -> news/corporate_actions -> release hardening -> release` 推进。benchmark、真实 API 测试和文档/CHANGELOG 作为横向轨道持续贯穿所有 phase，而不是只在最后补。

**Tech Stack:** Rust 2024 edition, Cargo library crate, reqwest transport, real Alpaca Market Data HTTP API integration tests, limited mock fault injection for exceptional paths only

---

## Phase Summary

- **Phase 0: Bootstrap**，已完成
- **Phase 1: Shared Core**，已完成（`v0.1.0`）
- **Phase 2: Stocks**，已完成（`v0.2.0`）
- **Phase 3: Options**，已完成（`v0.3.0`）
- **Phase 4: Crypto**，已完成（`v0.4.0`）
- **Phase 5: News + Corporate Actions**，已完成（`v0.5.0`）
- **Phase 6: Release Hardening**，当前分支已完成（`v0.6.0` phase-close candidate）
- **Phase 7: Release**

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
- `src/transport/endpoint.rs` 已提供完整 crypto HTTP 路由：historical、latest family 与 snapshots
- `src/transport/http.rs`、`src/transport/retry.rs`、`src/transport/rate_limit.rs` 已具备最小实际行为
- `tests/mock_transport_errors.rs` 已覆盖共享 transport 的异常路径
- `src/transport/pagination.rs` 已提供最小分页聚合与按页 stream helper
- `tests/live_crypto_latest_quotes_smoke.rs`、`tests/live_crypto_historical.rs`、`tests/live_crypto_latest.rs` 与 `tests/live_crypto_snapshots.rs` 已验证完整 crypto happy path
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

**Status:** Done in `v0.3.0`

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
- `benches/options.rs` 已建立 `options.chain` 的本地 benchmark baseline

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

**Status:** Done in `v0.4.0`

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

- 无凭证 crypto historical 请求通路
- `loc` 参数序列化与 historical endpoint 路由完成
- `latest_orderbooks` 真实行为完成
- `snapshots` 真实行为完成

**Exit Criteria:**

- `crypto()` 在无凭证配置下可运行成功路径测试
- `loc` 的默认值和显式值都完成验证
- crypto 真实 API 测试覆盖核心 happy path

## Phase 5: News + Corporate Actions

**Status:** Done in `v0.5.0`

**Delivered so far:**

- `news.list`、`news.list_all` 与 `news.list_stream` 已具备真实 HTTP 行为
- `news::ListRequest` 现在按官方 query 单词序列化 `start` / `end` / `sort` / `symbols` / `limit` / `include_content` / `exclude_contentless` / `page_token`
- `news::NewsItem` 与 `news::NewsImage` 现已补齐官方字段，并保持官方 `news` + `next_page_token` wrapper
- `tests/live_news.rs` 已用真实 Alpaca API 验证 `news` happy path 与分页便利层
- `corporate_actions.list`、`corporate_actions.list_all` 与 `corporate_actions.list_stream` 已具备真实 HTTP 行为
- `corporate_actions::CorporateActionType` 现已覆盖真实 API 接受的 15 个 query 值，`CorporateActions` 现已保留官方 bucketed wrapper、13 个 documented family typed model、`contract_adjustments` / `partial_calls` fallback 与 `other` unknown bucket
- `tests/live_corporate_actions.rs` 已用真实 Alpaca API 验证 `corporate_actions` happy path；共享 pagination helper 现已拒绝重复 `next_page_token`
- `tests/mock_news_corporate_actions_errors.rs` 现已覆盖 `news` / `corporate_actions` 的损坏 JSON、分页 merge 与重复 `next_page_token` 回归
- `benches/news_corporate_actions.rs` 现已为 `news.list` 与 `corporate_actions.list` 补齐本地 benchmark baseline

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
- Phase 5 的完整验证、文档总对齐与 MINOR 收尾版本已在 `v0.5.0` 完成；当前分支已把 `Phase 6: Release Hardening` 收敛到 `v0.6.0` 的收尾版本候选

**Exit Criteria:**

- `news` 和 `corporate_actions` 成功路径真实 API 测试通过
- 分页聚合和按页 stream 行为与共享框架一致

## Phase 6: Release Hardening

**Status:** Done on the current branch in `v0.6.0` as a phase-close candidate awaiting merge approval

**Goal:** 在不移除内部工作文档、也不自动发布 crates.io 的前提下，把项目从“功能可用”推进到“release-prepared”。

**Primary Scope:**

- 对外英文文档收口
- examples 与 rustdoc 补齐
- API coverage contract
- API sync skill
- package metadata 与 package boundary
- CI guardrails
- `cargo package`
- `cargo publish --dry-run`
- release checklist 与版本发布流程定型

**Files Likely Touched:**

- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `memory/` 中相关文档
- Create: `docs/` 下公开英文文档
- Create: `examples/` 下标准 examples
- Create: `tools/api-coverage/` 下 coverage manifest
- Create: `.agents/skills/alpaca-market-data-sync/`
- Create: `.github/workflows/ci.yml`

**Deliverables:**

- release-prepared crate 状态
- 公开英文文档、examples 和 rustdoc 与代码一致
- package 不包含 `.agents/`、`.github/`、`AGENTS.md`、`docs/superpowers/` 与 `memory/`
- API coverage contract 与 API sync skill 成型
- 预发布验证流程成型，但仍不自动发布 crates.io

**Exit Criteria:**

- `cargo fmt --check` 通过
- `cargo test` 通过
- 必要 live tests 通过
- `cargo check --examples` 通过
- `cargo test --doc` 通过
- `cargo doc --no-deps` 通过
- `cargo bench --no-run` 通过
- `cargo package` 通过
- `cargo publish --dry-run` 通过
- `CHANGELOG.md`、版本号、README、public docs、examples 与 rustdoc 保持一致
- package 产物中不包含 `.agents/`、`.github/`、`AGENTS.md`、`docs/superpowers/` 与 `memory/`

## Phase 7: Release

**Goal:** 在 `Phase 6` 完成 release preparation 之后，做最终 public-repo cleanup，并根据用户决策决定是否真正发布。

**Primary Scope:**

- 从 git 中移除 `docs/superpowers/`
- 从 git 中移除 `memory/`
- 将上述目录加入 `.gitignore`
- 最终 public-repo cleanup
- 可选的 GitHub Pages 叙述型站点准备
- 最终 release gate
- 根据用户明确指令决定是否真正发布 crates.io

**Files Likely Touched:**

- Modify: `.gitignore`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `Cargo.toml`
- Modify: `docs/` 下对外文档
- Delete: `docs/superpowers/`
- Delete: `memory/`

**Deliverables:**

- public repo 结构收口
- internal working docs 从 git 中退场
- 发布决策所需的最终检查材料

**Exit Criteria:**

- `docs/superpowers/` 与 `memory/` 已从 git 中移除
- `.gitignore` 已覆盖这两个内部目录
- `Phase 6` 的验证闭环仍保持可重复执行
- 是否真正发布 crates.io 由用户最终明确决定

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
- `Phase 6` 负责公开英文文档与 package boundary；`Phase 7` 负责 internal docs 的 git 移除与最终 public repo cleanup
- phase 收尾时要完成完整验证、自动做一次 MINOR 版本升级，并在合并 `main` 推送后删除开发分支

## Recommended Execution Order From Now

- [x] Execute **Phase 1: Shared Core** first
- [x] Then execute **Phase 2: Stocks** as the first real resource implementation
- [x] Then execute **Phase 3: Options**
- [x] Then execute **Phase 4: Crypto**
- [x] Then execute **Phase 5: News + Corporate Actions**
- [x] Execute **Phase 6: Release Hardening**
- [ ] Finish with **Phase 7: Release**

## Next Planning Step

This roadmap is the master plan. Each phase should now get its own focused implementation plan before coding starts. The current next detailed plan is:

- `Phase 7: Release`

After `Phase 6` closes, the next planning target is:

- `Phase 7: Release`
