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
- 对于官方直接返回顶层动态 key JSON object 的 endpoint，不额外发明 wrapper；响应类型保持 map 形状，例如 `stocks::SnapshotsResponse`、`stocks::ConditionCodesResponse`、`stocks::ExchangeCodesResponse`
