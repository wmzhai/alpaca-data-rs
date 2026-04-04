# Examples

This document describes the runnable example surface for `alpaca-data`.

## Example Style

Examples in this repository follow these rules:

- async-only
- minimal happy-path code
- official method names
- no extra abstraction layer
- environment-driven authentication

## Example Set

| Example | Focus | Auth |
| --- | --- | --- |
| [`client_builder.rs`](../examples/client_builder.rs) | client construction and runtime tuning | optional |
| [`stocks_latest_bar.rs`](../examples/stocks_latest_bar.rs) | authenticated stock latest data | required |
| [`stocks_bars_all.rs`](../examples/stocks_bars_all.rs) | authenticated stock pagination helper | required |
| [`options_chain.rs`](../examples/options_chain.rs) | authenticated options chain snapshots | required |
| [`crypto_latest_quotes.rs`](../examples/crypto_latest_quotes.rs) | public crypto latest quotes | optional |
| [`news_list.rs`](../examples/news_list.rs) | authenticated news listing | required |
| [`corporate_actions_list.rs`](../examples/corporate_actions_list.rs) | authenticated corporate actions listing | required |

## Run Examples

Examples that hit authenticated endpoints expect:

- `APCA_API_KEY_ID`
- `APCA_API_SECRET_KEY`

Optional override:

- `APCA_API_DATA_URL`

Run an example with:

```bash
cargo run --example client_builder
cargo run --example stocks_latest_bar
cargo run --example stocks_bars_all
cargo run --example options_chain
cargo run --example crypto_latest_quotes
cargo run --example news_list
cargo run --example corporate_actions_list
```
