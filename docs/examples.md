# Examples

This document describes the runnable example surface for `alpaca-data`.

## Example Style

Examples in this repository follow these rules:

- async-only
- minimal happy-path code
- official method names
- no extra abstraction layer
- environment-driven authentication

## Planned Example Set

- [`client_builder.rs`](../examples/client_builder.rs)
- [`stocks_latest_bar.rs`](../examples/stocks_latest_bar.rs)
- [`stocks_bars_all.rs`](../examples/stocks_bars_all.rs)
- [`options_chain.rs`](../examples/options_chain.rs)
- [`crypto_latest_quotes.rs`](../examples/crypto_latest_quotes.rs)
- [`news_list.rs`](../examples/news_list.rs)
- [`corporate_actions_list.rs`](../examples/corporate_actions_list.rs)

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
