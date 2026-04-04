# Phase 3 Options Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `options` 资源域完整落地为官方 Alpaca Market Data API HTTP `options` 模块，覆盖 historical、latest、snapshot/chain、metadata、真实 API 测试与 benchmark 基线。

**Architecture:** 这一 phase 直接复用 `stocks` 已经跑通的模块分层与分页 helper，但严格遵守 options 自身的 path、query 与返回体形状。实现顺序按 `historical -> latest/meta -> snapshot/chain -> benchmark/docs/phase completion` 推进，确保每一步都能形成可验证的真实 API 增量。

**Tech Stack:** Rust 2024 edition, `reqwest` async client with `rustls`, `tokio`, `serde`/`serde_json`, shared pagination helpers, real Alpaca Market Data API integration tests, limited `wiremock` for exceptional-path tests only, `criterion` for local benchmark baselines

---

## Execution Constraints

- 遵守 `AGENTS.md`：不要使用 `.worktrees/`；直接在普通 git 分支上执行
- 当前 phase 使用分支：`phase-3-options`
- 每个 task 完成前先跑对应验证，再同步 `CHANGELOG.md`、受影响文档和版本号
- 每个 task 完成后做一次 patch 版本提交
- `Phase 3` 收尾时执行一次 MINOR 版本升级
- 正常成功路径测试严禁 mock；尽量使用真实 Alpaca API

## File Structure

### Existing files to modify

- `Cargo.toml`
- `README.md`
- `CHANGELOG.md`
- `src/transport/endpoint.rs`
- `src/options/mod.rs`
- `src/options/client.rs`
- `src/options/enums.rs`
- `src/options/request.rs`
- `src/options/response.rs`
- `src/options/model.rs`
- `tests/public_api.rs`
- `memory/README.md`
- `memory/api/README.md`
- `memory/core/system-map.md`
- `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

### New files to create

- `tests/live_options_historical.rs`
- `tests/live_options_latest_metadata.rs`
- `tests/live_options_snapshots_chain.rs`
- `tests/mock_options_errors.rs`
- `benches/options.rs`

## Public API To Deliver

- `bars`
- `bars_all`
- `bars_stream`
- `trades`
- `trades_all`
- `trades_stream`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `snapshots_all`
- `snapshots_stream`
- `chain`
- `chain_all`
- `chain_stream`
- `exchange_codes`

## Task Order

### Task 1: Historical Bars and Trades

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/options/mod.rs`
- Modify: `src/options/client.rs`
- Modify: `src/options/enums.rs`
- Modify: `src/options/request.rs`
- Modify: `src/options/response.rs`
- Modify: `src/options/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_options_historical.rs`

- [x] Add failing public API and route coverage for `options.bars` and `options.trades`
- [x] Add failing request/response serialization tests for official query words and wrapper shapes
- [x] Run the targeted tests and confirm they fail for the missing behavior
- [x] Implement `Endpoint` routes for `/v1beta1/options/bars` and `/v1beta1/options/trades`
- [x] Implement `TimeFrame` and `OptionsFeed` / `ContractType` official string encoding needed by this task
- [x] Implement typed `Bar` / `Trade` models and `BarsResponse` / `TradesResponse` wrappers
- [x] Implement `BarsRequest` / `TradesRequest` query serialization with official field words only
- [x] Implement `OptionsClient::bars` and `OptionsClient::trades`
- [x] Add live happy-path coverage using real Alpaca API with bounded query windows
- [x] Run targeted tests, then `cargo test`
- [x] Sync docs and prepare patch version commit

### Task 2: Latest and Metadata

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/options/client.rs`
- Modify: `src/options/request.rs`
- Modify: `src/options/response.rs`
- Modify: `src/options/model.rs`
- Modify: `src/options/enums.rs`
- Create: `tests/live_options_latest_metadata.rs`
- Modify: `tests/public_api.rs`

- [x] Add failing tests for `latest_quotes`, `latest_trades`, and `exchange_codes`
- [x] Implement routes, requests, typed quote model, typed latest responses, and top-level metadata map shape
- [x] Verify with real API and sync docs/version

### Task 3: Snapshots, Chain, and Convenience Layer

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/options/client.rs`
- Modify: `src/options/request.rs`
- Modify: `src/options/response.rs`
- Modify: `src/options/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_options_snapshots_chain.rs`
- Create: `tests/mock_options_errors.rs`

- [x] Add failing tests for `snapshots`, `chain`, `*_all`, and `*_stream`
- [x] Implement official path/query handling, including `underlying_symbol`, `r#type`, `updated_since`, and pagination
- [x] Implement `Snapshot` / `Greeks` typed model with official field words
- [x] Implement pagination merge logic for `snapshots` and `chain`
- [x] Verify with real API plus targeted mock fault tests, then sync docs/version

### Task 4: Benchmark and Phase Completion

**Files:**
- Modify: `Cargo.toml`
- Create: `benches/options.rs`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`
- Modify: `AGENTS.md`
- Modify: `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

- [ ] Add `options` benchmark baseline
- [ ] Run full phase verification: format, unit/integration/live tests, benchmark compile
- [ ] Sync all docs to the final `Phase 3` state
- [ ] Bump MINOR version for phase completion and finish branch per repo rules
