# Phase 5 News and Corporate Actions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `news` 与 `corporate_actions` 两个 list/filter/pagination 型资源域完整落地为官方 Alpaca Market Data API HTTP 客户端模块，补齐真实 API happy-path、异常路径 mock 与本地 benchmark 基线。

**Architecture:** 这一 phase 继续复用 `stocks` / `options` / `crypto` 已经定型的 request-model-response-client 分层、共享 query writer、shared pagination helper 与 live test 模板。实现顺序按 `docs baseline -> news -> corporate_actions -> fault coverage + benchmark + phase completion` 推进，确保每一步都形成可验证的真实 API 增量。

**Tech Stack:** Rust 2024 edition, `reqwest` async client with `rustls`, `tokio`, `serde`/`serde_json`, shared pagination helpers, real Alpaca Market Data API integration tests, limited `wiremock` for exceptional-path tests only, `criterion` for local benchmark baselines

---

## Execution Constraints

- 遵守 `AGENTS.md`：不要使用 `.worktrees/`；直接在普通 git 分支上执行
- 当前 phase 使用分支：`phase-5-news-corporate-actions`
- 每个 task 完成前先跑对应验证，再同步 `CHANGELOG.md`、受影响文档和版本号
- 每个 task 完成后做一次 patch 版本提交
- `Phase 5` 收尾时执行一次 MINOR 版本升级
- 正常成功路径测试严禁 mock；尽量使用真实 Alpaca API

## File Structure

### Existing files to modify

- `Cargo.toml`
- `README.md`
- `CHANGELOG.md`
- `AGENTS.md`
- `src/transport/endpoint.rs`
- `src/news/mod.rs`
- `src/news/client.rs`
- `src/news/request.rs`
- `src/news/response.rs`
- `src/news/model.rs`
- `src/corporate_actions/mod.rs`
- `src/corporate_actions/client.rs`
- `src/corporate_actions/request.rs`
- `src/corporate_actions/response.rs`
- `src/corporate_actions/model.rs`
- `tests/public_api.rs`
- `memory/README.md`
- `memory/api/README.md`
- `memory/core/system-map.md`
- `memory/core/workflows.md`
- `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

### New files to create

- `tests/live_news.rs`
- `tests/live_corporate_actions.rs`
- `tests/mock_news_corporate_actions_errors.rs`
- `benches/news_corporate_actions.rs`

## Public API To Deliver

- `news.list`
- `news.list_all`
- `news.list_stream`
- `corporate_actions.list`
- `corporate_actions.list_all`
- `corporate_actions.list_stream`

## Task Order

### Task 1: News Endpoint, Typed Models, and Pagination Convenience

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/news/mod.rs`
- Modify: `src/news/client.rs`
- Modify: `src/news/request.rs`
- Modify: `src/news/response.rs`
- Modify: `src/news/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_news.rs`

- [x] Add failing request, response, route, and public API coverage for `news.list`, `news.list_all`, and `news.list_stream`
- [x] Run the targeted tests and confirm they fail for the current placeholder implementation
- [x] Implement `/v1beta1/news` endpoint routing
- [x] Replace placeholder `NewsItem` with typed `NewsItem` / `NewsImage` models using official field words
- [x] Implement `ListRequest` query serialization with official parameters and shared `PaginatedRequest`
- [x] Implement `ListResponse` serde + `PaginatedResponse`
- [x] Implement `NewsClient::list`, `list_all`, and `list_stream`
- [x] Add live happy-path coverage with bounded `AAPL` queries and forced pagination via low `limit`
- [x] Run targeted tests, then `cargo test`
- [x] Sync docs and prepare patch version commit

### Task 2: Corporate Actions Buckets, Typed Families, and Pagination Convenience

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/corporate_actions/mod.rs`
- Modify: `src/corporate_actions/client.rs`
- Modify: `src/corporate_actions/request.rs`
- Modify: `src/corporate_actions/response.rs`
- Modify: `src/corporate_actions/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_corporate_actions.rs`

- [ ] Add failing request, response, route, and public API coverage for `corporate_actions.list`, `corporate_actions.list_all`, and `corporate_actions.list_stream`
- [ ] Run the targeted tests and confirm they fail for the current placeholder implementation
- [ ] Implement `/v1/corporate-actions` endpoint routing
- [ ] Expand `CorporateActionType` to the full current real-API enum surface
- [ ] Replace placeholder models with typed bucketed `CorporateActions` response families for the documented action schemas
- [ ] Add conservative fallback modeling for `contract_adjustments`, `partial_calls`, and future unknown buckets without flattening the whole response into untyped JSON
- [ ] Implement `ListRequest` query serialization with official parameters and shared `PaginatedRequest`
- [ ] Implement `ListResponse` serde + `PaginatedResponse` bucket merge logic
- [ ] Implement `CorporateActionsClient::list`, `list_all`, and `list_stream`
- [ ] Add live happy-path coverage using broad historical date windows that return multiple buckets and multiple pages
- [ ] Run targeted tests, then `cargo test`
- [ ] Sync docs and prepare patch version commit

### Task 3: Fault Injection, Benchmark, and Phase Completion

**Files:**
- Modify: `Cargo.toml`
- Create: `tests/mock_news_corporate_actions_errors.rs`
- Create: `benches/news_corporate_actions.rs`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`
- Modify: `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

- [ ] Add mock-only fault coverage for malformed JSON and pagination merge behavior in `news` / `corporate_actions`
- [ ] Add `news_corporate_actions` benchmark baseline and verify it compiles with `cargo bench --bench news_corporate_actions --no-run`
- [ ] Run full phase verification: format, unit/integration/live tests, benchmark compile
- [ ] Sync all docs to the final `Phase 5` state
- [ ] Bump MINOR version for phase completion and finish branch per repo rules
