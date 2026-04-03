# Crate Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 初始化 `alpaca-data` 的最小可发布 Rust crate 骨架，落下 `Cargo.toml`、`src/` 公开模块、基础测试与文档导航更新。

**Architecture:** 先建立最小公开 API 入口和资源模块壳，保证 `alpaca_data::Client`、资源 accessor 与资源模块短类型名可被 `cargo test` 编译验证。本次不实现任何真实 Alpaca HTTP 逻辑，只为后续逐步填充接口留出稳定骨架。

**Tech Stack:** Rust 2024 edition, standard library test harness, Cargo library crate

---

### Task 1: 建立 manifest 与忽略规则

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`

- [ ] Step 1: 写入标准 crate manifest，包名使用 `alpaca-data`
- [ ] Step 2: 写入最小 `.gitignore`，忽略 `target/`、备份文件和环境文件

### Task 2: 建立最小公开模块骨架

**Files:**
- Create: `src/lib.rs`
- Create: `src/client.rs`
- Create: `src/error.rs`
- Create: `src/auth.rs`
- Create: `src/common/mod.rs`
- Create: `src/common/enums.rs`
- Create: `src/common/time.rs`
- Create: `src/common/response.rs`
- Create: `src/transport/mod.rs`
- Create: `src/transport/http.rs`
- Create: `src/transport/retry.rs`
- Create: `src/transport/pagination.rs`
- Create: `src/transport/rate_limit.rs`
- Create: `src/stocks/mod.rs`
- Create: `src/stocks/client.rs`
- Create: `src/stocks/request.rs`
- Create: `src/stocks/response.rs`
- Create: `src/stocks/model.rs`
- Create: `src/stocks/enums.rs`
- Create: `src/options/mod.rs`
- Create: `src/options/client.rs`
- Create: `src/options/request.rs`
- Create: `src/options/response.rs`
- Create: `src/options/model.rs`
- Create: `src/options/enums.rs`
- Create: `src/crypto/mod.rs`
- Create: `src/crypto/client.rs`
- Create: `src/crypto/request.rs`
- Create: `src/crypto/response.rs`
- Create: `src/crypto/model.rs`
- Create: `src/crypto/enums.rs`
- Create: `src/news/mod.rs`
- Create: `src/news/client.rs`
- Create: `src/news/request.rs`
- Create: `src/news/response.rs`
- Create: `src/news/model.rs`
- Create: `src/corporate_actions/mod.rs`
- Create: `src/corporate_actions/client.rs`
- Create: `src/corporate_actions/request.rs`
- Create: `src/corporate_actions/response.rs`
- Create: `src/corporate_actions/model.rs`

- [ ] Step 1: 定义 `Client`、`ClientBuilder`、`Error` 和共享 `Inner`
- [ ] Step 2: 为五个资源域建立公开模块、resource client 与最小 request/response/model 类型
- [ ] Step 3: 在 `lib.rs` 中 re-export 根入口与资源模块

### Task 3: 先写编译期公开 API 测试，再让骨架通过

**Files:**
- Create: `tests/public_api.rs`

- [ ] Step 1: 先写失败的公开 API 编译使用测试，覆盖 `Client::new()`、`Client::builder()`、资源 accessor 和短类型名
- [ ] Step 2: 运行 `cargo test`，确认在骨架完成前失败
- [ ] Step 3: 补齐最小实现，让测试通过
- [ ] Step 4: 再运行 `cargo test`，确认通过

### Task 4: 同步记忆文档中的真实结构

**Files:**
- Modify: `memory/core/system-map.md`
- Modify: `memory/README.md`

- [ ] Step 1: 把已经存在的 `Cargo.toml`、`src/`、`tests/` 更新为真实仓库事实
- [ ] Step 2: 保持其余未落地部分继续标记为后续实现方向

### Task 5: 最终验证

**Files:**
- No file changes; verification only

- [ ] Step 1: 运行 `cargo test`
- [ ] Step 2: 运行 `cargo metadata --no-deps`
- [ ] Step 3: 检查 `git status --short --branch`
- [ ] Step 4: 不做 commit（用户已明确要求）
