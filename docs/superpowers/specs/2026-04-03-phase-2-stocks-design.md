# Phase 2 Stocks Design

**Date:** 2026-04-03
**Status:** Approved for planning
**Target Phase:** `Phase 2: Stocks`

## Goal

将 `stocks` 做成第一个完整资源域，实现 Alpaca Market Data API 官方 `stocks` HTTP endpoints 的完整对应，并作为后续 `options`、`crypto`、`news`、`corporate_actions` 的标准模板。

## Scope

本 phase 覆盖官方 `stocks` HTTP API 的以下两类 endpoint：

### Batch endpoints

- historical: `bars`、`quotes`、`trades`
- latest: `latest_bars`、`latest_quotes`、`latest_trades`
- snapshot: `snapshots`
- metadata: `condition_codes`、`exchange_codes`

### Single-symbol endpoints

- historical: `bars_single`、`quotes_single`、`trades_single`
- latest: `latest_bar`、`latest_quote`、`latest_trade`
- snapshot: `snapshot`

### Convenience endpoints

只为可分页的 historical endpoints 提供：

- batch: `bars_all`、`bars_stream`、`quotes_all`、`quotes_stream`、`trades_all`、`trades_stream`
- single-symbol: `bars_single_all`、`bars_single_stream`、`quotes_single_all`、`quotes_single_stream`、`trades_single_all`、`trades_single_stream`

## Non-Goals

- 不在本 phase 内实现 `options`、`crypto`、`news`、`corporate_actions`
- 不引入 websocket / SSE
- 不引入缓存、预聚合或官方 API 之外的派生字段
- 不为了“简洁”而合并 batch / single 官方端点

## Canonical Standard

唯一语义标准是 Alpaca 官方 Market Data HTTP API。SDK 命名和结构在 Rust 风格下做最小必要适配，但不改写官方 endpoint 的核心词义。

官方参考文档：

- Historical bars (multi): https://docs.alpaca.markets/reference/stockbars
- Historical bars (single): https://docs.alpaca.markets/reference/stockbarsingle
- Historical quotes (multi): https://docs.alpaca.markets/reference/stockquotes-1
- Historical quotes (single): https://docs.alpaca.markets/v1.3/reference/stockquotesingle
- Historical trades (multi): https://docs.alpaca.markets/reference/stocktrades-1
- Latest bars (multi): https://docs.alpaca.markets/reference/stocklatestbars
- Latest bar (single): https://docs.alpaca.markets/v1.3/reference/stocklatestbarsingle
- Latest quote (single): https://docs.alpaca.markets/reference/stocklatestquotesingle-1
- Latest trade (single): https://docs.alpaca.markets/reference/stocklatesttradesingle-1
- Snapshots (multi): https://docs.alpaca.markets/v1.3/reference/stocksnapshots
- Snapshot (single): https://docs.alpaca.markets/reference/stocksnapshotsingle
- Condition codes: https://docs.alpaca.markets/reference/stockmetaconditions-1
- Exchange codes: https://docs.alpaca.markets/reference/stockmetaexchanges-1
- Historical stock data overview: https://docs.alpaca.markets/docs/historical-stock-data-1

## Naming Decision

### Chosen approach

采用方案 A：官方词根保留，single 用最小后缀或单数语义区分。

### Public methods

Batch methods:

- `bars`
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `condition_codes`
- `exchange_codes`

Single-symbol methods:

- `bars_single`
- `quotes_single`
- `trades_single`
- `latest_bar`
- `latest_quote`
- `latest_trade`
- `snapshot`

Convenience methods:

- `bars_all` / `bars_stream`
- `quotes_all` / `quotes_stream`
- `trades_all` / `trades_stream`
- `bars_single_all` / `bars_single_stream`
- `quotes_single_all` / `quotes_single_stream`
- `trades_single_all` / `trades_single_stream`

### Why this approach

