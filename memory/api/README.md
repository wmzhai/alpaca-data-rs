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
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `chain`
- `list`

## 上层便利层

在同一资源 client 下补：

- `*_all`
- `*_stream`

语义约束：

- `*_all` 自动取完所有分页，返回同名 `Response`
- `*_stream` 按页返回同名 `Response`

## 命名约束

- 资源模块承担资源语义
- 类型名尽量短，不重复资源前缀
- 例如：`stocks::BarsRequest`、`options::ChainResponse`
- 不使用 `StockBarsRequest` 这类重复命名

## 字段约束

- 请求字段名和响应字段名都必须直接使用官方原词
- 一个字母都不要改
- Rust 关键字冲突时，只做最小适配，例如 `r#type`
