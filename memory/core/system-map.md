# 系统地图

## 当前仓库结构

当前仓库已完成 `Phase 1: Shared Core`、`Phase 2: Stocks`、`Phase 3: Options`、`Phase 4: Crypto` 与 `Phase 5: News + Corporate Actions`；`stocks`、`options`、`crypto`、`news` 与 `corporate_actions` 现在已组成五个完整资源模板模块。当前分支正在执行 `Phase 6: Release Hardening`，核心文件和目录如下：

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
- `src/transport/endpoint.rs`：共享 endpoint 路由定义，当前已打通完整 `crypto` 路由（historical + latest + snapshots）、完整 `stocks` 路由、完整 `options` 路由（historical + latest + snapshots/chain + metadata），以及 `news` 与 `corporate_actions` 的 list 路由
- `src/transport/http.rs`：共享 async HTTP JSON transport，当前已具备 timeout、status/error mapping 和 retry-after 解析
- `src/transport/retry.rs`：共享最小重试策略
- `src/transport/rate_limit.rs`：共享最小并发限制器
- `src/transport/pagination.rs`：共享分页 trait 与 helper，当前已提供 `collect_all` 和 `stream_pages`，并会在服务端重复返回同一个 `next_page_token` 时立即返回 `Error::Pagination`
- `src/stocks/`：第一个开始真实实现的资源域，当前已包含 `bars` / `quotes` / `trades` 历史 batch + single request、typed response/model、query 序列化、client fetcher、batch + single historical 的 `*_all` / `*_stream`，以及 latest / snapshot / metadata 的 batch + single 端点
- `src/options/`：第二个开始真实实现的资源域，当前已包含 historical batch `bars` / `trades`、latest `latest_quotes` / `latest_trades`、snapshot family `snapshots` / `chain`、metadata `exchange_codes` 的 request/response/model、query 序列化、client fetcher，以及全部 `*_all` / `*_stream` 便利层；`Snapshot` 现已包含 `greeks` 与 `impliedVolatility`
- `src/crypto/`：当前已包含完整 crypto HTTP mirror 实现：historical `bars` / `quotes` / `trades` request/response/model、`Loc`/`TimeFrame` 官方字符串建模、public no-auth client fetcher、historical `*_all` / `*_stream` 便利层、latest `latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks`、`snapshots`，以及 `OrderbookLevel` / `Snapshot` typed model
- `src/news/`：当前已包含 `list` / `list_all` / `list_stream` 的 request/response/model/client 实现，保留官方 `/v1beta1/news` wrapper 形状，并已接入真实 API happy-path
- `src/corporate_actions/`：当前已包含 `list` / `list_all` / `list_stream` 的 request/response/model/client 实现，保留官方 `/v1/corporate-actions` bucketed wrapper，并按 documented family 输出 typed model，同时为 `contract_adjustments` / `partial_calls` 与未来未知 bucket 保留 fallback
- `tests/public_api.rs`：公开 API 形状的编译期使用测试
- `tests/client_builder.rs`：`ClientBuilder` 运行时配置与认证校验测试
- `tests/mock_transport_errors.rs`：共享 transport 的异常路径测试
- `tests/live_crypto_latest_quotes_smoke.rs`：真实 Alpaca API 下的 crypto latest quotes smoke test
- `tests/live_crypto_historical.rs`：真实 Alpaca API 下的 `crypto.bars` / `crypto.quotes` / `crypto.trades` 以及 `bars_all` happy-path baseline
- `tests/live_crypto_latest.rs`：真实 Alpaca API 下的 `crypto.latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks` happy-path baseline
- `tests/live_crypto_snapshots.rs`：真实 Alpaca API 下的 `crypto.snapshots` happy-path baseline
- `tests/live_news.rs`：真实 Alpaca API 下的 `news.list` / `list_all` / `list_stream` happy-path baseline
- `tests/live_corporate_actions.rs`：真实 Alpaca API 下的 `corporate_actions.list` / `list_all` / `list_stream` happy-path baseline
- `tests/live_stocks_batch_historical.rs`：真实 Alpaca API 下的 `stocks.bars` / `stocks.quotes` / `stocks.trades` 以及 batch `*_all` / `*_stream` happy-path baseline
- `tests/live_options_historical.rs`：真实 Alpaca API 下的 `options.bars` / `options.trades` 以及 batch `bars_all` / `trades_stream` happy-path baseline
- `tests/live_options_latest_metadata.rs`：真实 Alpaca API 下的 `options.latest_quotes` / `options.latest_trades` / `options.exchange_codes` happy-path baseline
- `tests/live_options_snapshots_chain.rs`：真实 Alpaca API 下的 `options.snapshots` / `options.chain` 以及 `snapshots_all` / `snapshots_stream` / `chain_all` / `chain_stream` happy-path baseline
- `tests/live_stocks_single_historical.rs`：真实 Alpaca API 下的 `stocks.*_single` 与 `*_single_all` / `*_single_stream` happy-path baseline
- `tests/live_stocks_latest_snapshot.rs`：真实 Alpaca API 下的 `stocks.latest_*` 与 `stocks.snapshot*` happy-path baseline
- `tests/live_stocks_metadata.rs`：真实 Alpaca API 下的 `stocks.condition_codes` 与 `stocks.exchange_codes` happy-path baseline
- `tests/mock_stocks_errors.rs`：`stocks` single historical 的异常 JSON 与分页一致性故障测试
- `tests/mock_options_errors.rs`：`options` snapshots / chain 的异常 JSON 与分页一致性故障测试
- `tests/mock_crypto_errors.rs`：`crypto` snapshots / latest orderbooks 的异常 JSON -> `Error::Deserialize` 故障测试
- `tests/mock_news_corporate_actions_errors.rs`：`news` / `corporate_actions` 的异常 JSON、分页 merge 与重复 `next_page_token` 回归测试
- `benches/shared_core.rs`：本地 `criterion` benchmark baseline，当前覆盖 `crypto.latest_quotes` 共享通路
- `benches/stocks.rs`：本地 `criterion` benchmark baseline，当前覆盖 `stocks.latest_quote` 的本地 hot path
- `benches/options.rs`：本地 `criterion` benchmark baseline，当前覆盖 `options.chain` 的本地 hot path
- `benches/crypto.rs`：本地 `criterion` benchmark baseline，当前覆盖 `crypto.snapshots` 的本地 hot path
- `benches/news_corporate_actions.rs`：本地 `criterion` benchmark baseline，当前覆盖 `news.list` 与 `corporate_actions.list` 的本地 hot path
- `examples/`：`Phase 6 / Task 2` 新增的对外 runnable examples 目录，当前包含 `client_builder`、`stocks_latest_bar`、`stocks_bars_all`、`options_chain`、`crypto_latest_quotes`、`news_list` 与 `corporate_actions_list`
- `docs/`：`Phase 6` 起承载对外公开英文文档的主目录
- `tools/api-coverage/`：`Phase 6 / Task 3` 新增的 API coverage 工具目录，当前包含对外 coverage contract 对应的机器清单 `market-data-api.json`
- `.agents/skills/alpaca-market-data-sync/`：`Phase 6 / Task 4` 新增的内部 API parity 审计 skill 目录，当前用于 future sessions 对照官方 OpenAPI / reference 与本地 coverage manifest 做 drift audit
- `memory/`：项目导航、约束和后续扩展落点；在 `Phase 6` 继续保留在 git 中驱动工作，`Phase 7` 才会处理其 git 清理