- historical single endpoint 在官方 path 中仍然是复数资源，例如 `/v2/stocks/{symbol}/bars`，因此 `bars_single` 比 `bar_single` 更准确
- latest / snapshot 的 single endpoint 语义本身是单数，因此 `latest_bar`、`latest_quote`、`latest_trade`、`snapshot` 最自然
- 不需要引入额外 facade，例如 `client.stocks().symbol("AAPL")`
- 能在 Rust API 中清晰区分 batch 和 single，又保留官方词根

## Request / Response Type Design

### Request types

Batch historical:

- `BarsRequest`
- `QuotesRequest`
- `TradesRequest`

Single historical:

- `BarsSingleRequest`
- `QuotesSingleRequest`
- `TradesSingleRequest`

Batch latest:

- `LatestBarsRequest`
- `LatestQuotesRequest`
- `LatestTradesRequest`

Single latest:

- `LatestBarRequest`
- `LatestQuoteRequest`
- `LatestTradeRequest`

Batch / single snapshot:

- `SnapshotsRequest`
- `SnapshotRequest`

Metadata:

- `ConditionCodesRequest`
- `ExchangeCodesRequest` is not needed because the current official endpoint has no path or query input

### Response types

Batch historical:

- `BarsResponse`
- `QuotesResponse`
- `TradesResponse`

Single historical:

- `BarsSingleResponse`
- `QuotesSingleResponse`
- `TradesSingleResponse`

Batch latest:

- `LatestBarsResponse`
- `LatestQuotesResponse`
- `LatestTradesResponse`

Single latest:

- `LatestBarResponse`
- `LatestQuoteResponse`
- `LatestTradeResponse`

Batch / single snapshot:

- `SnapshotsResponse`
- `SnapshotResponse`

Metadata:

- `ConditionCodesResponse`
- `ExchangeCodesResponse`

### Field rules

- 请求字段名与响应字段名必须直接使用官方原词
- 一个字母都不能改
- 遇到 Rust 关键字时，只做最小适配，例如 `r#as`
- 不允许新增官方 HTTP API 中不存在的自定义字段
- 时间字段、价格字段、数量字段和 code 字段尽量使用 typed model，而不是 `serde_json::Value`

## File Structure

Phase 2 保持当前标准 Rust 资源布局：

- `src/stocks/mod.rs`
- `src/stocks/client.rs`
- `src/stocks/request.rs`
- `src/stocks/response.rs`
- `src/stocks/model.rs`
- `src/stocks/enums.rs`

职责划分：

- `mod.rs`: 导出公开类型
- `client.rs`: 公开方法、endpoint 路由、query 组装、transport 调用、分页 helper 集成
- `request.rs`: 所有 request structs
- `response.rs`: 所有 response wrapper structs
- `model.rs`: `Bar`、`Quote`、`Trade`、`Snapshot`、`ConditionCode`、`ExchangeCode` 等实际模型
- `enums.rs`: `Adjustment`、`DataFeed`、`Sort`、`TimeFrame`、`Currency` 等枚举及其官方字符串映射

如果 `model.rs` 在完整字段落地后过大，可以在同一 phase 内拆成私有子文件，但公开导出路径不变。

## Internal Architecture

- 对外同时保留 batch 和 single 两套方法
- 对内尽量复用 shared core：
  - `QueryWriter`
  - `Endpoint`
  - `HttpClient`
  - `PaginatedRequest`
  - `PaginatedResponse`
  - `collect_all`
  - `stream_pages`
- endpoint path、query 序列化和 response decode 应尽量共用，避免在 `stocks` 内复制平行实现

## Pagination Strategy

### Historical batch endpoints

以下方法支持完整分页：

- `bars` / `bars_all` / `bars_stream`
- `quotes` / `quotes_all` / `quotes_stream`
- `trades` / `trades_all` / `trades_stream`

### Historical single-symbol endpoints

以下方法也支持完整分页：

