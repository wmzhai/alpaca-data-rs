# memory/README.md

`memory/` 用来帮助新 session 快速定位项目事实、约束和后续落点。

## 热路径

- 想知道项目做什么：先读 `README.md`
- 想知道每个版本具体改了什么：先读 `CHANGELOG.md`
- 想知道必须遵守什么：先读 `memory/core/invariants.md`
- 想知道仓库当前有哪些目录和它们负责什么：读 `memory/core/system-map.md`
- 想知道默认工作顺序和验证基线：读 `memory/core/workflows.md`
- 想知道公开 API 设计和命名约束：读 `memory/api/README.md`

## 冷路径

- 遇到“该先看哪份文档”的问题：读 `memory/core/symptom-routing.md`
- 代码真正开始落地后，再按实际模块补充新的领域文档

## 当前状态说明

- 这是一个新项目，当前以设计文档为主。
- 当前已经落下最小 Rust crate 骨架：`Cargo.toml`、`src/`、`tests/`、`.gitignore`。
- `memory/` 里的内容目前主要记录设计约束、目录约定和最小代码骨架的默认导航。
- 后续随着真实 HTTP transport、请求/响应模型和资源实现补齐，需要继续把这里更新成真实代码入口。
