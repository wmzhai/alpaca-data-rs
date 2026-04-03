# 系统地图

## 当前仓库结构

当前仓库仍以设计约束为主，但已经开始落地共享基础层，核心文件和目录如下：

- `README.md`：最终设计方案与公开 API 契约
- `CHANGELOG.md`：版本提交的变化记录
- `AGENTS.md`：新会话必须先遵守的高优先级规则
- `Cargo.toml`：crate manifest，包名约定为 `alpaca-data`
- `.gitignore`：Rust 库仓库的最小忽略规则
- `src/lib.rs`：根模块导出，公开 `Client`、`Error` 和五个资源模块
- `src/client.rs`：`Client`、`ClientBuilder`、共享 `Inner`，以及最小运行时配置（`base_url`、`timeout`、`max_retries`、`max_in_flight`）
- `src/auth.rs`：认证配置与 `api_key` / `secret_key` 成对校验
- `src/error.rs`：顶层 `Error` 类型，当前已包含 `InvalidConfiguration`
- `src/common/query.rs`：共享 query 参数构造器，当前已支持 CSV 参数和可选参数写入
- `src/transport/endpoint.rs`：共享 endpoint 路由定义，当前已打通 crypto latest quotes 路径映射
- `src/stocks/`、`src/options/`、`src/crypto/`、`src/news/`、`src/corporate_actions/`：五个资源域的最小模块骨架
- `tests/public_api.rs`：公开 API 形状的编译期使用测试
- `tests/client_builder.rs`：`ClientBuilder` 运行时配置与认证校验测试
- `memory/`：项目导航、约束和后续扩展落点

## 当前还没有的结构

以下结构目前仍未落地，属于后续代码实现阶段的预期目录：

- benchmark 目录
- 真实 API 集成测试目录
- 基于真实 Alpaca HTTP 行为的 transport 实现
- 真实请求/响应字段的完整模型

## 预期的代码分层

根据当前设计约定，后续代码会按这些层次展开：

- 根入口：`alpaca_data::Client`
- 资源模块：`stocks`、`options`、`crypto`、`news`、`corporate_actions`
- 底层镜像层：忠实映射官方 HTTP API
- 上层便利层：`*_all`、`*_stream`
- transport 层：统一处理 HTTP、重试、分页和限流

## 当前事实边界

- 现在已经存在的是“最小骨架 + 部分共享基础层”，还不是完整 API 实现。
- 当前真正落地的共享层能力已覆盖 builder/runtime config、认证配对校验、配置错误建模、基础 query 序列化和最小 endpoint 路由。
- 当前各资源 client 的 endpoint 方法大多仍是占位壳，不能当成真实 HTTP 逻辑已完成。
- 后续代码真正补齐后，这份文档需要继续从“骨架图”更新为更细的真实目录图。
