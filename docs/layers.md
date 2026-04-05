# Layers

`alpaca-data` intentionally exposes two layers.

## Mirror Layer

The mirror layer maps the official Alpaca Market Data HTTP API into Rust.

Examples:

- `stocks().bars(...)`
- `stocks().latest_quote(...)`
- `options().chain(...)`
- `crypto().latest_orderbooks(...)`
- `news().list(...)`
- `corporate_actions().list(...)`

Mirror-layer rules:

- endpoint semantics come from the official HTTP API
- request and response field words use the official API terms
- wrapper shapes such as `next_page_token` are preserved
- documented request-shape violations fail fast with `Error::InvalidRequest`

## Convenience Layer

The convenience layer adds only pagination helpers:

- `*_all`
- `*_stream`
- plus `*_single_all` and `*_single_stream` where a dedicated single-symbol historical endpoint exists

Examples:

- `stocks().bars_all(...)`
- `stocks().bars_single_stream(...)`
- `options().chain_all(...)`
- `news().list_stream(...)`
- `corporate_actions().list_all(...)`

Convenience-layer rules:

- only added for endpoints that already have pagination semantics
- response field words are unchanged
- `*_all` returns the same response type with aggregated pages
- `*_stream` yields pages, not flattened items
- documented hard-limit violations do not trigger silent auto-chunking

## Why This Split Exists

This split keeps two goals compatible:

- strict fidelity to the official HTTP API
- ergonomic Rust helpers for common pagination workflows

The mirror layer remains the semantic source of truth inside the crate.

For the current endpoint-by-endpoint mapping, see [api-coverage.md](api-coverage.md).
