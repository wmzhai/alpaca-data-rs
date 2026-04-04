# 公开 API 约定

这份文档只记录当前已经拍板的公开 API 设计约束。

## 根入口

- `alpaca_data::Client`
- `Client::new()`
- `Client::builder()`

## 资源入口

- `client.stocks()`
- `client.options()`
- `client.crypto()`
- `client.news()`
- `client.corporate_actions()`

## 底层镜像层

底层方法直接对应官方资源和 endpoint 语义，例如：

- `bars`
- `bars_single`
- `quotes`
- `quotes_single`
- `trades`
- `trades_single`
- `latest_bars`
- `latest_bar`
- `latest_quotes`
- `latest_quote`
- `latest_trades`
- `latest_trade`
- `snapshots`
- `snapshot`
- `condition_codes`
- `exchange_codes`
- `chain`
- `list`

## 上层便利层

在同一资源 client 下补：

- `*_all`
- `*_stream`

语义约束：

- 只在官方 endpoint 本身存在分页语义时补 convenience 层
- `*_all` 自动取完所有分页，返回同名 `Response`
- `*_stream` 按页返回同名 `Response`
- 对存在官方 single-symbol historical endpoint 的资源域，也允许补 `*_single_all` 和 `*_single_stream`

## 命名约束

- 资源模块承担资源语义
- 类型名尽量短，不重复资源前缀
- 例如：`stocks::BarsRequest`、`options::ChainResponse`
- 不使用 `StockBarsRequest` 这类重复命名
- single-symbol endpoint 在需要和 batch 区分时，使用最小后缀，例如 `bars_single`
- latest / snapshot 的 single endpoint 优先使用单数语义，例如 `latest_quote`、`snapshot`
- `stocks` 已经作为第一份完整模板验证了 batch + single + convenience 三层命名模式；后续资源域默认沿用这套命名边界

## 字段约束

- 请求字段名和响应字段名都必须直接使用官方原词
- 一个字母都不要改
- Rust 关键字冲突时，只做最小适配，例如 `r#type`
- metadata 请求也不例外：`stocks::ConditionCodesRequest` 继续使用 `ticktype` 和 `tape`，对应 Rust enum 为 `stocks::TickType` 和 `stocks::Tape`
- 对于官方直接返回顶层动态 key JSON object 的 endpoint，不额外发明 wrapper；响应类型保持 map 形状，例如 `stocks::SnapshotsResponse`、`stocks::ConditionCodesResponse`、`stocks::ExchangeCodesResponse`、`options::ExchangeCodesResponse`
- 对于官方同时返回 `next_page_token` 和动态 key map 的 endpoint，继续保留官方 wrapper 结构，例如 `options::SnapshotsResponse` 和 `options::ChainResponse` 继续公开顶层 `snapshots` + `next_page_token`
- `stocks`、`options` 与 `crypto` 现已成为前三个完整资源模板；其中 `crypto` 明确 `loc` 只作为 path 路由而不进入 query、`snapshots` 不补 convenience
- `news` 现已完成第四个资源模板的第一步：`list` / `list_all` / `list_stream` 都已落地，`symbols` 以 `Option<Vec<String>>` 承载官方 CSV query，response 保持官方 `news` + `next_page_token` wrapper
- `corporate_actions` 现已接通官方 `list` / `list_all` / `list_stream`，并保持 `corporate_actions` 顶层 bucketed wrapper：13 个 documented bucket 使用 typed `Vec<_>`，`contract_adjustments` / `partial_calls` 使用 `UnknownCorporateAction`，未来未知 bucket 进入 `other`
- `corporate_actions::CorporateActionType` 现在覆盖当前真实 API 接受的全部 15 个 query 值；共享 pagination helper 也已补上重复 `next_page_token` 防护，避免 `list_all` / `list_stream` 卡死
- `tests/mock_news_corporate_actions_errors.rs` 现在覆盖 `news` / `corporate_actions` 的损坏 JSON、分页 merge 与重复 `next_page_token` 回归；`benches/news_corporate_actions.rs` 则为两个资源域补齐本地 benchmark baseline
- `news` 与 `corporate_actions` 现在已成为第四、第五个完整资源模板；当前收尾工作只剩 Phase 5 的 merge 审批和后续 `Phase 6: Release Hardening`
