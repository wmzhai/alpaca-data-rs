# Phase 6 Release Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在不提前清理内部工作目录和不自动发布 crates.io 的前提下，把 `alpaca-data` 收敛到标准的 release-prepared 状态。

**Architecture:** 这一 phase 不再新增资源域，而是把“对外公开面”和“发布产物边界”做实。执行顺序按 `public docs -> examples + rustdoc -> API coverage contract -> API sync skill -> metadata + package boundary + CI -> pre-release verification` 推进，确保内部工作文档继续保留在 git 中，但不会污染 published crate。

**Tech Stack:** Rust 2024 edition, Cargo library crate, `reqwest` async HTTP client, `tokio`, `serde`/`serde_json`, real Alpaca Market Data API live tests, limited `wiremock` for fault injection only, docs.rs-compatible rustdoc, optional GitHub Actions CI

---

## Phase Start Gate

- 本计划已基于 2026-04-04 获批的 `Phase 6` 边界编写。
- `Phase 5` 已经完成并已落到 `main`，当前基线版本为 `v0.5.0`。
- `docs/superpowers/` 与 `memory/` 在整个 `Phase 6` 都继续保留在 git 中；它们的移除明确属于 `Phase 7`。
- 本 phase 只做发布准备，不自动发布 crates.io。

## Execution Constraints

- 遵守 `AGENTS.md`：不要使用 `.worktrees/`；直接在普通 git 分支上执行
- 当前 phase 使用分支：`phase-6-release-hardening`
- 每个 task 完成前先跑对应验证，再同步 `CHANGELOG.md`、受影响文档和版本号
- 每个 task 完成后做一次 patch 版本提交
- `Phase 6` 收尾时执行一次 MINOR 版本升级
- 正常成功路径测试严禁 mock；live tests 尽量使用真实 Alpaca API
- public-facing docs 必须使用英文；`docs/superpowers/` 与 `memory/` 在 `Phase 6` 可以继续保留当前内部工作语言
- 如果执行到 package metadata 时仍缺少明确的 license 决策，必须停下来向用户确认，不能擅自决定 license

## File Structure

### Existing files to modify

