# Phase 4 Crypto Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 `crypto` 资源域完整落地为官方 Alpaca Market Data API HTTP `crypto` 模块，覆盖 historical、latest、snapshots、真实 API 测试与 benchmark 基线。

**Architecture:** 这一 phase 继续复用 `stocks` / `options` 已经稳定下来的 request-model-response-client 分层与分页 helper，但严格遵守 crypto 自身的 path、query、无凭证行为和小数字段形状。实现顺序按 `historical -> latest -> snapshots/loc/no-auth -> benchmark/docs/phase completion` 推进，确保每一步都形成可验证的真实 API 增量。

**Tech Stack:** Rust 2024 edition, `reqwest` async client with `rustls`, `tokio`, `serde`/`serde_json`, shared pagination helpers, real Alpaca Market Data API integration tests, limited `wiremock` for exceptional-path tests only, `criterion` for local benchmark baselines

---

## Execution Constraints

- 遵守 `AGENTS.md`：不要使用 `.worktrees/`；直接在普通 git 分支上执行
- 当前 phase 使用分支：`phase-4-crypto`
- 每个 task 完成前先跑对应验证，再同步 `CHANGELOG.md`、受影响文档和版本号
- 每个 task 完成后做一次 patch 版本提交
- `Phase 4` 收尾时执行一次 MINOR 版本升级
- 正常成功路径测试严禁 mock；尽量使用真实 Alpaca API

## File Structure

### Existing files to modify

- `Cargo.toml`
- `README.md`
- `CHANGELOG.md`
- `AGENTS.md`
- `src/transport/endpoint.rs`
- `src/crypto/mod.rs`
- `src/crypto/client.rs`
- `src/crypto/enums.rs`
- `src/crypto/request.rs`
- `src/crypto/response.rs`
- `src/crypto/model.rs`
- `tests/public_api.rs`
- `memory/README.md`
- `memory/api/README.md`
- `memory/core/system-map.md`
- `memory/core/workflows.md`
- `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

### New files to create

- `tests/live_crypto_historical.rs`
- `tests/live_crypto_latest.rs`
- `tests/live_crypto_snapshots.rs`
- `tests/mock_crypto_errors.rs`
- `benches/crypto.rs`

## Public API To Deliver

- `bars`
- `bars_all`
- `bars_stream`
- `quotes`
- `quotes_all`
- `quotes_stream`
- `trades`
- `trades_all`
- `trades_stream`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `latest_orderbooks`
- `snapshots`

## Task Order

### Task 1: Historical Endpoints and Pagination Convenience

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/crypto/mod.rs`
- Modify: `src/crypto/client.rs`
- Modify: `src/crypto/enums.rs`
- Modify: `src/crypto/request.rs`
- Modify: `src/crypto/response.rs`
- Modify: `src/crypto/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_crypto_historical.rs`

- [x] Add failing route, request, response, and public API coverage for `bars`, `quotes`, `trades`, `bars_all`, `bars_stream`, `quotes_all`, `quotes_stream`, `trades_all`, and `trades_stream`
- [x] Run the targeted tests and confirm they fail for the missing behavior, including the incorrect `us1` path
- [x] Implement official `Loc` string encoding for `us`, `us-1`, and `eu-1`
- [x] Implement historical endpoint routing for `/v1beta3/crypto/{loc}/bars`, `/quotes`, and `/trades`
- [x] Replace placeholder crypto models with typed `Bar`, `Quote`, and `Trade` models using official field words and decimal sizes
- [x] Implement historical request query serialization and shared pagination trait impls
- [x] Implement `CryptoClient::bars`, `quotes`, `trades`, and their `*_all` / `*_stream` helpers without credential gating
- [x] Add live happy-path coverage using public crypto pairs and bounded page windows
- [x] Run targeted tests, then `cargo test`
- [x] Sync docs and prepare patch version commit

### Task 2: Latest Endpoints and Orderbook Modeling

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/crypto/mod.rs`
- Modify: `src/crypto/client.rs`
- Modify: `src/crypto/request.rs`
- Modify: `src/crypto/response.rs`
- Modify: `src/crypto/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_crypto_latest.rs`

- [x] Add failing tests for `latest_bars`, `latest_quotes`, `latest_trades`, and `latest_orderbooks`
- [x] Run the targeted tests and confirm they fail for the missing latest behavior
- [x] Implement latest endpoint routing and query serialization with `symbols` only
- [x] Implement typed latest response wrappers and `Orderbook` / `OrderbookLevel` models with official field words
- [x] Implement `CryptoClient::latest_bars`, `latest_quotes`, `latest_trades`, and `latest_orderbooks`
- [x] Add live happy-path coverage for all latest endpoints across multiple `loc` values
- [x] Run targeted tests, then `cargo test`
- [x] Sync docs and prepare patch version commit

### Task 3: Snapshots, No-Auth Guarantees, and Fault Coverage

**Files:**
- Modify: `src/transport/endpoint.rs`
- Modify: `src/crypto/client.rs`
- Modify: `src/crypto/request.rs`
- Modify: `src/crypto/response.rs`
- Modify: `src/crypto/model.rs`
- Modify: `tests/public_api.rs`
- Create: `tests/live_crypto_snapshots.rs`
- Create: `tests/mock_crypto_errors.rs`

- [x] Add failing tests for `snapshots`, no-auth client behavior, and crypto fault cases
- [x] Run the targeted tests and confirm they fail for the missing snapshot behavior
- [x] Implement `snapshots` routing, query serialization, typed `Snapshot` response/model, and official wrapper shape without pagination fields
- [x] Add live coverage for `Client::builder().build()?.crypto()` success paths and explicit `Loc::Us1` / `Loc::Eu1` routing
- [x] Add mock fault tests only for malformed JSON / invalid transport responses and any crypto-specific pagination error path still worth preserving
- [x] Run targeted tests, then `cargo test`
- [x] Sync docs and prepare patch version commit

### Task 4: Benchmark and Phase Completion

**Files:**
- Modify: `Cargo.toml`
- Create: `benches/crypto.rs`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`
- Modify: `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`

- [ ] Add `crypto` benchmark baseline
- [ ] Run full phase verification: format, unit/integration/live tests, benchmark compile
- [ ] Sync all docs to the final `Phase 4` state
- [ ] Bump MINOR version for phase completion and finish branch per repo rules
