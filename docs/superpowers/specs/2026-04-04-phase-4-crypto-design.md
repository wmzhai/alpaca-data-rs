# Phase 4 Crypto Design

**Date:** 2026-04-04
**Status:** Historical + latest implemented in `v0.3.2`; phase still in progress
**Target Phase:** `Phase 4: Crypto`

## Goal

将 `crypto` 做成继 `stocks`、`options` 之后的第三个完整资源域，忠实对应 Alpaca Market Data HTTP API 的 crypto HTTP 端点，并锁定 `loc` 路由、公开访问行为、以及 crypto 特有的数值字段模型。

## Scope

本 phase 只覆盖官方 crypto HTTP API 中已经列入 roadmap 的端点：

- historical: `bars`、`quotes`、`trades`
- latest: `latest_bars`、`latest_quotes`、`latest_trades`、`latest_orderbooks`
- snapshot: `snapshots`
- convenience: `bars_all` / `bars_stream`、`quotes_all` / `quotes_stream`、`trades_all` / `trades_stream`

## Non-Goals

- 不在本 phase 内实现 `news`、`corporate_actions`
- 不实现 WebSocket / SSE
- 不增加任何官方 HTTP API 之外的派生 analytics
- 不发明 `symbol("...")` 一类 facade
- 不为 crypto 增加官方不存在的 single-symbol endpoint
- 不保留任何官方 HTTP API 中并不存在的 `feed` query 抽象

## Canonical Standard

唯一语义标准是 Alpaca 官方 Market Data HTTP API 与真实返回体。Rust 侧只做最小必要适配。

官方参考文档：

- Crypto bars: https://docs.alpaca.markets/reference/cryptobars-1
- Crypto quotes: https://docs.alpaca.markets/reference/cryptoquotes-1
- Crypto trades: https://docs.alpaca.markets/reference/cryptotrades-1
- Crypto latest bars: https://docs.alpaca.markets/reference/cryptolatestbars-1
- Crypto latest quotes: https://docs.alpaca.markets/reference/cryptolatestquotes-1
- Crypto latest trades: https://docs.alpaca.markets/reference/cryptolatesttrades-1
- Crypto latest orderbooks: https://docs.alpaca.markets/reference/cryptolatestorderbooks-1
- Crypto snapshots: https://docs.alpaca.markets/reference/cryptosnapshots-1

另外，2026-04-04 已用真实 API 交叉验证以下关键事实：

- 全部 crypto HTTP endpoint 当前都可在无凭证下成功访问；`Client::builder().build()?.crypto()` 必须继续可用
- `loc` 是 path segment，不是 query；发送 `loc=...` 会返回 `400 unexpected query parameter(s): loc`
- 官方 path 必须使用带连字符的 location word：`us`、`us-1`、`eu-1`
- 现有占位实现中的 `us1` path 是错误行为，必须修正
- historical 端点：
  - `/v1beta3/crypto/{loc}/bars`
  - `/v1beta3/crypto/{loc}/quotes`
  - `/v1beta3/crypto/{loc}/trades`
- latest 端点：
  - `/v1beta3/crypto/{loc}/latest/bars`
  - `/v1beta3/crypto/{loc}/latest/quotes`
  - `/v1beta3/crypto/{loc}/latest/trades`
  - `/v1beta3/crypto/{loc}/latest/orderbooks`
- snapshot 端点：
  - `/v1beta3/crypto/{loc}/snapshots`
- `bars` / `quotes` / `trades` 返回顶层 symbol-keyed map + `next_page_token`
- `latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks` 返回顶层 map，不带 `next_page_token`
- `snapshots` 返回顶层 `snapshots` map，不分页；`limit` 会返回 `400 unexpected query parameter(s): limit`
- crypto 的历史 `v`、quote size、trade size、orderbook level `s` 都可能是小数，不能沿用 stocks/options 的整数 size 建模

## Naming Decision

采用与 `stocks` / `options` 一致的命名原则：保留官方词根，Rust 风格只做最小适配。

### Public methods

- `bars`
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `latest_orderbooks`
- `snapshots`

### Convenience methods

- `bars_all` / `bars_stream`
- `quotes_all` / `quotes_stream`
- `trades_all` / `trades_stream`

### Request / response names

- `BarsRequest` / `BarsResponse`
- `QuotesRequest` / `QuotesResponse`
- `TradesRequest` / `TradesResponse`
- `LatestBarsRequest` / `LatestBarsResponse`
- `LatestQuotesRequest` / `LatestQuotesResponse`
- `LatestTradesRequest` / `LatestTradesResponse`
- `LatestOrderbooksRequest` / `LatestOrderbooksResponse`
- `SnapshotsRequest` / `SnapshotsResponse`

