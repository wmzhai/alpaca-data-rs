---
title: "API Reference Index"
description: "Module-by-module reference index for alpaca-data."
sidebar_position: 1
---

# API Reference

## Root Client

- Type: `alpaca_data::Client`
- Builder: `alpaca_data::ClientBuilder`
- Accessors: `stocks()`, `options()`, `crypto()`, `news()`, `corporate_actions()`
- [Client reference](./client.md)

## Module Index

| Module | Path | Client | Mirror Methods | Convenience Methods | Surface |
| --- | --- | --- | ---: | ---: | --- |
| [Stocks](./stocks.md) | `alpaca_data::stocks` | `StocksClient` | 18 | 16 | Public |
| [Options](./options.md) | `alpaca_data::options` | `OptionsClient` | 8 | 8 | Public |
| [Crypto](./crypto.md) | `alpaca_data::crypto` | `CryptoClient` | 8 | 6 | Public |
| [News](./news.md) | `alpaca_data::news` | `NewsClient` | 1 | 2 | Public |
| [Corporate Actions](./corporate-actions.md) | `alpaca_data::corporate_actions` | `CorporateActionsClient` | 1 | 2 | Public |
| [Common](./common.md) | `alpaca_data::common` | `internal` | 0 | 0 | Internal only |
| [Transport](./transport.md) | `alpaca_data::transport` | `internal` | 0 | 0 | Internal only |