## 当前还没有的结构

以下结构目前仍未落地，属于后续代码实现阶段的预期目录：

- 按资源域拆分的 `tests/live/` 与 `tests/mock/` 子目录（当前 live/mock 测试仍位于 `tests/` 根下）

## 预期的代码分层

根据当前设计约定，后续代码会按这些层次展开：

- 根入口：`alpaca_data::Client`
- 资源模块：`stocks`、`options`、`crypto`、`news`、`corporate_actions`
- 底层镜像层：忠实映射官方 HTTP API
- 上层便利层：`*_all`、`*_stream`
- transport 层：统一处理 HTTP、重试、分页和限流

## 当前事实边界

- 现在已经存在的是“共享基础层 + 部分真实资源实现”，还不是完整 API 实现。
- 当前真正落地的真实能力已覆盖共享层、完整 `crypto` 模块、完整 `stocks` 模块、完整 `options` 模块与其对应 convenience 层，以及 `news` 与 `corporate_actions` 的 list/filter/pagination 资源域。
- 当前五个资源模板都已完成 phase 级收尾，`Phase 5` 已在 `v0.5.0` 落到 `main`。
- 当前主线是 `Phase 6: Release Hardening`：补对外英文文档、examples、rustdoc、API coverage、API sync skill、package metadata、package boundary 与预发布验证。
- `docs/superpowers/` 与 `memory/` 在 `Phase 6` 继续保留在 git 中；它们的 git 移除、`.gitignore` 收口和最终发布决策属于 `Phase 7: Release`。
- 后续代码真正补齐后，这份文档需要继续从“部分真实目录图”更新为更细的完整实现图。
