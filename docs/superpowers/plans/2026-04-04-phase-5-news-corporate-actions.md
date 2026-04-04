# Phase 5 News and Corporate Actions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `news` 与 `corporate_actions` 两个 list/filter/pagination 型资源域完整落地为官方 Alpaca Market Data API HTTP 客户端模块，补齐真实 API happy-path、异常路径 mock 与本地 benchmark 基线。

**Architecture:** 这一 phase 继续复用 `stocks` / `options` / `crypto` 已经定型的 request-model-response-client 分层、共享 query writer、shared pagination helper 与 live test 模板。实现顺序按 `docs baseline -> news -> corporate_actions -> fault coverage + benchmark + phase completion` 推进，确保每一步都形成可验证的真实 API 增量。

**Tech Stack:** Rust 2024 edition, `reqwest` async client with `rustls`, `tokio`, `serde`/`serde_json`, shared pagination helpers, real Alpaca Market Data API integration tests, limited `wiremock` for exceptional-path tests only, `criterion` for local benchmark baselines

---

## Phase Start Gate

- 本计划已在 2026-04-04 获得用户确认，并已作为当前 phase 的执行基线。
- `news` baseline 已按 no-op freeze 重新核对并验证通过。
- `corporate_actions` / shared pagination 草稿已完成 reconciliation，只保留与计划一致的实现。

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

## Reconciliation Rule

在你确认本计划之后，先执行一轮 reconciliation：

- 保留已经提交且与计划一致的 `news` 代码、测试和文档
- 逐项比对当前工作区里的 `corporate_actions` / pagination 草稿
- 与计划一致的实现可直接吸收
- 与计划冲突的实现必须先修改，再进入对应 task 的正式提交

## Task Order

### Task 1: Baseline Reconciliation and News Freeze

**Files:**
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/system-map.md`
- Modify: `docs/superpowers/specs/2026-04-04-phase-5-news-corporate-actions-design.md`
- Modify: `docs/superpowers/plans/2026-04-04-phase-5-news-corporate-actions.md`
- Modify: `src/transport/endpoint.rs`
- Modify: `src/news/mod.rs`
- Modify: `src/news/client.rs`
- Modify: `src/news/request.rs`
- Modify: `src/news/response.rs`
- Modify: `src/news/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_news.rs`

- [x] Re-read the shipped `v0.4.1` `news` implementation against the approved phase design
- [x] Keep the existing `news` implementation unchanged if it matches the approved design; otherwise patch it before any further phase work
- [x] Re-run `tests/live_news.rs` and relevant unit/public API tests as the baseline freeze verification
- [x] Sync phase docs so they clearly distinguish “already shipped baseline” from “remaining work”
- [x] If any baseline adjustments were required, prepare the corresponding patch version commit; if no code/doc changes were needed, record that Task 1 is a no-op baseline freeze

### Task 2: Corporate Actions Buckets, Typed Families, and Pagination Convenience

**Files:**
- Modify: `src/common/response.rs`
- Modify: `src/transport/endpoint.rs`
- Modify: `src/transport/pagination.rs`
- Modify: `src/corporate_actions/mod.rs`
- Modify: `src/corporate_actions/client.rs`
- Modify: `src/corporate_actions/request.rs`
- Modify: `src/corporate_actions/response.rs`
- Modify: `src/corporate_actions/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_corporate_actions.rs`

- [x] Add failing request, response, route, and public API coverage for `corporate_actions.list`, `corporate_actions.list_all`, and `corporate_actions.list_stream`
- [x] Run the targeted tests and confirm they fail for the current placeholder implementation
- [x] Implement `/v1/corporate-actions` endpoint routing
- [x] Expand `CorporateActionType` to the full current real-API enum surface
- [x] Replace placeholder models with typed bucketed `CorporateActions` response families for the documented action schemas
- [x] Add conservative fallback modeling for `contract_adjustments`, `partial_calls`, and future unknown buckets without flattening the whole response into untyped JSON
- [x] Add repeated-`next_page_token` protection to the shared pagination helper so `corporate_actions` cannot hang on pathological server pagination
- [x] Implement `ListRequest` query serialization with official parameters and shared `PaginatedRequest`
- [x] Implement `ListResponse` serde + `PaginatedResponse` bucket merge logic
- [x] Implement `CorporateActionsClient::list`, `list_all`, and `list_stream`
- [x] Add live happy-path coverage using bounded real queries:
  - one query that proves multi-bucket decoding
  - one query that proves `list_all` success on a stable bounded dataset such as explicit `ids`
  - one query that proves `list_stream` yields multiple real pages without depending on unbounded history fetches
- [x] Run targeted tests, then `cargo test`
- [x] Sync docs and prepare patch version commit

### Task 3: Fault Injection and Benchmark

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

- [x] Add mock-only fault coverage for malformed JSON and pagination merge behavior in `news` / `corporate_actions`
- [x] Add a mock regression for repeated `next_page_token` so the shared guard stays covered
- [x] Add `news_corporate_actions` benchmark baseline and verify it compiles with `cargo bench --bench news_corporate_actions --no-run`
- [x] Run targeted tests and benchmark compile verification
- [x] Sync docs and prepare patch version commit

### Task 4: Phase Completion Candidate

**Files:**
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`
- Modify: `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

- [ ] Run full phase verification: format, full test suite, required live tests, benchmark compile
- [ ] Sync all docs to the final `Phase 5` state
- [ ] Bump MINOR version for phase completion and create the phase closing release commit
- [ ] Stop and wait for user approval before any fast-forward merge to `main`, push, or branch deletion
