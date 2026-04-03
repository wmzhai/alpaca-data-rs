# 系统地图

## 当前仓库结构

当前仓库已完成 `Phase 1: Shared Core`，并已落地到 `Phase 2 / Task 4` 的 `stocks` 历史 + latest + snapshot endpoint，核心文件和目录如下：

- `README.md`：最终设计方案与公开 API 契约
- `CHANGELOG.md`：版本提交的变化记录
- `AGENTS.md`：新会话必须先遵守的高优先级规则
- `Cargo.toml`：crate manifest，包名约定为 `alpaca-data`
- `.gitignore`：Rust 库仓库的最小忽略规则
- `src/lib.rs`：根模块导出，公开 `Client`、`Error` 和五个资源模块
- `src/client.rs`：`Client`、`ClientBuilder`、共享 `Inner`，以及最小运行时配置（`base_url`、`timeout`、`max_retries`、`max_in_flight`）
- `src/auth.rs`：认证配置与 `api_key` / `secret_key` 成对校验
- `src/error.rs`：顶层 `Error` 类型，当前已包含 `InvalidConfiguration`、`RateLimited`、`HttpStatus`、`Deserialize` 等共享错误变体
- `src/common/enums.rs`：共享 `Sort` 与 `Currency` 基础类型，当前已经按官方 Market Data 参数字符串建模
- `src/common/query.rs`：共享 query 参数构造器，当前已支持 CSV 参数和可选参数写入
- `src/transport/endpoint.rs`：共享 endpoint 路由定义，当前已打通 `crypto.latest_quotes` 与 `stocks` batch / single skeleton 路径映射
- `src/transport/http.rs`：共享 async HTTP JSON transport，当前已具备 timeout、status/error mapping 和 retry-after 解析
- `src/transport/retry.rs`：共享最小重试策略
- `src/transport/rate_limit.rs`：共享最小并发限制器
- `src/transport/pagination.rs`：共享分页 trait 与 helper，当前已提供 `collect_all` 和 `stream_pages`
- `src/stocks/`：第一个开始真实实现的资源域，当前已包含 `bars` / `quotes` / `trades` 历史 batch + single request、typed response/model、query 序列化、client fetcher、single historical 的 `*_single_all` / `*_single_stream`，以及 latest / snapshot 的 batch + single 端点
- `src/options/`、`src/crypto/`、`src/news/`、`src/corporate_actions/`：其余资源域当前仍以最小模块骨架为主
- `tests/public_api.rs`：公开 API 形状的编译期使用测试
- `tests/client_builder.rs`：`ClientBuilder` 运行时配置与认证校验测试
- `tests/mock_transport_errors.rs`：共享 transport 的异常路径测试
- `tests/live_crypto_latest_quotes_smoke.rs`：真实 Alpaca API 下的 crypto latest quotes smoke test
- `tests/live_stocks_batch_historical.rs`：真实 Alpaca API 下的 `stocks.bars` / `stocks.quotes` / `stocks.trades` happy-path baseline
- `tests/live_stocks_single_historical.rs`：真实 Alpaca API 下的 `stocks.*_single` 与 `*_single_all` / `*_single_stream` happy-path baseline
- `tests/live_stocks_latest_snapshot.rs`：真实 Alpaca API 下的 `stocks.latest_*` 与 `stocks.snapshot*` happy-path baseline
- `tests/mock_stocks_errors.rs`：`stocks` single historical 的异常 JSON 与分页一致性故障测试
- `benches/shared_core.rs`：本地 `criterion` benchmark baseline，当前覆盖 `crypto.latest_quotes` 共享通路
- `memory/`：项目导航、约束和后续扩展落点

## 当前还没有的结构

以下结构目前仍未落地，属于后续代码实现阶段的预期目录：

- 按资源域拆分的 `tests/live/` 与 `tests/mock/` 子目录（当前 live/mock 测试仍位于 `tests/` 根下）
- `stocks` 的 metadata endpoint 实现
- `options`、`news`、`corporate_actions` 的真实 HTTP endpoint 实现
- 除 `crypto.latest_quotes` 与 `stocks` 历史 + latest + snapshot 之外的完整 Market Data 请求/响应字段模型
- 资源级 benchmark 基线

## 预期的代码分层

根据当前设计约定，后续代码会按这些层次展开：

- 根入口：`alpaca_data::Client`
- 资源模块：`stocks`、`options`、`crypto`、`news`、`corporate_actions`
- 底层镜像层：忠实映射官方 HTTP API
- 上层便利层：`*_all`、`*_stream`
- transport 层：统一处理 HTTP、重试、分页和限流

## 当前事实边界

- 现在已经存在的是“共享基础层 + 部分真实资源实现”，还不是完整 API 实现。
- 当前真正落地的真实能力已覆盖共享层、`crypto.latest_quotes`，以及 `stocks` 历史 batch + single、latest、snapshot 的公开 HTTP 行为。
- 当前 `stocks` 的 metadata 和其余资源域方法仍以占位壳为主，不能当成真实 HTTP 逻辑已完成。
- 后续代码真正补齐后，这份文档需要继续从“部分真实目录图”更新为更细的完整实现图。
