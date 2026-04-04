---
title: "News"
description: "News endpoints. Mirror method:"
---

# News

- Module path: `alpaca_data::news`
- Client type: `NewsClient`
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/news/](https://docs.rs/alpaca-data/latest/alpaca_data/news/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/)
- Scope: Public API surface

News endpoints. Mirror method:

## Methods

| Method | Kind | Async | Request | Return | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- | --- | --- |
| `list` | mirror | yes | `ListRequest` | `Result<ListResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/news/struct.NewsClient.html#method.list) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/struct.NewsClient.html#method.list) |
| `list_all` | convenience | yes | `ListRequest` | `Result<ListResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/news/struct.NewsClient.html#method.list_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/struct.NewsClient.html#method.list_all) |
| `list_stream` | convenience | no | `ListRequest` | `ResponseStream<Result<ListResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/news/struct.NewsClient.html#method.list_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/struct.NewsClient.html#method.list_stream) |

## Requests

### `ListRequest`

- Kind: struct
- Summary: -
- docs.rs: [ListRequest](https://docs.rs/alpaca-data/latest/alpaca_data/news/struct.ListRequest.html)
- Site rustdoc: [ListRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/struct.ListRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `symbols` | `symbols` | `Option<Vec<String>>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `include_content` | `include_content` | `Option<bool>` | - |
| `exclude_contentless` | `exclude_contentless` | `Option<bool>` | - |
| `page_token` | `page_token` | `Option<String>` | - |


## Responses

### `ListResponse`

- Kind: struct
- Summary: -
- docs.rs: [ListResponse](https://docs.rs/alpaca-data/latest/alpaca_data/news/struct.ListResponse.html)
- Site rustdoc: [ListResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/struct.ListResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `news` | `news` | `Vec<NewsItem>` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |


## Models

### `NewsImage`

- Kind: struct
- Summary: -
- docs.rs: [NewsImage](https://docs.rs/alpaca-data/latest/alpaca_data/news/struct.NewsImage.html)
- Site rustdoc: [NewsImage](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/struct.NewsImage.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `size` | `size` | `String` | - |
| `url` | `url` | `String` | - |

### `NewsItem`

- Kind: struct
- Summary: -
- docs.rs: [NewsItem](https://docs.rs/alpaca-data/latest/alpaca_data/news/struct.NewsItem.html)
- Site rustdoc: [NewsItem](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/struct.NewsItem.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `i64` | - |
| `headline` | `headline` | `String` | - |
| `author` | `author` | `String` | - |
| `created_at` | `created_at` | `String` | - |
| `updated_at` | `updated_at` | `String` | - |
| `summary` | `summary` | `String` | - |
| `content` | `content` | `String` | - |
| `url` | `url` | `Option<String>` | - |
| `images` | `images` | `Vec<NewsImage>` | - |
| `symbols` | `symbols` | `Vec<String>` | - |
| `source` | `source` | `String` | - |


## Enums

### `Sort`

- Kind: enum
- Summary: -
- docs.rs: [Sort](https://docs.rs/alpaca-data/latest/alpaca_data/news/enum.Sort.html)
- Site rustdoc: [Sort](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/news/enum.Sort.html)

| Variant | Official Value |
| --- | --- |
| `Asc` | `asc` |
| `Desc` | `desc` |


## Related Repository Artifacts

- Examples: `examples/news_list.rs`
- Tests: `tests/live_news.rs`, `tests/mock_news_corporate_actions_errors.rs`
- Benchmarks: `benches/news_corporate_actions.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
