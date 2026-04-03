# alpaca_data

`alpaca_data` 是一个面向 Rust 的高性能 Alpaca Market Data API HTTP 客户端。

本项目的最终设计目标是：

- 以 Alpaca 官方 Market Data HTTP API 为唯一标准
- 公开 API 的词汇尽量与官方 endpoint 和参数一一对应
- 命名、模块、类型风格完全遵循标准 Rust crate 习惯
- 同时提供两层接口：
  - 底层镜像层：忠实映射官方 HTTP API
  - 上层便利层：提供一次性取全量和流式读取等 Rust 友好封装

当前明确不包含：

- Trading API
- Broker API
- WebSocket / SSE / 实时流
- 隐式缓存
- 默认 Greeks / IV 补算

## 当前实现状态

- 当前已完成 `Phase 1: Shared Core`，并已进入 `Phase 2: Stocks`；当前已落地到 `v0.1.5`
- 已落地共享 `ClientBuilder` 运行时配置、认证配对校验与 header 注入、query 构造、endpoint 路由、async HTTP transport、错误映射和分页 helper
- 当前真实打通的 endpoint 包括 `crypto.latest_quotes`，以及 `stocks` 的历史 batch / single `bars`、`quotes`、`trades`、latest / snapshot 的 batch / single 端点，以及 metadata `condition_codes` / `exchange_codes`
- `stocks` 的 `bars_single_all` / `bars_single_stream`、`quotes_single_all` / `quotes_single_stream`、`trades_single_all` / `trades_single_stream` 也已接到共享分页便利层
- 真实 happy-path 测试已覆盖 `crypto.latest_quotes`、`stocks` 历史 batch / single、latest / snapshot，以及 metadata 端点
- 当前 `stocks` batch historical 的 `bars_all` / `bars_stream`、`quotes_all` / `quotes_stream`、`trades_all` / `trades_stream` 和 `benches/stocks.rs` 仍在收尾中，随后再完成 `Phase 2`
- 当前本地 micro-benchmark baseline 位于 `benches/shared_core.rs`，资源级 benchmark 会在后续 phase task 继续补齐

## 设计原则

### 1. 官方 HTTP API 是唯一标准

- 方法语义以官方 Market Data API 为准
- 请求字段名和响应字段名直接使用官方原始单词
- 字段名一个字母都不改
- 只有在 Rust 语法必须要求时才做最小适配

例如：

- `symbols`
- `start`
- `end`
- `limit`
- `feed`
- `page_token`
- `next_page_token`
- `corporate_actions`

如果字段名与 Rust 关键字冲突，则使用最小 Rust 适配：

```rust
pub struct ChainRequest {
    pub underlying_symbol: String,
    pub r#type: Option<ContractType>,
}
```

序列化到 HTTP query 时仍然发送官方原始参数 `type`。

### 2. Rust 风格优先于 Python 风格

虽然语义上对齐官方 API，但具体命名形式遵循标准 Rust 风格：

- crate 名：`alpaca_data`
- 根 client：`alpaca_data::Client`
- 模块名：`stocks`、`options`、`crypto`
- 类型名：`BarsRequest`、`BarsResponse`、`Trade`
- 方法名：`bars`、`bars_single`、`latest_quotes`、`latest_quote`、`snapshots`、`snapshot`
- 字段名：`snake_case`

不使用冗长的类型名去重复模块语义，例如：

- 使用 `stocks::BarsRequest`
- 不使用 `StockBarsRequest`

### 3. 只做 async

本库只提供 `async` API。

- 不提供 `sync` 版本
- 不维护双栈接口
- 优先为高吞吐、低开销、批量场景优化

### 4. 两层 API 并存

#### 底层镜像层

底层方法保持官方原始形态：

- 方法名贴近官方 endpoint
- 返回值保留官方响应包裹结构对应的 Rust 对象
- 分页字段按官方语义保留，例如 `next_page_token`

#### 上层便利层

上层只添加少量便利封装：

- `*_all`
- `*_stream`

其中：

- `*_all`：自动取完所有分页，返回同名 `Response`
- `*_stream`：按页异步返回同名 `Response`

## 官方 API 范围

本库只覆盖 Alpaca 官方文档中的 Market Data API HTTP 端点。

当前设计范围按资源域划分为：

