# API Coverage

This document records how the local `alpaca-data` API maps to the official Alpaca Market Data HTTP API.

## Source Of Truth

This coverage contract is based on:

- Official reference root: `https://docs.alpaca.markets/reference/api-references`
- Official OpenAPI: `https://docs.alpaca.markets/openapi/market-data-api.json`
- Official OpenAPI title: `Market Data API`
- Official OpenAPI version observed on 2026-04-04: `1.1`

The machine-readable companion file is:

- `tools/api-coverage/market-data-api.json`

## Interpretation Rules

- Mirror layer means a local method that directly corresponds to an official HTTP endpoint.
- Convenience layer means a local helper such as `*_all` or `*_stream` that automates pagination on top of a mirror method.
- The convenience layer is not counted as official endpoint coverage.
- Out-of-scope entries are split into:
  - gaps inside already adopted resource families
  - official Market Data resource families not yet adopted by the local crate

## Coverage Summary

As of 2026-04-04:

- Official OpenAPI paths in the Market Data API: `47`
- Official paths in the adopted local resource families (`stocks`, `options`, `crypto`, `news`, `corporate_actions`): `36`
- Implemented local mirror paths in those adopted families: `33`
- Known open gaps inside adopted families: `3`

Open gaps inside adopted families:

- `StockAuctions` at `/v2/stocks/auctions`
- `StockAuctionSingle` at `/v2/stocks/{symbol}/auctions`
- `OptionMetaConditions` at `/v1beta1/options/meta/conditions/{ticktype}`

Known parity gap inside an implemented family:

- Official crypto `loc` currently includes `us-2` and `bs-1`, but local `crypto::Loc` currently exposes only `us`, `us-1`, and `eu-1`

Official Market Data families not yet adopted locally:

- `Forex`
- `Logos`
- `Fixed income`
- `Screener`
- `Crypto perpetual futures`

## Mirror Layer

### Stocks

| Official path | Operation ID | Local mirror method | Convenience helpers | Status |
| --- | --- | --- | --- | --- |
| `/v2/stocks/bars` | `StockBars` | `stocks().bars` | `bars_all`, `bars_stream` | Implemented |
| `/v2/stocks/{symbol}/bars` | `StockBarSingle` | `stocks().bars_single` | `bars_single_all`, `bars_single_stream` | Implemented |
| `/v2/stocks/quotes` | `StockQuotes` | `stocks().quotes` | `quotes_all`, `quotes_stream` | Implemented |
| `/v2/stocks/{symbol}/quotes` | `StockQuoteSingle` | `stocks().quotes_single` | `quotes_single_all`, `quotes_single_stream` | Implemented |
| `/v2/stocks/trades` | `StockTrades` | `stocks().trades` | `trades_all`, `trades_stream` | Implemented |
| `/v2/stocks/{symbol}/trades` | `StockTradeSingle` | `stocks().trades_single` | `trades_single_all`, `trades_single_stream` | Implemented |
| `/v2/stocks/bars/latest` | `StockLatestBars` | `stocks().latest_bars` | None | Implemented |
| `/v2/stocks/{symbol}/bars/latest` | `StockLatestBarSingle` | `stocks().latest_bar` | None | Implemented |
| `/v2/stocks/quotes/latest` | `StockLatestQuotes` | `stocks().latest_quotes` | None | Implemented |
| `/v2/stocks/{symbol}/quotes/latest` | `StockLatestQuoteSingle` | `stocks().latest_quote` | None | Implemented |
| `/v2/stocks/trades/latest` | `StockLatestTrades` | `stocks().latest_trades` | None | Implemented |
| `/v2/stocks/{symbol}/trades/latest` | `StockLatestTradeSingle` | `stocks().latest_trade` | None | Implemented |
| `/v2/stocks/snapshots` | `StockSnapshots` | `stocks().snapshots` | None | Implemented |
| `/v2/stocks/{symbol}/snapshot` | `StockSnapshotSingle` | `stocks().snapshot` | None | Implemented |
| `/v2/stocks/meta/conditions/{ticktype}` | `StockMetaConditions` | `stocks().condition_codes` | None | Implemented |
| `/v2/stocks/meta/exchanges` | `StockMetaExchanges` | `stocks().exchange_codes` | None | Implemented |
| `/v2/stocks/auctions` | `StockAuctions` | None | None | Gap in adopted family |
| `/v2/stocks/{symbol}/auctions` | `StockAuctionSingle` | None | None | Gap in adopted family |

### Options

| Official path | Operation ID | Local mirror method | Convenience helpers | Status |
| --- | --- | --- | --- | --- |
| `/v1beta1/options/bars` | `optionBars` | `options().bars` | `bars_all`, `bars_stream` | Implemented |
| `/v1beta1/options/trades` | `OptionTrades` | `options().trades` | `trades_all`, `trades_stream` | Implemented |
| `/v1beta1/options/quotes/latest` | `OptionLatestQuotes` | `options().latest_quotes` | None | Implemented |
| `/v1beta1/options/trades/latest` | `OptionLatestTrades` | `options().latest_trades` | None | Implemented |
| `/v1beta1/options/snapshots` | `OptionSnapshots` | `options().snapshots` | `snapshots_all`, `snapshots_stream` | Implemented |
| `/v1beta1/options/snapshots/{underlying_symbol}` | `OptionChain` | `options().chain` | `chain_all`, `chain_stream` | Implemented |
| `/v1beta1/options/meta/exchanges` | `OptionMetaExchanges` | `options().exchange_codes` | None | Implemented |
| `/v1beta1/options/meta/conditions/{ticktype}` | `OptionMetaConditions` | None | None | Gap in adopted family |

