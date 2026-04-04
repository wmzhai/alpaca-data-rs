# 稳定约束

## API 语义

- Alpaca 官方 Market Data HTTP API 是唯一语义标准。
- 本库只覆盖 Market Data API，不覆盖 Trading API 和 Broker API。
- 当前设计不包含 WebSocket / SSE。

## 命名与字段

- 请求字段名和响应字段名必须直接使用官方原始单词，一个字母都不能改。
- 只有在 Rust 关键字冲突时，才允许使用最小 Rust 适配，例如 `r#type`。
- 公开 API 的具体写法必须遵循标准 Rust 风格。
- 公开根入口固定为 `alpaca_data::Client`。
- 资源入口固定为 `stocks()`、`options()`、`crypto()`、`news()`、`corporate_actions()`。
- 类型名尽量短，不重复资源前缀，例如 `stocks::BarsRequest`。

## 接口层次

- 底层接口保持官方原始形态。
- 上层接口只补 `*_all` 和 `*_stream`。
- `*_all` 返回同名 `Response`，并在聚合完成后令 `next_page_token = None`。
- `*_stream` 按页返回同名 `Response`，不直接拍平成单条 item。

## 运行时与性能

- 只做 `async`，不做 `sync`。
- 正常热路径不使用 `serde_json::Value` 作为核心解析模型。
- 不做隐式缓存。
- 不在核心层默认做 Greeks / IV 补算。
- crates.io 包名约定为 `alpaca-data`，代码导入名为 `alpaca_data`。

## commit 约束

- 自动生成的 commit message 必须全英文，不能使用中文。
- commit 标题必须使用 Conventional Commits 风格：`<type>: <summary>`。
- 当前允许并优先使用的 `type`：`feat`、`fix`、`chore`、`refactor`、`docs`。
- 如果需要使用子代理，模型固定只允许使用 `gpt-5.4`，不要使用其他模型。
- 如需补充说明，优先在 commit body 中追加一小段英文说明本次提交包含什么。
- 每次提交前，都必须全面检查代码、测试、`README.md`、`AGENTS.md`、`memory/`、相关 plan/spec 文档与 `CHANGELOG.md` 是否彼此对齐；发现不一致时先修正，再提交。
- 每完成一个明确 task，默认做一次带版本号更新的提交，不长时间堆积多个已完成 task。
- task 级版本提交标题使用：`<type>: <summary> (vX.Y.Z)`。
- 每次新版本提交都必须同步更新 `CHANGELOG.md`。
- `CHANGELOG.md` 必须记录各种新变化，不只记录结构变化；至少覆盖对外接口、文档、测试、工程配置和内部实现上的重要变化。
- 最终那个带 `CHANGELOG` 的发版提交，标题格式固定为 `chore: bump version and changelog (vX.Y.Z)`。
- 每个 phase 开始时，必须先有对应的 spec / plan 文档，并在开始该 phase 的代码开发前得到用户确认。
- 每个 phase 一旦设计获批，phase 内各个 task 默认连续执行，不在 task 之间逐一停顿。
- 每个 phase 完成后，必须先跑完整验证、同步所有受影响文档、自动执行一次 MINOR 版本升级与 phase 收尾提交；在合并到 `main`、推送远端并删除当前开发分支前，必须再得到用户确认。
- 每个 phase 的最终版本提交必须就是最终落到 `main` 的那个 commit；不允许在 phase 发版提交之后，再额外补一个 merge commit。
- phase 合并到 `main` 时默认必须使用 fast-forward；如果 `git merge --ff-only` 无法成立，先停下来处理，再继续。
- 版本号格式固定为三段：`MAJOR.MINOR.PATCH`。

## 测试红线

- 正常、正确场景的测试严禁使用 mock。
- 只要官方 API key 能覆盖，就必须优先使用真实 Alpaca API。
- mock 只允许用于 429、5xx、timeout、连接中断、损坏响应、异常分页 token 等故障注入场景。
- 不允许用 mock 得出主正确性结论。