- Stocks
- Options
- Crypto
- News
- Corporate Actions

后续可再评估是否补充：

- Screener
- Logos
- Forex
- Fixed Income

## 公开 API 形状

### 根入口

```rust
use alpaca_data::Client;

let client = Client::new();
let client = Client::builder().build()?;
```

### 资源入口

```rust
client.stocks()
client.options()
client.crypto()
client.news()
client.corporate_actions()
```

### 公开模块

```text
alpaca_data::Client
alpaca_data::Error
alpaca_data::stocks
alpaca_data::options
alpaca_data::crypto
alpaca_data::news
alpaca_data::corporate_actions
```

## 资源方法矩阵

### Stocks

底层镜像层：

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

上层便利层：

- `bars_all`
- `bars_stream`
- `bars_single_all`
- `bars_single_stream`
- `quotes_all`
- `quotes_stream`
- `quotes_single_all`
- `quotes_single_stream`
- `trades_all`
- `trades_stream`
- `trades_single_all`
- `trades_single_stream`

### Options

底层镜像层：

- `bars`
- `trades`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `chain`
- `exchange_codes`

上层便利层：

- `bars_all`
- `bars_stream`
- `trades_all`
- `trades_stream`
- `snapshots_all`
- `snapshots_stream`
- `chain_all`
- `chain_stream`

### Crypto

底层镜像层：

- `bars`
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `latest_orderbooks`
- `snapshots`

上层便利层：

- `bars_all`
- `bars_stream`
- `quotes_all`
- `quotes_stream`
- `trades_all`
- `trades_stream`

### News

底层镜像层：

- `list`

上层便利层：

- `list_all`
- `list_stream`

### Corporate Actions

底层镜像层：

- `list`

上层便利层：

- `list_all`
- `list_stream`

## 请求与响应建模

### 请求对象

每个 endpoint 单独一个 `Request` 类型。

示例：

- `stocks::BarsRequest`
- `stocks::BarsSingleRequest`
- `stocks::QuotesRequest`
- `stocks::QuotesSingleRequest`
- `stocks::TradesRequest`
- `stocks::TradesSingleRequest`
- `stocks::LatestBarsRequest`
- `stocks::LatestBarRequest`
- `stocks::LatestQuotesRequest`
- `stocks::LatestQuoteRequest`
- `stocks::LatestTradesRequest`
- `stocks::LatestTradeRequest`
- `stocks::SnapshotsRequest`
- `stocks::SnapshotRequest`
- `stocks::ConditionCodesRequest`
- `stocks::TickType`
- `stocks::Tape`

- `options::BarsRequest`
- `options::TradesRequest`
- `options::LatestQuotesRequest`
- `options::LatestTradesRequest`
- `options::SnapshotsRequest`
- `options::ChainRequest`

- `crypto::BarsRequest`
- `crypto::QuotesRequest`
- `crypto::TradesRequest`
- `crypto::LatestBarsRequest`
- `crypto::LatestQuotesRequest`
- `crypto::LatestTradesRequest`
- `crypto::LatestOrderbooksRequest`
- `crypto::SnapshotsRequest`

- `news::ListRequest`
- `corporate_actions::ListRequest`

请求对象约束：

- 字段名严格使用官方参数原词
- 可选参数使用 `Option<T>`
- 枚举型参数使用 Rust enum
- 枚举序列化值严格等于官方字符串
- metadata 请求继续直接使用官方参数词：`ConditionCodesRequest` 保留 `ticktype` 和 `tape`，其中 `stocks::TickType` 序列化为 `trade|quote`，`stocks::Tape` 序列化为 `A|B|C`
- 不添加官方 HTTP API 中不存在的自定义字段

### 响应对象

每个 endpoint 单独一个 `Response` 类型。

示例：

- `stocks::BarsResponse`
- `stocks::BarsSingleResponse`
- `stocks::QuotesResponse`
- `stocks::QuotesSingleResponse`
- `stocks::TradesResponse`
- `stocks::TradesSingleResponse`
- `stocks::LatestBarsResponse`
- `stocks::LatestBarResponse`
- `stocks::LatestQuotesResponse`
- `stocks::LatestQuoteResponse`
- `stocks::LatestTradesResponse`
- `stocks::LatestTradeResponse`
- `stocks::SnapshotsResponse`
- `stocks::SnapshotResponse`
- `stocks::ConditionCodesResponse`
- `stocks::ExchangeCodesResponse`

