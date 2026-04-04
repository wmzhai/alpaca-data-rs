# AGENTS.md

## 先读哪里

新会话默认按这个顺序读取：

1. `README.md`
2. `memory/README.md`
3. `memory/core/invariants.md`
4. 与当前任务最相关的 `memory/` 文档

## 项目当前状态

- 这是一个新的 Rust crate 仓库，当前已完成 `Phase 1: Shared Core`、`Phase 2: Stocks`、`Phase 3: Options`、`Phase 4: Crypto` 与 `Phase 5: News + Corporate Actions`。
- 项目目标是构建一个高性能的 Alpaca Market Data API HTTP Rust 客户端。
- 当前范围只包含 Market Data API，不包含 Trading API、Broker API、WebSocket / SSE。
- crates.io 包名约定为 `alpaca-data`，代码导入路径约定为 `alpaca_data`。
- 当前已经落地共享基础层，以及五个完整资源模板：`stocks` 的历史 batch / single、latest、snapshot、metadata 与历史 batch / single `*_all` / `*_stream` 便利层；`options` 的 historical `bars` / `trades`、latest `latest_quotes` / `latest_trades`、snapshot family `snapshots` / `chain`、metadata `exchange_codes` 与全部 `*_all` / `*_stream` 便利层；`crypto` 的 historical `bars` / `quotes` / `trades`、historical `*_all` / `*_stream`、latest `latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks`、`snapshots`；以及 `news` / `corporate_actions` 的 `list` / `list_all` / `list_stream`、happy-path、fault coverage 与 benchmark baseline。
- 当前 `Phase 5` 已在 `v0.5.0` 完成并落到 `main`；当前分支已把 `Phase 6: Release Hardening` 收敛到 `v0.6.1` 的 release-prepared 收尾版本候选，在得到用户确认前，不合并 `main`、不推送、也不删除分支；随后由 `Phase 7: Release` 负责 internal docs 的 git 清理与最终发布决策。

## 最高优先级规则

- 官方 Alpaca Market Data HTTP API 是唯一语义标准。
- 请求字段名和响应字段名必须直接使用官方原始单词，一个字母都不要改。
- 公开 API 的具体命名形式必须遵循标准 Rust 风格：模块小写、类型 PascalCase、字段和方法 `snake_case`。
- 公开根入口固定为 `alpaca_data::Client`。
- 公开资源入口固定为 `stocks()`、`options()`、`crypto()`、`news()`、`corporate_actions()`。
- 底层接口保持官方原始形态；上层只补 `*_all` 和 `*_stream` 这类便利封装。
- 正常、正确场景的测试严禁使用 mock；只要官方 API key 能覆盖，就必须优先使用真实 Alpaca API。
- mock 只允许用于 429、5xx、timeout、损坏响应等故障注入场景。
- 自动生成的 commit message 必须全英文，不能使用中文。
- commit message 必须使用 Conventional Commits 风格：`<type>: <summary>`。
- 当前允许并优先使用的 `type`：`feat`、`fix`、`chore`、`refactor`、`docs`。
- 如果需要使用子代理，模型固定只允许使用 `gpt-5.4`，不要使用其他模型。
- 如果没有经过审计并明确承诺的 MSRV，就不要在 `Cargo.toml` 里声明 `rust-version`。
- GitHub CI 使用浮动 `stable` 工具链，不要在 workflow 里硬编码旧 Rust 版本号。
- GitHub CI 只允许由用户推送 release tag 触发；普通提交、分支 push 和 PR 默认都不触发仓库 CI。
- 如果需要补充说明，优先在 commit body 里用一小段英文说明本次提交包含什么。
- 每完成一个明确的开发 task，都要做一次带版本号更新的提交，不能把多个已完成 task 长时间堆在工作区里不提交。
- 每个 task 完成后的提交前，都必须先同步版本号、`CHANGELOG.md` 和所有受影响文档，再进行提交。
- 每个 task 完成后的版本提交标题统一使用英文格式：`<type>: <summary> (vX.Y.Z)`。
- 开始 release work 或 API sync work 前，先运行 `.agents/skills/alpaca-market-data-sync/SKILL.md` 的审计流程，对照官方 OpenAPI / reference 与本地 `tools/api-coverage/market-data-api.json`；如果发现 mirror drift，必须先修 mirror layer，再重新验证 convenience layer。
- 每个 phase 开始时，必须先完成该 phase 的设计文档与实现计划文档，并停下来等待用户确认设计；未得到 phase 设计确认前，不得开始该 phase 的代码开发。
- 每个 phase 完成后，必须先全面跑完该 phase 的格式检查、单元测试、集成测试、所需 live tests 和 benchmark 验证，并对齐 `README.md`、`AGENTS.md`、`memory/`、相关 plan/spec 文档与 `CHANGELOG.md`。
- 每个 phase 完成后的收尾版本必须自动执行一次 MINOR 递增（`X.Y.Z -> X.(Y+1).0`）；在合并到 `main`、推送远端并删除当前开发分支之前，必须再停下来等待用户确认。
- 每个 phase 的最终版本提交必须直接成为落到 `main` 的那个 commit；不允许在 phase 最终版本提交之后，再额外生成一个只用于合并的 commit。
- phase 收尾合并默认只允许使用 `git merge --ff-only`；如果无法 fast-forward，必须先停下来处理分支差异，不能生成额外 merge commit。
- 每个 phase 一旦设计已获用户确认，默认一次性连续执行到该 phase 完成；除非遇到 blocker、官方 API 事实不确定、需求冲突或用户明确要求暂停，否则每个 task 完成后直接进入下一个 task，不再逐 task 询问用户。
- 每次提交前，都必须全面检查代码、测试、`README.md`、`AGENTS.md`、`memory/`、相关 plan/spec 文档与 `CHANGELOG.md` 是否彼此对齐；发现不一致时先直接修正，再提交。
- 每次新版本提交都必须同步更新 `CHANGELOG.md`。
- `CHANGELOG.md` 不只记录结构变化，也要记录各种新变化，包括对外接口、文档、测试、工程配置和内部实现上的重要变化。
- 如果是最终那个带 `CHANGELOG` 的发版提交，标题格式使用 `chore: bump version and changelog (vX.Y.Z)`。
- 版本号格式固定为三段：`MAJOR.MINOR.PATCH`。
- 没有真实代码事实时，只能写“当前约定”或“设计约束”，不能伪装成已实现事实。
- `Phase 6` 只做 release preparation，不移除 `docs/superpowers/` 或 `memory/`，也不自动发布 crates.io。
- `docs/superpowers/` 与 `memory/` 在 `Phase 6` 继续保留在 git 中驱动工作；如果需要保持发布产物干净，优先通过 package boundary 排除，而不是提前从 git 删除。
- `Phase 6` 的 published crate 应通过 package boundary 排除 `.agents/`、`.github/`、`AGENTS.md`、`docs/superpowers/` 与 `memory/`，而不是提前从 git 删除这些目录。
- `Phase 7` 才负责 `docs/superpowers/` / `memory/` 的 git 移除、`.gitignore` 收口，以及是否真正发布 crates.io 的最终决策。

