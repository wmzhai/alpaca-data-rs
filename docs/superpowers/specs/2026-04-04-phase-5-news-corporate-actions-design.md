# Phase 5 News and Corporate Actions Design

**Date:** 2026-04-04
**Status:** Approved (`news` shipped in `v0.4.1`; `corporate_actions` shipped in `v0.4.2`; fault coverage + benchmark shipped in `v0.4.3`; Task 4 pending)
**Target Phase:** `Phase 5: News + Corporate Actions`

## Goal

将 `news` 与 `corporate_actions` 做成继 `stocks`、`options`、`crypto` 之后的第四、第五个资源域，忠实对应 Alpaca Market Data HTTP API 的 list/filter/pagination 型 HTTP 端点，并把这两个资源域接入现有共享 pagination helper、真实 API happy-path 测试和 phase 级 benchmark / 文档收尾流程。

## Approval Gate

- 本文档和对应 implementation plan 已在 2026-04-04 获得用户确认，并已作为 `Phase 5` 的执行基线。
- 已提交的 `news` 基线与已落地的 `corporate_actions` / shared pagination 实现都已按最终方案对齐。

## Current Baseline

截至 `v0.4.3`：

- `news.list`、`news.list_all`、`news.list_stream` 已在 `v0.4.1` 落地并提交
- `tests/live_news.rs` 已验证 `news` happy path
- `corporate_actions.list`、`list_all`、`list_stream` 已在 `v0.4.2` 落地并提交
- `tests/live_corporate_actions.rs` 已验证 `corporate_actions` happy path
- 共享 pagination helper 已补上重复 `next_page_token` 防护
- `tests/mock_news_corporate_actions_errors.rs` 已覆盖 `news` / `corporate_actions` 的损坏 JSON、分页 merge 与重复 `next_page_token` 回归
- `benches/news_corporate_actions.rs` 已为 `news.list` 与 `corporate_actions.list` 建立本地 benchmark baseline

## Scope

本 phase 只覆盖 roadmap 中已经明确列入的两个资源域：

- `news.list`
- `news.list_all`
- `news.list_stream`
- `corporate_actions.list`
- `corporate_actions.list_all`
- `corporate_actions.list_stream`

## Non-Goals

- 不实现 Trading API、Broker API、WebSocket / SSE
- 不实现 `news` / `corporate_actions` 之外的新资源域
- 不发明官方 HTTP API 中不存在的 facade 或聚合捷径
- 不把 `corporate_actions` 拍平成自定义统一数组形态
- 不为了 undocumented bucket 的 item 形状去猜测并伪造不存在的 typed 字段

## Canonical Standard

唯一语义标准是 Alpaca 官方 Market Data HTTP API 的官方文档、官方 OpenAPI，以及 2026-04-04 使用真实 API 的交叉验证结果。Rust 侧只做最小必要适配。

官方参考文档：

- News reference: https://docs.alpaca.markets/reference/news-1
- Corporate actions reference: https://docs.alpaca.markets/reference/corporateactions-1
- Market Data OpenAPI: https://docs.alpaca.markets/openapi/market-data-api.json

2026-04-04 已确认的关键事实：

- `news` endpoint 路径是 `/v1beta1/news`
- `corporate_actions` endpoint 路径是 `/v1/corporate-actions`
- 两个 endpoint 都会对未知 query 参数返回 `400 {"message":"unexpected query parameter(s): ..."}`，因此 query writer 必须严格只发送官方参数
- `news` 返回顶层 `news: []` 与 `next_page_token`
- `news` item 当前真实字段包含：`id`、`headline`、`author`、`created_at`、`updated_at`、`summary`、`content`、`url`、`images`、`symbols`、`source`
- 当 `include_content=false` 时，真实 API 仍然返回 `content` 字段，但值会是空字符串
- `news.images[*]` 当前真实字段是 `size` 与 `url`
- `corporate_actions` 返回顶层 `corporate_actions: { ... }` 与 `next_page_token`
- `corporate_actions` 的 `corporate_actions` 不是数组，而是按 action family 分 bucket 的对象；真实 API 已验证存在 `name_changes`、`cash_dividends`、`forward_splits` 等 bucket
- `corporate_actions.types` 当前真实 API 接受的合法 query 值为：
  - `forward_split`
  - `reverse_split`
  - `unit_split`
  - `stock_dividend`
  - `cash_dividend`
  - `spin_off`
  - `cash_merger`
  - `stock_merger`
  - `stock_and_cash_merger`
  - `redemption`
  - `name_change`
  - `worthless_removal`
  - `rights_distribution`
  - `contract_adjustment`
  - `partial_call`