- `options::BarsResponse`
- `options::TradesResponse`
- `options::LatestQuotesResponse`
- `options::LatestTradesResponse`
- `options::SnapshotsResponse`
- `options::ChainResponse`

- `crypto::BarsResponse`
- `crypto::QuotesResponse`
- `crypto::TradesResponse`
- `crypto::LatestBarsResponse`
- `crypto::LatestQuotesResponse`
- `crypto::LatestTradesResponse`
- `crypto::LatestOrderbooksResponse`
- `crypto::SnapshotsResponse`

- `news::ListResponse`
- `corporate_actions::ListResponse`

响应对象约束：

- 字段名严格使用官方响应原词
- 保留官方包裹层级
- 底层方法直接返回官方响应形态对应的 Rust struct
- 如果官方 body 本身就是顶层动态 key JSON object，就直接保留 map 形状，不额外发明 wrapper 字段；例如 `stocks::SnapshotsResponse`、`stocks::ConditionCodesResponse` 和 `stocks::ExchangeCodesResponse` 都保持为 `HashMap` 形状
- 不在底层响应对象里增加额外推导字段

例如：

```rust
pub struct BarsResponse {
    pub bars: std::collections::HashMap<String, Vec<Bar>>,
    pub next_page_token: Option<String>,
}
```

### `*_all` 与 `*_stream`

- `*_all` 返回同名 `Response`
- `*_all` 内部自动循环分页
- `*_all` 最终返回时 `next_page_token` 为 `None`
- `*_stream` 按页返回同名 `Response`
- `*_stream` 不直接拍平成单条 item

## 命名规则

### 模块与类型

- 资源模块承担资源语义
- 类型名尽量短，不重复资源前缀

例如：

- `alpaca_data::stocks::BarsRequest`
- `alpaca_data::options::ChainResponse`

而不是：

- `alpaca_data::request::StockBarsRequest`
- `alpaca_data::model::OptionChainResponse`

### 方法

底层方法优先与官方 endpoint 一一对应：

- `bars`
- `quotes`
- `trades`
- `latest_bars`
- `latest_quotes`
- `latest_trades`
- `snapshots`
- `chain`
- `list`

便利方法统一使用：

- `*_all`
- `*_stream`

### 枚举

枚举名遵循 Rust 风格，枚举值序列化到 HTTP 时严格匹配官方字符串。

例如：

```rust
pub enum Sort {
    Asc,
    Desc,
}
```

序列化后对应：

- `asc`
- `desc`

## 认证策略

`Client` 支持无凭证创建：

```rust
let client = Client::new();
```

认证策略如下：

- `crypto()` 可在无凭证下使用
- 其他需要认证的资源域在请求时检查凭证
- 若当前请求需要凭证但未配置，则返回明确错误

## 运行时架构

### Client 结构

`Client` 内部持有共享运行时状态：

- HTTP client
- 认证配置
- retry 配置
- timeout 配置
- 并发限制器
- endpoint version 路由信息

各资源 client 只是轻量 facade，共享底层 transport 和连接池。

### 传输层

第一版使用可调优的 `reqwest` transport。

要求：

- 连接池复用
- 显式 timeout
- 429 / 5xx 重试
- `Retry-After` 优先
- 支持分页循环
- 不走 `serde_json::Value` 热路径
- 直接 typed deserialize 到对应 `Response`

### API version 路由

内部必须显式维护不同资源域的版本路由：

- Stocks: `/v2/stocks/*`
- Options: `/v1beta1/options/*`
- Crypto: `/v1beta3/crypto/{loc}/*`
- News: `/v1beta1/news`
- Corporate Actions: `/v1/corporate-actions`

## 性能约束

本库的性能设计基线如下：

- 只做 `async`
- 不做全局单例 API
- 不做隐式缓存
- 不在核心层做 Greeks / IV 补算
- 不在热路径使用 `serde_json::Value`
- 优先减少请求数、减少重复分页、减少多余转换

性能优化优先级：

1. 批量请求
2. 正确分页
3. typed deserialize
4. 连接复用
5. 合理并发

## 错误模型

公开顶层错误类型：

