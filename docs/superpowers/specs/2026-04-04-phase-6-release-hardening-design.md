# Phase 6 Release Hardening Design

**Date:** 2026-04-04
**Status:** Approved for implementation planning
**Target Phase:** `Phase 6: Release Hardening`

## Goal

将 `alpaca-data` 从“功能面已完整覆盖当前目标范围”的状态推进到“可以认真准备 crates.io 发布”的状态，但这一 phase 只做发布准备，不做最终发布，也不在这一 phase 移除仍在驱动开发的内部工作文档。

## Approval Gate

- 本 phase 的边界已在 2026-04-04 获得用户确认。
- `docs/superpowers/` 与 `memory/` 在 `Phase 6` 继续保留在 git 中，用于驱动后续工作。
- `docs/superpowers/` 与 `memory/` 的 git 移除、`.gitignore` 收口，以及是否真正发布到 crates.io，统一延后到 `Phase 7: Release`。

## Current Baseline

截至 `v0.5.0`：

- `Phase 1: Shared Core` 到 `Phase 5: News + Corporate Actions` 均已完成并已落到 `main`
- `stocks`、`options`、`crypto`、`news` 与 `corporate_actions` 已形成五个完整资源模板
- 真实 API happy-path、异常路径 mock 故障注入和本地 benchmark baseline 已覆盖当前目标资源范围
- 仓库仍缺少对外英文文档、社区标准示例、系统性的 rustdoc、API 覆盖对照文档、API 同步 skill、完整的 package metadata 与预发布验证闭环
- `docs/superpowers/` 与 `memory/` 当前仍在 git 中，且仍承担工作流驱动职责

## Scope

`Phase 6` 只覆盖以下六个工作面：

1. 对外英文文档收口
2. 短小标准 examples 与 rustdoc 补齐
3. 本地 API 与官方 Market Data API 的 coverage contract
4. 官方 API 同步 / 审计 skill
5. package metadata、package boundary 与 release guardrails
6. 预发布验证闭环

## Non-Goals

- 不新增新的 Alpaca Market Data 资源域
- 不实现 Trading API、Broker API、WebSocket / SSE
- 不在这一 phase 从 git 中移除 `docs/superpowers/` 或 `memory/`
- 不在这一 phase 把 `docs/superpowers/` 或 `memory/` 加入 `.gitignore`
- 不在这一 phase 自动发布 crates.io
- 不在这一 phase 强制上线 GitHub Pages
- 不为了“更 Rust”而修改已经与官方 HTTP API 对齐的原始单词

## Canonical Standard

`Phase 6` 继续遵守已有 API 语义标准，并新增发布层面的社区标准约束。

API 语义标准：

- Alpaca Market Data API reference: https://docs.alpaca.markets/reference/api-references
- Alpaca Market Data OpenAPI: https://docs.alpaca.markets/openapi/market-data-api.json

发布与文档标准：

- Cargo manifest reference: https://doc.rust-lang.org/cargo/reference/manifest.html
- docs.rs about: https://docs.rs/about
- docs.rs builds: https://docs.rs/about/builds
- GitHub Pages publishing source: https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site

## Phase Boundary Decision

### Phase 6 owns

- 公开 README、`docs/`、examples、rustdoc 和 release checklist 的对外质量
- `Cargo.toml` 的 crates.io / docs.rs 元数据
- published crate 的 package boundary 干净度
- mirror layer 和官方 API 的对应文档
- 供后续 Codex 会话执行的 API 审计 skill
- 发布前的验证命令与流程

### Phase 7 owns

- 从 git 中移除 `docs/superpowers/`
- 从 git 中移除 `memory/`
- 把上述目录加入 `.gitignore`
- 最终 public-repo cleanup
- 是否建立 GitHub Pages 叙述型站点
- 是否真正执行 crates.io 发布

## Public Documentation Strategy

`Phase 6` 的对外文档采用“公开文档前移，内部文档暂时保留”的策略。

### Public docs

对外公开文档统一放在仓库根和 `docs/` 下，并统一使用英文：

- `README.md`
- `docs/getting-started.md`
- `docs/authentication.md`
- `docs/layers.md`
- `docs/api-coverage.md`
- `docs/examples.md`
- `docs/release-checklist.md`

这些文档必须能够独立解释：

- 这个 crate 做什么
- 如何认证
- mirror layer 与 convenience layer 的边界
- 当前覆盖了哪些官方 endpoint
- 如何运行 examples 和 live tests
- 发布前需要做哪些验证

### Internal docs

`docs/superpowers/` 与 `memory/` 在 `Phase 6` 继续保留并继续服务于内部工作流。它们不是这一 phase 的对外文档主入口，也不要求这一 phase 先做 git 清理。

## Documentation Hosting Decision

社区标准做法定为：

- `docs.rs` 是 API 文档主入口
- crates.io 页面只承担 crate 元数据和 README 展示
- GitHub 仓库内的 `docs/` 负责承载对外 narrative docs
- GitHub Pages 在 `Phase 6` 只保留为可选后续项，不作为当前 phase 的必做项

这意味着：