- 当前官方 OpenAPI 为前 13 个 type 提供了明确 bucket schema，但尚未给 `contract_adjustment` 与 `partial_call` 提供公开 item schema；实现必须保留这两个 type 的 query 支持，同时避免在 response 里无依据地伪造字段

## Naming Decision

继续沿用项目已经定型的命名原则：保留官方 endpoint / query / response 词根，Rust 风格只做最小适配。

### Public methods

- `news.list`
- `news.list_all`
- `news.list_stream`
- `corporate_actions.list`
- `corporate_actions.list_all`
- `corporate_actions.list_stream`

### Request / response names

- `news::ListRequest`
- `news::ListResponse`
- `news::NewsItem`
- `news::NewsImage`
- `corporate_actions::ListRequest`
- `corporate_actions::ListResponse`
- `corporate_actions::CorporateActions`

### Corporate action model names

公开导出 action-family 对应的 typed model：

- `ForwardSplit`
- `ReverseSplit`
- `UnitSplit`
- `StockDividend`
- `CashDividend`
- `SpinOff`
- `CashMerger`
- `StockMerger`
- `StockAndCashMerger`
- `Redemption`
- `NameChange`
- `WorthlessRemoval`
- `RightsDistribution`

另外提供一个保守 fallback：

- `UnknownCorporateAction`

它只用于当前官方 query 已接受、但官方公开 schema 尚未给出的 response family，例如 `contract_adjustments` 与 `partial_calls`。

## Request Design

### News

`news::ListRequest` 保留官方 query 单词：

- `start`
- `end`
- `sort`
- `symbols`
- `limit`
- `include_content`
- `exclude_contentless`
- `page_token`

Rust 侧适配：

- `symbols` 使用 `Option<Vec<String>>`，序列化时仍发送官方 CSV 参数 `symbols=AAPL,TSLA,BTCUSD`
- 其他可选字段使用 `Option<T>`
- `sort` 继续复用共享 `Sort`

### Corporate Actions

`corporate_actions::ListRequest` 保留官方 query 单词：

- `symbols`
- `cusips`
- `types`
- `start`
- `end`
- `ids`
- `limit`
- `sort`
- `page_token`

Rust 侧适配：

- `symbols` / `cusips` / `ids` 使用 `Option<Vec<String>>`
- `types` 使用 `Option<Vec<CorporateActionType>>`
- `start` / `end` 保持 `Option<String>`，继续直接承载官方 `YYYY-MM-DD`
- `sort` 继续复用共享 `Sort`

`CorporateActionType` 必须覆盖当前真实 API 接受的全部 15 个 query 值，即使其中两个 type 目前没有公开 response schema。

## Response and Model Design

### News response

`news` 保持标准官方 wrapper：

```rust
pub struct ListResponse {
    pub news: Vec<NewsItem>,
    pub next_page_token: Option<String>,
}
```

`NewsItem` 只保留官方字段：

- `id: i64`
- `headline: String`
- `author: String`
- `created_at: String`
- `updated_at: String`
- `summary: String`
- `content: String`
- `url: Option<String>`
- `images: Vec<NewsImage>`
- `symbols: Vec<String>`
- `source: String`

`NewsImage` 只保留：

- `size: String`
- `url: String`

### Corporate actions response

`corporate_actions` 保持官方两层包裹，不发明自定义扁平数组：