```text
alpaca_data::Error
```

至少包含这些错误类别：

- `MissingCredentials`
- `Transport`
- `Timeout`
- `RateLimited`
- `HttpStatus`
- `Deserialize`
- `InvalidRequest`
- `Pagination`

要求：

- 429 错误保留 `Retry-After` 相关信息
- HTTP 状态错误保留状态码和必要上下文
- 缺凭证错误必须明确区分

## 项目结构

公开 API 采用资源域模块组织，内部按资源和 transport 拆分。

```text
src/
├── lib.rs
├── client.rs
├── error.rs
├── auth.rs
├── transport/
│   ├── mod.rs
│   ├── http.rs
│   ├── retry.rs
│   ├── pagination.rs
│   └── rate_limit.rs
├── common/
│   ├── mod.rs
│   ├── enums.rs
│   ├── time.rs
│   └── response.rs
├── stocks/
│   ├── mod.rs
│   ├── client.rs
│   ├── request.rs
│   ├── response.rs
│   ├── model.rs
│   └── enums.rs
├── options/
│   ├── mod.rs
│   ├── client.rs
│   ├── request.rs
│   ├── response.rs
│   ├── model.rs
│   └── enums.rs
├── crypto/
│   ├── mod.rs
│   ├── client.rs
│   ├── request.rs
│   ├── response.rs
│   ├── model.rs
│   └── enums.rs
├── news/
│   ├── mod.rs
│   ├── client.rs
│   ├── request.rs
│   ├── response.rs
│   └── model.rs
└── corporate_actions/
    ├── mod.rs
    ├── client.rs
    ├── request.rs
    ├── response.rs
    └── model.rs
```

公开 API 应主要通过以下路径访问：

- `alpaca_data::Client`
- `alpaca_data::Error`
- `alpaca_data::stocks::*`
- `alpaca_data::options::*`
- `alpaca_data::crypto::*`
- `alpaca_data::news::*`
- `alpaca_data::corporate_actions::*`

## 测试与 Benchmark

### 测试原则

正常、正确场景的测试严禁使用 mock。

只要官方 API key 能覆盖的正确逻辑，必须优先使用真实 Alpaca API 进行验证。

### 真实 API 测试

真实 API 测试覆盖：

- 成功路径
- 真实认证行为
- 真实分页
- 真实字段反序列化
- 真实资源域行为

当前已落地的 live test 命令：

```bash
ALPACA_LIVE_TESTS=1 cargo test --test live_crypto_latest_quotes_smoke -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_batch_historical -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_single_historical -- --nocapture
ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_latest_snapshot -- --nocapture
```

### mock 的使用边界

mock 只允许用于真实 API 难以稳定制造的异常和故障场景，例如：

- 429 / `Retry-After`
- 5xx 重试
- timeout
- 连接中断
- 非法 JSON
- 异常分页 token

mock 不是主测试手段，只是故障注入工具。

### Benchmark 原则

benchmark 以真实 API 为主，用于验证：

- 单请求开销
- 批量 symbol 吞吐
- 分页拉取吞吐
- `*_all` 与 `*_stream` 的端到端表现

本地 micro-benchmark 仅用于补测库自身开销，例如：

- 反序列化成本
- 聚合成本
- 分页拼接成本

当前已落地的本地 benchmark baseline 命令：

```bash
cargo bench --bench shared_core
```

不使用 mock 得出主性能结论。

## 示例

```rust
use alpaca_data::{stocks, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .api_key("APCA-API-KEY-ID")
        .secret_key("APCA-API-SECRET-KEY")
        .build()?;

    let response = client
        .stocks()
        .bars(stocks::BarsRequest {
            symbols: vec!["AAPL".into(), "MSFT".into()],
            timeframe: stocks::TimeFrame::from("1Min"),
            start: None,
            end: None,
            limit: Some(1000),
            adjustment: Some(stocks::Adjustment::raw()),
            feed: Some(stocks::DataFeed::Iex),
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: Some(stocks::Currency::from("USD")),
            page_token: None,
        })
        .await?;

    println!("{}", response.bars.len());

    Ok(())
}
```

## 参考文档

- Alpaca Market Data API overview: https://docs.alpaca.markets/docs/about-market-data-api
- Alpaca API references: https://docs.alpaca.markets/reference/api-references