- `Cargo.toml` 需要补上 `documentation`
- 若 `homepage` 没有独立站点，不在 `Phase 6` 强行伪造一个 URL
- 不在 `Phase 6` 自动部署 GitHub Pages

## Package Boundary Decision

`docs/superpowers/` 与 `memory/` 既然在 `Phase 6` 仍需保留在 git 中，就不能通过“先删掉目录”来保持 crate 干净；相反，应通过 Cargo package boundary 控制发布产物。

因此这一 phase 的发布边界约束是：

- 目录仍保留在仓库
- 但 published crate 不应包含 `docs/superpowers/**`
- published crate 不应包含 `memory/**`
- package boundary 通过 `Cargo.toml` 的 `exclude` / `include` 机制显式控制

## Examples Strategy

示例必须短小、真实、社区标准，不引入自定义脚手架。

计划中的 examples：

- `examples/client_builder.rs`
- `examples/stocks_latest_bar.rs`
- `examples/stocks_bars_all.rs`
- `examples/options_chain.rs`
- `examples/crypto_latest_quotes.rs`
- `examples/news_list.rs`
- `examples/corporate_actions_list.rs`

示例约束：

- 全部使用 `async`
- 默认读取环境变量认证
- 名词与方法名直接对应当前公开 API
- 优先展示最小成功路径，而不是复杂抽象

## Rustdoc Strategy

`docs.rs` 可读性依赖 rustdoc，而不是只依赖 README。

`Phase 6` 的 rustdoc 目标：

- `src/lib.rs` 补 crate-level docs
- 每个公开资源模块的 `mod.rs` 补 module-level docs
- `Client`、`ClientBuilder`、资源入口方法补基础 docs
- 每个资源 client 的核心 mirror 方法与 convenience 方法补最小可读 docs
- 例子优先使用 `no_run` 或与 examples 目录保持一致的最小代码片段

不把这一 phase 扩展成“给每个字段逐个写百科式注释”；重点是让 crate 入口、资源边界和常用方法在 docs.rs 上可导航。

## API Coverage Contract

`Phase 6` 要建立两份对照材料：

1. 给人看的 `docs/api-coverage.md`
2. 给工具读的 `tools/api-coverage/market-data-api.json`

这两份材料都必须明确区分：

- mirror layer：官方 HTTP endpoint 的一一对应层
- convenience layer：只额外补 `*_all` / `*_stream`

Coverage contract 必须能回答：

- 官方有哪些 Market Data HTTP endpoint 在当前范围内
- 本库 mirror layer 对应到哪个 Rust 方法
- convenience layer 补了哪些方法
- 哪些官方资源域当前明确不在范围内

## API Sync Skill

需要新增一个内部 skill，供未来 Codex 会话执行官方 API 一致性检查。

建议路径：

- `.agents/skills/alpaca-market-data-sync/SKILL.md`

必要职责：

- 读取官方 Market Data OpenAPI 或官方文档
- 读取本地 `tools/api-coverage/market-data-api.json`
- 生成一致性审计结果
- 明确区分 mirror drift 与 convenience-only additions
- 如果发现 mirror layer 漂移，先建议修正 mirror layer，再检查 convenience layer 是否仍兼容

这一 skill 是工作辅助能力，不要求 `Phase 6` 里直接自动改完未来所有官方变化。

## Release Metadata and Guardrails

`Phase 6` 需要把 `Cargo.toml` 收敛到标准发布形态，至少覆盖：

- `description`
- `readme`
- `repository`
- `documentation`
- 合法的 crates.io `keywords`
- 合法的 crates.io `categories`
- package boundary 的 `exclude` / `include`

如果当前仍没有明确 license 决策，则这会成为 `Phase 6` 执行中的一个显式 blocker；在真正准备 dry-run 前，需要用户明确 license 方案，不能擅自替用户做法律选择。

此外，发布前 guardrails 还应包括最小 CI 校验，至少覆盖：

- `cargo fmt --check`
- `cargo test`
- examples 编译
- `cargo doc`
- `cargo package`

CI 在这一 phase 的职责是帮助防回归，不承担自动发布。

## Verification Contract

`Phase 6` 完成前，至少要形成以下可重复执行的验证闭环：

- `cargo fmt --check`
- `cargo test`
- 必要的 live tests
- `cargo check --examples`
- `cargo test --doc` 或等效 rustdoc 编译校验
- `cargo doc --no-deps`
- `cargo bench --no-run`
- `cargo package`
- `cargo publish --dry-run`

另外需要核对：

- package 产物不包含 `docs/superpowers/`
- package 产物不包含 `memory/`
- README、public docs、examples、rustdoc 和本地公开 API 一致

## Success Criteria

当以下条件全部成立时，`Phase 6` 才算完成：

- 对外 README 和 `docs/` 文档形成英文公开面
- examples 和 rustdoc 足以支撑 `docs.rs` 可读性
- API coverage contract 和 sync skill 已落地
- package metadata 足以支撑 crates.io 发布准备
- internal docs 仍可继续驱动工作，但不会进入 published crate
- 预发布验证命令全部通过
- 本 phase 结束时仍然不自动发布 crates.io