```rust
pub struct ListResponse {
    pub corporate_actions: CorporateActions,
    pub next_page_token: Option<String>,
}
```

`CorporateActions` 使用“typed documented buckets + conservative fallback buckets”的设计：

- documented buckets 使用显式字段和 typed `Vec<_>`
- 当前真实 API 已接受但无公开 schema 的 buckets 也保留显式字段
- 再加一层 `flatten` fallback，避免未来官方新增 bucket 时 silently drop 数据

`CorporateActions` 公开字段：

- `forward_splits`
- `reverse_splits`
- `unit_splits`
- `stock_dividends`
- `cash_dividends`
- `spin_offs`
- `cash_mergers`
- `stock_mergers`
- `stock_and_cash_mergers`
- `redemptions`
- `name_changes`
- `worthless_removals`
- `rights_distributions`
- `contract_adjustments`
- `partial_calls`

其中：

- 前 13 个字段按官方公开 schema 建模为 `Vec<ForwardSplit>` 等 typed arrays
- `contract_adjustments` 与 `partial_calls` 当前建模为 `Vec<UnknownCorporateAction>`
- 额外未知 bucket 进入 `other: BTreeMap<String, Vec<UnknownCorporateAction>>`

`UnknownCorporateAction` 的职责不是替代正常 typed model，而是保留当前官方未公开 schema 的原始对象键值，避免 response 数据被直接吞掉。

## Pagination Semantics

- `news.list_all` 与 `corporate_actions.list_all` 都复用共享 `collect_all`
- `news.list_stream` 与 `corporate_actions.list_stream` 都复用共享 `stream_pages`
- `*_all` 返回同名 `ListResponse`，并在聚合后令 `next_page_token = None`
- `*_stream` 按页返回 `ListResponse`
- `news` 分页合并逻辑是单纯追加 `news`
- `corporate_actions` 分页合并逻辑是逐 bucket 追加对应数组；不会尝试跨 bucket 去重或重排
- 共享 pagination helper 还必须防御“服务端重复返回同一个 `next_page_token`”的情况；一旦检测到重复 token，统一返回 `Error::Pagination`，避免 `list_all` / `list_stream` 陷入无限循环

## Internal Architecture

- 公开入口继续使用 `alpaca_data::Client::news()` 与 `alpaca_data::Client::corporate_actions()`
- 资源布局继续保持 `client.rs`、`request.rs`、`response.rs`、`model.rs`
- `client.rs` 负责 credentials gate、query 组装、endpoint 路由、transport 调用和 shared pagination helper 集成
- `request.rs` 负责 typed query 与 `PaginatedRequest`
- `response.rs` 负责 typed wrapper 与 `PaginatedResponse`
- `model.rs` 负责 item schema
- `transport::endpoint::Endpoint` 增加 `NewsList` 与 `CorporateActionsList`

## Testing Strategy

- 正常成功路径优先真实 Alpaca API
- `news` 必须有 live coverage，至少验证：
  - `list`
  - `list_all`
  - `list_stream`
  - `include_content=false` 仍返回 `content` 字段
- `corporate_actions` 必须有 live coverage，至少验证：
  - `list`
  - `list_all`
  - `list_stream`
  - bucketed wrapper 形状
  - 至少两个不同 family 的真实 item shape
  - 重复 `next_page_token` 不会让共享分页 helper 无限循环
- mock 只用于：
  - 损坏 JSON
  - transport error
  - 分页 merge 行为
  - undocumented bucket 的保留行为回归测试
  - 重复 `next_page_token` 的故障注入回归测试

## Exit Criteria

- `news` 与 `corporate_actions` 的 happy path 都使用真实 Alpaca API 验证通过
- `corporate_actions` 的 bucketed response 形状被 typed Rust object 正确保留
- `contract_adjustment` / `partial_call` query 不会因为 SDK enum 缺失而无法发起请求
- `corporate_actions` 相关真实 API 分页路径不会因为重复 `next_page_token` 而挂死
- 所有 phase 5 文档、CHANGELOG、tests、benchmark 和版本号在 phase 收尾时保持一致