### Crypto

| Official path | Operation ID | Local mirror method | Convenience helpers | Status |
| --- | --- | --- | --- | --- |
| `/v1beta3/crypto/{loc}/bars` | `CryptoBars` | `crypto().bars` | `bars_all`, `bars_stream` | Implemented |
| `/v1beta3/crypto/{loc}/quotes` | `CryptoQuotes` | `crypto().quotes` | `quotes_all`, `quotes_stream` | Implemented |
| `/v1beta3/crypto/{loc}/trades` | `CryptoTrades` | `crypto().trades` | `trades_all`, `trades_stream` | Implemented |
| `/v1beta3/crypto/{loc}/latest/bars` | `CryptoLatestBars` | `crypto().latest_bars` | None | Implemented |
| `/v1beta3/crypto/{loc}/latest/quotes` | `CryptoLatestQuotes` | `crypto().latest_quotes` | None | Implemented |
| `/v1beta3/crypto/{loc}/latest/trades` | `CryptoLatestTrades` | `crypto().latest_trades` | None | Implemented |
| `/v1beta3/crypto/{loc}/latest/orderbooks` | `CryptoLatestOrderbooks` | `crypto().latest_orderbooks` | None | Implemented |
| `/v1beta3/crypto/{loc}/snapshots` | `CryptoSnapshots` | `crypto().snapshots` | None | Implemented |

### News

| Official path | Operation ID | Local mirror method | Convenience helpers | Status |
| --- | --- | --- | --- | --- |
| `/v1beta1/news` | `News` | `news().list` | `list_all`, `list_stream` | Implemented |

### Corporate Actions

| Official path | Operation ID | Local mirror method | Convenience helpers | Status |
| --- | --- | --- | --- | --- |
| `/v1/corporate-actions` | `CorporateActions` | `corporate_actions().list` | `list_all`, `list_stream` | Implemented |

## Convenience Layer

The local convenience layer adds helpers on top of paginated official endpoints.

| Resource | Mirror method | Convenience helpers |
| --- | --- | --- |
| `stocks` | `bars` | `bars_all`, `bars_stream` |
| `stocks` | `bars_single` | `bars_single_all`, `bars_single_stream` |
| `stocks` | `quotes` | `quotes_all`, `quotes_stream` |
| `stocks` | `quotes_single` | `quotes_single_all`, `quotes_single_stream` |
| `stocks` | `trades` | `trades_all`, `trades_stream` |
| `stocks` | `trades_single` | `trades_single_all`, `trades_single_stream` |
| `options` | `bars` | `bars_all`, `bars_stream` |
| `options` | `trades` | `trades_all`, `trades_stream` |
| `options` | `snapshots` | `snapshots_all`, `snapshots_stream` |
| `options` | `chain` | `chain_all`, `chain_stream` |
| `crypto` | `bars` | `bars_all`, `bars_stream` |
| `crypto` | `quotes` | `quotes_all`, `quotes_stream` |
| `crypto` | `trades` | `trades_all`, `trades_stream` |
| `news` | `list` | `list_all`, `list_stream` |
| `corporate_actions` | `list` | `list_all`, `list_stream` |

## Official Endpoints Not Yet Adopted Locally

These official Market Data families are present in the OpenAPI but remain outside the current local crate scope.

| Official tag | Official paths |
| --- | --- |
| `Fixed income` | `/v1beta1/fixed_income/latest/prices` |
| `Forex` | `/v1beta1/forex/latest/rates`, `/v1beta1/forex/rates` |
| `Logos` | `/v1beta1/logos/{symbol}` |
| `Screener` | `/v1beta1/screener/stocks/most-actives`, `/v1beta1/screener/{market_type}/movers` |
| `Crypto perpetual futures` | `/v1beta1/crypto-perps/{loc}/latest/bars`, `/v1beta1/crypto-perps/{loc}/latest/pricing`, `/v1beta1/crypto-perps/{loc}/latest/orderbooks`, `/v1beta1/crypto-perps/{loc}/latest/quotes`, `/v1beta1/crypto-perps/{loc}/latest/trades` |

## Known Parity Gaps Inside Implemented Families

| Kind | Local area | Official fact | Local fact | Status |
| --- | --- | --- | --- | --- |
| Parameter enum gap | `crypto::Loc` | Official crypto `loc` includes `us`, `us-1`, `us-2`, `eu-1`, `bs-1` | Local `crypto::Loc` currently exposes `us`, `us-1`, `eu-1` | Open |

## Release Interpretation

This document is intentionally strict:

- Implemented means the local crate exposes a mirror-layer method for that official HTTP endpoint.
- Gap means the official endpoint exists in an adopted local resource family but is not yet implemented.
- Not adopted means the official family exists in Alpaca Market Data, but the local crate has not added that resource family yet.

The machine-readable manifest is the source that future API audit tooling should consume first.