## 当前默认工作方式

- 修改设计约束时，先同步 `README.md`，再同步 `AGENTS.md` 和 `memory/`。
- 新增代码骨架后，优先补 `memory/core/system-map.md` 和对应领域文档。
- 结构变化后，要及时更新记忆文档，不保留过期路线。
- 不要使用 `.worktrees/` 或其他 git worktree 工作目录；新任务直接创建并切换到普通 git 分支即可。
- 每个 task 完成后，默认同步检查并更新：`README.md`、`AGENTS.md`、`memory/`、相关 plan/spec 文档、`CHANGELOG.md`。
- 每个 task 完成后，默认先做版本号递增，再做带版本号的提交；除非用户明确要求暂不提交。
- 每个 phase 的标准节奏固定为：
  1. 先写并同步该 phase 的 spec / plan 文档
  2. 停下来等待用户确认 phase 设计
  3. 设计确认后，连续执行该 phase 的各个 task，中间不逐 task 询问
  4. phase 内所有 task 完成后，先做完整验证、文档对齐、MINOR 版本升级和 phase 收尾提交
  5. 在合并 `main`、推送远端和删除分支之前，再停下来等待用户确认
  6. 用户确认后，只允许用 fast-forward 方式把该 phase 的最终版本提交落到 `main`
- 提交前的对齐检查不是走形式；如果文档和代码不一致，要先把事实修正到一致，再执行验证和提交。
- 当前 release 主线拆成两段：
  1. `Phase 6: Release Hardening` 负责公开英文文档、examples、rustdoc、API coverage、API sync skill、package metadata、package boundary 与预发布验证
  2. `Phase 7: Release` 负责 internal docs 的 git 移除、`.gitignore` 收口和最终发布决策

## 关键目录边界

- `README.md`：项目最终设计方案与公开 API 契约。
- `CHANGELOG.md`：每个版本提交的变化记录，版本提交前必须同步更新。
- `AGENTS.md`：新会话必须先遵守的高优先级规则。
- `.agents/skills/`：仓库内的内部 agent workflow 入口；当前包含 Alpaca Market Data API sync 审计 skill。
- `memory/`：项目导航、约束、工作流和领域落点。

## 当前禁区

- 不要把 Trading API 或 Broker API 混入本库。
- 不要为了 SDK 自己的“优雅”而偏离官方 HTTP API 词汇和结构。
- 不要在正常成功路径测试里使用 mock。