- `bars_single` / `bars_single_all` / `bars_single_stream`
- `quotes_single` / `quotes_single_all` / `quotes_single_stream`
- `trades_single` / `trades_single_all` / `trades_single_stream`

### Non-paginated endpoints

以下方法不提供 `*_all` / `*_stream`：

- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `latest_bar`
- `latest_quote`
- `latest_trade`
- `snapshots`
- `snapshot`
- `condition_codes`
- `exchange_codes`

### Aggregation rules

- `*_all` 返回同名 `Response`
- 聚合结束后 `next_page_token = None`
- batch historical 聚合时必须正确合并多 symbol 的序列
- single historical 聚合时必须正确合并同一 symbol 的序列
- `*_stream` 继续按页返回 `Response`，不拍平成单条 item

## Model Completeness Strategy

本 phase 不采用“只做最小 happy-path 字段集”的策略。`stocks` 作为模板模块，应尽量一次补齐官方字段。

优先级如下：

1. 先完整对齐 request 参数
2. 再完整对齐 response wrapper
3. 再尽量补齐 `Bar` / `Quote` / `Trade` / `Snapshot` / metadata models 的官方字段
4. 在真实 API 响应和文档定义不完全一致时，以官方真实 HTTP 响应为准，并在实现文档中记录差异

## Testing Strategy

### Real API tests are primary

正常成功路径测试优先使用真实 Alpaca API，不使用 mock 得出主正确性结论。

Phase 2 至少覆盖：

- batch historical happy path
- single historical happy path
- batch latest happy path
- single latest happy path
- batch snapshot happy path
- single snapshot happy path
- metadata happy path
- batch historical pagination happy path
- single historical pagination happy path

### Mock tests are limited to exceptional paths

mock 只用于以下场景：

- 429 / `Retry-After`
- 5xx retry
- malformed JSON
- timeout
- 无效分页 token 或意外响应形状

### Environment

真实 API tests 继续沿用：

- `ALPACA_LIVE_TESTS=1`
- `APCA_API_KEY_ID`
- `APCA_API_SECRET_KEY`
- `APCA_API_DATA_URL`

## Benchmark Strategy

Phase 2 需要新增 `stocks` benchmark baseline，至少包括：

- 一个 historical batch endpoint 的本地 micro-benchmark
- 一个 historical single endpoint 的本地 micro-benchmark
- 一个 latest 或 snapshot endpoint 的本地 micro-benchmark

benchmark 仍以本地 micro-benchmark 为主，不把 mock 结果当作真实性能结论。

## Documentation Requirements

Phase 2 实施时需要同步更新：

- `README.md`
- `AGENTS.md`（如高优先级规则发生变化）
- `memory/README.md`
- `memory/api/README.md`
- `memory/core/system-map.md`
- `memory/core/workflows.md`
- `CHANGELOG.md`
- `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 后续新增的 Phase 2 详细 implementation plan

## Phase Exit Criteria

当且仅当以下条件全部成立时，`Phase 2: Stocks` 视为完成：

- `stocks` batch / single 官方 endpoint 全部具备真实 HTTP 行为
- 对外字段继续严格保持官方原词
- batch / single historical 的 `*_all` / `*_stream` 都已可用
- 正常成功路径测试以真实 API 为主并通过
- exception-path mock tests 通过
- `stocks` benchmark baseline 建立完成
- phase 收尾文档、版本号、`CHANGELOG.md` 与实现事实对齐

## Implementation Breakdown Guidance

Phase 2 应拆成多个 task 顺序完成，而不是一次性堆成一个超大提交。建议的实现顺序：

1. endpoint routing + request/query completeness
2. historical batch (`bars` / `quotes` / `trades`) + real tests
3. historical single + pagination convenience + real tests
4. latest + snapshot batch/single + real tests
5. metadata + model completeness收尾
6. benchmark / docs / phase completion

这个拆分只作为 planning 输入，真正的任务颗粒度以后续 implementation plan 为准。