- `Cargo.toml`
- `README.md`
- `CHANGELOG.md`
- `AGENTS.md`
- `src/lib.rs`
- `src/client.rs`
- `src/stocks/mod.rs`
- `src/options/mod.rs`
- `src/crypto/mod.rs`
- `src/news/mod.rs`
- `src/corporate_actions/mod.rs`
- `memory/README.md`
- `memory/api/README.md`
- `memory/core/invariants.md`
- `memory/core/system-map.md`
- `memory/core/workflows.md`
- `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- `docs/superpowers/specs/2026-04-04-phase-6-release-hardening-design.md`
- `docs/superpowers/plans/2026-04-04-phase-6-release-hardening.md`

### New files to create

- `docs/getting-started.md`
- `docs/authentication.md`
- `docs/layers.md`
- `docs/api-coverage.md`
- `docs/examples.md`
- `docs/release-checklist.md`
- `examples/client_builder.rs`
- `examples/stocks_latest_bar.rs`
- `examples/stocks_bars_all.rs`
- `examples/options_chain.rs`
- `examples/crypto_latest_quotes.rs`
- `examples/news_list.rs`
- `examples/corporate_actions_list.rs`
- `tools/api-coverage/market-data-api.json`
- `.agents/skills/alpaca-market-data-sync/SKILL.md`
- `.github/workflows/ci.yml`

## Task Order

### Task 1: Public Docs Baseline and English Documentation Surface

**Files:**
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/invariants.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`
- Modify: `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- Create: `docs/getting-started.md`
- Create: `docs/authentication.md`
- Create: `docs/layers.md`
- Create: `docs/examples.md`
- Create: `docs/release-checklist.md`

- [x] Reconcile all stale docs that still claim `Phase 5` is only a merge candidate; update them to the real `v0.5.0` baseline
- [x] Rewrite the public README in English so it reflects the current shipped API surface and the approved `Phase 6` / `Phase 7` boundary
- [x] Add the first set of public English docs under `docs/`, keeping them independent from `docs/superpowers/` and `memory/`
- [x] Document in the public docs that `docs.rs` is the API-doc entrypoint and that `Phase 6` is release-prep only
- [x] Sync roadmap, AGENTS, and memory docs so they all agree that internal-doc removal is deferred to `Phase 7`
- [x] Run a doc consistency pass across README, `docs/`, AGENTS, memory, and roadmap
- [x] Prepare the patch version commit for the docs baseline task

### Task 2: Examples and Rustdoc

**Files:**
- Modify: `Cargo.toml`
- Modify: `CHANGELOG.md`
- Modify: `src/lib.rs`
- Modify: `src/client.rs`
- Modify: `src/stocks/mod.rs`
- Modify: `src/options/mod.rs`
- Modify: `src/crypto/mod.rs`
- Modify: `src/news/mod.rs`
- Modify: `src/corporate_actions/mod.rs`
- Modify: `README.md`
- Modify: `docs/examples.md`
- Modify: `memory/core/system-map.md`
- Create: `examples/client_builder.rs`
- Create: `examples/stocks_latest_bar.rs`
- Create: `examples/stocks_bars_all.rs`
- Create: `examples/options_chain.rs`
- Create: `examples/crypto_latest_quotes.rs`
- Create: `examples/news_list.rs`
- Create: `examples/corporate_actions_list.rs`

- [x] Add crate-level rustdoc to `src/lib.rs` with a minimal `Client` setup example and resource entrypoint overview
- [x] Add module-level rustdoc for each public resource module, documenting mirror methods vs convenience methods
- [x] Add concise docs for `Client`, `ClientBuilder`, and the resource accessor methods that appear in the public entrypoint
- [x] Create the short standard examples listed in the design doc, using async-only happy paths and current public method names
- [x] Cross-link README and `docs/examples.md` to the new examples directory
- [x] Run `cargo fmt`
- [x] Run `cargo check --examples`
- [x] Run `cargo test --doc`
- [x] Prepare the patch version commit for the examples and rustdoc task

### Task 3: API Coverage Contract

**Files:**
- Modify: `Cargo.toml`
- Modify: `CHANGELOG.md`
- Create: `docs/api-coverage.md`
- Create: `tools/api-coverage/market-data-api.json`
- Modify: `README.md`
- Modify: `docs/layers.md`
- Modify: `docs/release-checklist.md`
- Modify: `memory/api/README.md`

- [ ] Enumerate the in-scope official Market Data HTTP endpoints from the current official docs/OpenAPI and map each mirror endpoint to its Rust method
- [ ] Separate the mirror layer table from the convenience layer additions so `*_all` / `*_stream` are documented as additive helpers, not official endpoints
- [ ] Record explicit out-of-scope official resources such as Trading API, Broker API, WebSocket/SSE, and any Market Data resource families not yet adopted
- [ ] Store the human-readable mapping in `docs/api-coverage.md`
- [ ] Store the machine-readable mapping in `tools/api-coverage/market-data-api.json`
- [ ] Cross-check the manifest against the current codebase and fix any documentation-side mismatch before freezing the contract
- [ ] Prepare the patch version commit for the API coverage contract task

### Task 4: API Sync Skill

**Files:**
- Modify: `Cargo.toml`
- Modify: `CHANGELOG.md`
- Create: `.agents/skills/alpaca-market-data-sync/SKILL.md`
- Modify: `docs/api-coverage.md`
- Modify: `docs/release-checklist.md`
- Modify: `AGENTS.md`

- [ ] Create the internal skill scaffold for Alpaca Market Data API audits
- [ ] Teach the skill to use the official Market Data OpenAPI, the official reference pages, and the local `tools/api-coverage/market-data-api.json` manifest as its sources of truth
- [ ] Document the expected audit outputs: missing mirror endpoints, parameter drift, response-field drift, convenience-layer compatibility notes, and recommended follow-up actions
- [ ] Explicitly document that mirror drift must be fixed before convenience-layer compatibility is re-validated
- [ ] Add operator guidance showing how future sessions should run the audit before release work or API sync work
- [ ] Prepare the patch version commit for the API sync skill task

### Task 5: Package Metadata, Package Boundary, and CI Guardrails

**Files:**
- Modify: `Cargo.toml`
- Modify: `CHANGELOG.md`
- Modify: `README.md`
- Modify: `docs/release-checklist.md`
- Modify: `AGENTS.md`
- Modify: `memory/core/workflows.md`
- Create: `.github/workflows/ci.yml`

- [ ] Add the missing release metadata to `Cargo.toml`: `repository`, `documentation`, valid crates.io `keywords`, valid crates.io `categories`, and package boundary controls
- [ ] Exclude `docs/superpowers/**` and `memory/**` from the published crate while keeping both directories tracked in git for `Phase 6`
- [ ] Only add docs.rs-specific package metadata if the doc build actually needs it; avoid cargo manifest noise otherwise
- [ ] Add a minimal CI workflow that verifies format, tests, example compilation, docs build, and package creation without attempting publication
- [ ] If no explicit license decision exists by this point, stop and ask the user before finalizing publish metadata
- [ ] Run `cargo package --list` and verify that the package boundary matches the approved design
- [ ] Prepare the patch version commit for the metadata and CI task

### Task 6: Pre-Release Verification and Phase Completion Candidate

**Files:**
- Modify: `Cargo.toml`
- Modify: `README.md`
- Modify: `CHANGELOG.md`
- Modify: `AGENTS.md`
- Modify: `memory/README.md`
- Modify: `memory/api/README.md`
- Modify: `memory/core/invariants.md`
- Modify: `memory/core/system-map.md`
- Modify: `memory/core/workflows.md`
- Modify: `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- Modify: `docs/release-checklist.md`

- [ ] Run full phase verification:
  - `cargo fmt --check`
  - `cargo test`
  - required live tests with real Alpaca credentials
  - `cargo check --examples`
  - `cargo test --doc`
  - `cargo doc --no-deps`
  - `cargo bench --no-run`
  - `cargo package`
  - `cargo publish --dry-run`
- [ ] Verify that README, public docs, examples, rustdoc, API coverage manifest, AGENTS, memory, and roadmap all describe the same final `Phase 6` reality
- [ ] Bump the MINOR version for phase completion and create the phase closing release commit
- [ ] Stop and wait for user approval before any fast-forward merge to `main`, push, or branch deletion
