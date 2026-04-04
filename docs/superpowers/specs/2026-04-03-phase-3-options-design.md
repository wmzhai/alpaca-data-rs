# Phase 3 Options Design

**Date:** 2026-04-03
**Status:** Approved for planning
**Target Phase:** `Phase 3: Options`

## Goal

将 `options` 做成继 `stocks` 之后的第二个完整资源域，忠实对应 Alpaca Market Data HTTP API 的 options 端点，并锁定 options 特有的 snapshot / chain / greeks 建模方式。

## Scope

本 phase 只覆盖官方 options HTTP API 中已经列入 roadmap 的端点：

- historical: `bars`、`trades`
- latest: `latest_quotes`、`latest_trades`
- snapshot family: `snapshots`、`chain`
- metadata: `exchange_codes`
- convenience: `bars_all` / `bars_stream`、`trades_all` / `trades_stream`、`snapshots_all` / `snapshots_stream`、`chain_all` / `chain_stream`

## Non-Goals

- 不在本 phase 内实现 `crypto`、`news`、`corporate_actions`
- 不实现 WebSocket / SSE
- 不增加任何官方 HTTP API 之外的派生 analytics
- 不发明 `symbol("...")` 一类 facade
- 不为 options 补 single historical / single latest facade，除非官方 HTTP API 本身提供对应端点

## Canonical Standard

唯一语义标准是 Alpaca 官方 Market Data HTTP API 与真实返回体。Rust 侧只做最小必要适配。

官方参考文档：

- Options bars: https://docs.alpaca.markets/reference/optionbars
- Options trades: https://docs.alpaca.markets/reference/optiontrades
- Options latest quotes: https://docs.alpaca.markets/reference/optionlatestquotes
- Options latest trades: https://docs.alpaca.markets/reference/optionlatesttrades
- Options snapshots: https://docs.alpaca.markets/reference/optionsnapshots
- Options chain: https://docs.alpaca.markets/reference/optionchain
- Options exchange codes: https://docs.alpaca.markets/reference/optionmetaexchanges

另外，2026-04-03 已用真实 API 交叉验证以下关键事实：

- `chain` 的官方 path 是 `/v1beta1/options/snapshots/{underlying_symbol}`
- `exchange_codes` 返回顶层 map，不是 wrapper object
- `latest_quotes` 返回顶层 `quotes` map
- `latest_trades` 返回顶层 `trades` map
- `bars` / `trades` 返回顶层 symbol-keyed map + `next_page_token`
- `snapshots` 与 `chain` 都返回顶层 `snapshots` map + `next_page_token`
- `bars` / `trades` 不接受 `feed` query；真实 API 会返回 `400 unexpected query parameter(s): feed`
- `snapshots` 与 `chain` 接受 `feed`
- `chain` 继续接受官方过滤 query，例如 `type`、`strike_price_gte`、`expiration_date_gte`、`updated_since`

## Naming Decision

采用与 `stocks` 一致的命名原则：保留官方词根，Rust 风格只做最小适配。

### Public methods

- `bars`
- `trades`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `chain`
- `exchange_codes`

### Convenience methods

- `bars_all` / `bars_stream`
- `trades_all` / `trades_stream`
- `snapshots_all` / `snapshots_stream`
- `chain_all` / `chain_stream`

### Request / response names

- `BarsRequest` / `BarsResponse`
- `TradesRequest` / `TradesResponse`
- `LatestQuotesRequest` / `LatestQuotesResponse`
- `LatestTradesRequest` / `LatestTradesResponse`
- `SnapshotsRequest` / `SnapshotsResponse`
- `ChainRequest` / `ChainResponse`
- `ExchangeCodesResponse`

## Request Design

### Historical

`bars`：

- `symbols`
- `timeframe`
- `start`
- `end`
- `limit`
- `sort`
- `page_token`

`trades`：

- `symbols`
- `start`
- `end`
- `limit`
- `sort`
- `page_token`

### Latest

`latest_quotes` / `latest_trades`：

- `symbols`
- `feed`

### Snapshots

`snapshots`：

- `symbols`
- `feed`
- `limit`
- `page_token`

### Chain

`chain`：

- path: `underlying_symbol`
- query: `feed`
- query: `r#type` -> serialize as official `type`
- query: `strike_price_gte`
- query: `strike_price_lte`
- query: `expiration_date`
- query: `expiration_date_gte`
- query: `expiration_date_lte`
- query: `root_symbol`
- query: `updated_since`
- query: `limit`
- query: `page_token`

## Response / Model Design

### Historical wrappers

- `BarsResponse { bars, next_page_token }`
- `TradesResponse { trades, next_page_token }`

都保持顶层 symbol-keyed map 形状，不发明新的 wrapper 字段。

### Latest wrappers

- `LatestQuotesResponse { quotes }`
- `LatestTradesResponse { trades }`

### Snapshot wrappers

- `SnapshotsResponse { snapshots, next_page_token }`
- `ChainResponse { snapshots, next_page_token }`

`chain` 与 `snapshots` 在返回体形状上保持一致，只是获取方式不同，因此模型层复用 `Snapshot`。

### Snapshot model

`Snapshot` 保持官方字段名：

- `latestTrade`
- `latestQuote`
- `minuteBar`
- `dailyBar`
- `prevDailyBar`
- `greeks`
- `impliedVolatility`

其中 `greeks` 为 typed struct，继续保持官方 field word：`delta`、`gamma`、`rho`、`theta`、`vega`。

### Metadata

`ExchangeCodesResponse` 直接保持顶层 map 形状：`HashMap<String, String>`。

## Internal Architecture

- 公开入口继续使用 `alpaca_data::Client::options()`
- 资源布局继续保持：`client.rs`、`request.rs`、`response.rs`、`model.rs`、`enums.rs`
- `client.rs` 负责 query 组装、endpoint 路由、transport 调用、pagination helper 集成
- `request.rs` / `response.rs` / `model.rs` 只做 typed contract 建模
- 对分页 endpoint，统一复用共享 `PaginatedRequest`、`PaginatedResponse`、`collect_all`、`stream_pages`
- 对 top-level map response，保持官方 body 形状，不为了统一感引入伪 wrapper

## Testing Strategy

- 正常成功路径优先真实 Alpaca API
- `bars` / `trades` / `latest_quotes` / `latest_trades` / `snapshots` / `chain` / `exchange_codes` 都要有 live coverage
- 只有损坏 JSON、分页 token 错误、跨页合并不一致等异常路径才使用 mock
- 真实 API 测试参数优先选择 `AAPL` 等流动性更高的 underlying，并把 page count 控制在可验证但不过重的范围内
- benchmark 基线单独在 phase 收尾时补 `benches/options.rs`

## Task Breakdown

1. 历史 `bars` / `trades` 底层 mirror + typed model + request serialization + live baseline
2. `latest_quotes` / `latest_trades` + `exchange_codes`
3. `snapshots` / `chain` + greeks / implied volatility 建模 + `*_all` / `*_stream`
4. mock fault tests、benchmark、phase 收尾文档与版本升级
