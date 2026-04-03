# AGENTS.md

## 先读哪里

新会话默认按这个顺序读取：

1. `README.md`
2. `memory/README.md`
3. `memory/core/invariants.md`
4. 与当前任务最相关的 `memory/` 文档

## 项目当前状态

- 这是一个新的 Rust crate 设计仓库，当前以设计文档为主，尚未开始正式代码落地。
- 项目目标是构建一个高性能的 Alpaca Market Data API HTTP Rust 客户端。
- 当前范围只包含 Market Data API，不包含 Trading API、Broker API、WebSocket / SSE。
- crates.io 包名约定为 `alpaca-data`，代码导入路径约定为 `alpaca_data`。

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
- 如果需要补充说明，优先在 commit body 里用一小段英文说明本次提交包含什么。
- 每次新版本提交都必须同步更新 `CHANGELOG.md`。
- `CHANGELOG.md` 不只记录结构变化，也要记录各种新变化，包括对外接口、文档、测试、工程配置和内部实现上的重要变化。
- 如果是最终那个带 `CHANGELOG` 的发版提交，标题格式使用 `chore: bump version and changelog (vX.Y.Z)`。
- 版本号格式固定为三段：`MAJOR.MINOR.PATCH`。
- 没有真实代码事实时，只能写“当前约定”或“设计约束”，不能伪装成已实现事实。

## 当前默认工作方式

- 修改设计约束时，先同步 `README.md`，再同步 `AGENTS.md` 和 `memory/`。
- 新增代码骨架后，优先补 `memory/core/system-map.md` 和对应领域文档。
- 结构变化后，要及时更新记忆文档，不保留过期路线。

## 关键目录边界

- `README.md`：项目最终设计方案与公开 API 契约。
- `CHANGELOG.md`：每个版本提交的变化记录，版本提交前必须同步更新。
- `AGENTS.md`：新会话必须先遵守的高优先级规则。
- `memory/`：项目导航、约束、工作流和领域落点。

## 当前禁区

- 不要把 Trading API 或 Broker API 混入本库。
- 不要为了 SDK 自己的“优雅”而偏离官方 HTTP API 词汇和结构。
- 不要在正常成功路径测试里使用 mock。
- 未经明确要求，不要做 commit。
