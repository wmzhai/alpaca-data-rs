---
title: "Client"
description: "Root client and builder reference."
sidebar_position: 2
---

# Client

- Module path: `alpaca_data`
- docs.rs: [https://docs.rs/alpaca-data](https://docs.rs/alpaca-data)
- Site rustdoc root: [https://wmzhai.github.io/alpaca-data-rs/api/](https://wmzhai.github.io/alpaca-data-rs/api/)

## Public Types

| Type | Kind | Summary | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- |
| `Client` | struct | Root async client for Alpaca Market Data HTTP APIs. Build a client once, then obtain resource clients with [`Self::stocks`], [`Self::options`], [`Self::crypto`], [`Self::news`], and [`Self::corporate_actions`]. # Examples ```no_run use alpaca_data::Client; let client = Client::builder().build()?; # let _ = client; # Ok::<(), alpaca_data::Error>(()) ``` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html) |
| `ClientBuilder` | struct | - | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html) |

## `Client` Methods

| Method | Kind | Async | Request | Return | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- | --- | --- |
| `new` | builder | no | - | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html#method.new) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html#method.new) |
| `builder` | builder | no | - | `ClientBuilder` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html#method.builder) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html#method.builder) |
| `stocks` | accessor | no | - | `StocksClient` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html#method.stocks) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html#method.stocks) |
| `options` | accessor | no | - | `OptionsClient` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html#method.options) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html#method.options) |
| `crypto` | accessor | no | - | `CryptoClient` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html#method.crypto) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html#method.crypto) |
| `news` | accessor | no | - | `NewsClient` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html#method.news) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html#method.news) |
| `corporate_actions` | accessor | no | - | `CorporateActionsClient` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.Client.html#method.corporate_actions) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.Client.html#method.corporate_actions) |

## `ClientBuilder` Methods

| Method | Kind | Async | Request | Return | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- | --- | --- |
| `api_key` | builder | no | `impl Into<String>` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.api_key) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.api_key) |
| `secret_key` | builder | no | `impl Into<String>` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.secret_key) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.secret_key) |
| `base_url` | builder | no | `impl Into<String>` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.base_url) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.base_url) |
| `timeout` | builder | no | `Duration` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.timeout) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.timeout) |
| `reqwest_client` | builder | no | `reqwest::Client` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.reqwest_client) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.reqwest_client) |
| `max_retries` | builder | no | `u32` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.max_retries) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.max_retries) |
| `retry_on_429` | builder | no | `bool` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.retry_on_429) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.retry_on_429) |
| `respect_retry_after` | builder | no | `bool` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.respect_retry_after) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.respect_retry_after) |
| `base_backoff` | builder | no | `Duration` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.base_backoff) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.base_backoff) |
| `max_backoff` | builder | no | `Duration` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.max_backoff) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.max_backoff) |
| `retry_jitter` | builder | no | `Duration` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.retry_jitter) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.retry_jitter) |
| `total_retry_budget` | builder | no | `Duration` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.total_retry_budget) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.total_retry_budget) |
| `max_in_flight` | builder | no | `usize` | `Self` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.max_in_flight) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.max_in_flight) |
| `build` | builder | no | - | `Result<Client, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/struct.ClientBuilder.html#method.build) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/struct.ClientBuilder.html#method.build) |