## Request Design

### Shared `loc`

所有 crypto request 都保留：

- `loc: Option<Loc>`

但 `loc` 只用于 Rust 侧 path 选择，不序列化进 query。默认值保持 `Loc::Us`。

`Loc` 公开值：

- `Us`
- `Us1`
- `Eu1`

对应官方 path word：

- `us`
- `us-1`
- `eu-1`

### Historical

`bars`：

- `symbols`
- `timeframe`
- `start`
- `end`
- `limit`
- `sort`
- `page_token`

`quotes` / `trades`：

- `symbols`
- `start`
- `end`
- `limit`
- `sort`
- `page_token`

### Latest

`latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks`：

- `symbols`

### Snapshots

`snapshots`：

- `symbols`

## Response / Model Design

### Historical wrappers

- `BarsResponse { bars, next_page_token }`
- `QuotesResponse { quotes, next_page_token }`
- `TradesResponse { trades, next_page_token }`

都保持顶层 symbol-keyed map 形状，不发明新的 wrapper 字段。

### Latest wrappers

- `LatestBarsResponse { bars }`
- `LatestQuotesResponse { quotes }`
- `LatestTradesResponse { trades }`
- `LatestOrderbooksResponse { orderbooks }`

### Snapshot wrapper

- `SnapshotsResponse { snapshots }`

不添加并不存在的 `next_page_token`。

### Bar model

`Bar` 保持官方字段名：

- `t`
- `o`
- `h`
- `l`
- `c`
- `v`
- `n`
- `vw`

其中：

- `v` 使用 `f64`
- `n` 使用 `u64`

### Quote model

`Quote` 保持官方字段名：

- `t`
- `bp`
- `bs`
- `ap`
- `as`

其中 `bs` / `as` 使用 `f64`。

### Trade model

`Trade` 保持官方字段名：

- `t`
- `p`
- `s`
- `i`
- `tks`

其中：

- `s` 使用 `f64`
- `i` 使用 `u64`
- `tks` 使用 `String`

### Orderbook model

`Orderbook` 保持官方字段名：

- `t`
- `b`
- `a`

其中 `b` / `a` 都是 `Vec<OrderbookLevel>`，level 保持：

- `p`
- `s`

### Snapshot model

`Snapshot` 保持官方字段名：

- `latestTrade`
- `latestQuote`
- `minuteBar`
- `dailyBar`
- `prevDailyBar`

## Internal Architecture

- 公开入口继续使用 `alpaca_data::Client::crypto()`
- 资源布局继续保持：`client.rs`、`request.rs`、`response.rs`、`model.rs`、`enums.rs`
- `client.rs` 负责 query 组装、endpoint 路由、transport 调用、pagination helper 集成
- `request.rs` / `response.rs` / `model.rs` 只做 typed contract 建模
- 对分页 endpoint，统一复用共享 `PaginatedRequest`、`PaginatedResponse`、`collect_all`、`stream_pages`
- `snapshots` 不分页，因此不增加 `snapshots_all` / `snapshots_stream`
- 删除占位且不符合官方 HTTP API 的 `CryptoFeed`
- `TimeFrame` 改为与 `stocks` / `options` 一致的透明 string newtype，以避免把官方 timeframe 词汇硬编码为过窄枚举

## Testing Strategy

- 正常成功路径优先真实 Alpaca API
- `bars` / `quotes` / `trades` / `latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks` / `snapshots` 都要有 live coverage
- 只有损坏 JSON、错误 location、分页合并异常等故障场景才使用 mock
- 真实 API 测试默认使用公开可访问的 `BTC/USD`、`ETH/USD`
- live tests 需要覆盖：
  - 无凭证 client 成功访问 core crypto endpoints
  - `Loc::Us`、`Loc::Us1`、`Loc::Eu1` 路由至少抽样验证
  - historical convenience 层的多页聚合
- benchmark 基线单独在 phase 收尾时补 `benches/crypto.rs`

## Task Breakdown

1. `loc` / endpoint routing 修正，历史 `bars` / `quotes` / `trades` mirror + pagination convenience + live baseline
2. `latest_bars` / `latest_quotes` / `latest_trades` / `latest_orderbooks` + typed latest models + live baseline
3. `snapshots` + crypto snapshot / orderbook model + loc/no-auth live coverage + mock fault tests
4. `criterion` benchmark、phase 收尾文档、版本升级与主干合并
