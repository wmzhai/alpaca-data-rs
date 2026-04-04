# 默认工作流

## 设计阶段默认顺序

1. 先更新 `README.md` 中的最终设计方案。
2. 如果设计约束发生变化，同步更新 `AGENTS.md`。
3. 再同步更新 `memory/` 中受影响的文档。
4. 为当前 phase 写出对应的 spec / plan 文档。
5. 最后检查是否有过期事实需要删除。
6. 在开始该 phase 的代码开发前，停下来等待用户确认设计。

## 代码落地阶段默认顺序

1. phase 设计确认后，再开始该 phase 的代码开发。
2. 先补 `Cargo.toml` 和 `src/` 骨架。
3. 优先建立 `Client`、资源模块和 transport 层的最小可运行结构。
4. 每落一层真实代码，就同步更新 `memory/core/system-map.md`。
5. 新增领域入口后，给对应领域补 memory 文档。
6. 每完成一个明确 task，先做代码和文档对齐检查，再准备版本提交。
7. 同一 phase 内的多个 task 默认连续执行，不在 task 之间逐一等待用户确认；只有遇到 blocker、官方事实不确定或需求冲突时才停下。

## 测试默认顺序

1. 先确认该场景是否能用真实 Alpaca API 验证。
2. 如果能，就直接写真实 API 集成测试；默认用环境变量开关控制，例如 `ALPACA_LIVE_TESTS=1`。
3. 只有真实 API 难以稳定制造异常时，才补 mock 故障注入测试。
4. benchmark 先看真实 API 端到端，再看本地 micro-benchmark。
5. 每个 phase 收尾时，必须至少跑完整的格式检查、默认测试、该 phase 的 live tests，以及对应 benchmark 验证。
6. 新资源的 benchmark baseline 落地后，先用 `cargo bench --bench <name> --no-run` 做编译验证，再决定是否跑完整采样。
7. 当前已落地的本地 benchmark target 包括 `shared_core`、`stocks`、`options`、`crypto`；phase 收尾时至少验证本 phase 对应 target 的编译链路。

## 文档更新要求

- 结构变化后，优先更新 `AGENTS.md` 和 `memory/`。
- 新版本提交前，必须同步更新 `CHANGELOG.md`。
- 每个 phase 完成后，必须同步更新 `README.md`、`AGENTS.md`、`memory/`、相关 plan/spec 文档和 `CHANGELOG.md`。
- 提交前必须全面检查代码、测试和所有受影响文档是否对齐；如有偏差，先修正事实，再提交。
- 删除过期路线，不保留旧说明占位。
- 没有真实代码事实时，明确写为“当前约定”或“待落地”。
- phase 收尾时，先完成验证、文档对齐、MINOR 版本升级和 phase 收尾提交，再在合并 `main` / 推送 / 删分支前停下来等待用户确认。
- phase 收尾时，最终版本提交必须先在开发分支上整理完成；不要把任何文档补丁或状态修正留到合并 `main` 之后再补。

## commit 默认要求

- 自动生成 commit message 时，标题必须使用英文 Conventional Commits 风格：`<type>: <summary>`。
- 推荐的 `type` 使用 `feat`、`fix`、`chore`、`refactor`、`docs`。
- 如有必要，在 body 中补一小段英文说明这次提交实际包含的内容。
- 每完成一个明确 task，默认做一次带版本号更新的提交，不长时间堆积多个已完成 task。
- task 级版本提交标题使用：`<type>: <summary> (vX.Y.Z)`。
- 提交前先递增版本号，并同步更新 `CHANGELOG.md` 与所有受影响文档。
- 每次新版本提交前，先更新 `CHANGELOG.md`，记录本版本的各种新变化，不只写结构变化。
- 如果是最终带 `CHANGELOG` 的发版提交，标题使用 `chore: bump version and changelog (vX.Y.Z)`。
- 每个 phase 收尾时，自动执行一次 MINOR 版本升级（`X.Y.Z -> X.(Y+1).0`）并完成 phase 收尾提交；合并到 `main`、推送远端并删除当前开发分支这一步需要在用户确认后再执行。
- phase 合并时只允许 `git merge --ff-only`；目标是让 phase 最终版本提交本身直接出现在 `main` 上，不再额外生成 merge commit。
