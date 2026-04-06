---
title: "Common"
description: "Shared query, response, enum, and timestamp primitives used across resource modules."
---

# Common

- Module path: `alpaca_data::common`
- Client type: internal-only module
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/common/](https://docs.rs/alpaca-data/latest/alpaca_data/common/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/common/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/common/)
- Scope: Internal architecture support only

Shared query, response, enum, and timestamp primitives used across resource modules.

## Shared Types

### `Currency`

- Kind: struct
- Summary: -
- docs.rs: [Currency](https://docs.rs/alpaca-data/latest/alpaca_data/common/struct.Currency.html)
- Site rustdoc: [Currency](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/common/struct.Currency.html)
- Example constructors: `usd -> USD`

### `Sort`

- Kind: enum
- Summary: -
- docs.rs: [Sort](https://docs.rs/alpaca-data/latest/alpaca_data/common/enum.Sort.html)
- Site rustdoc: [Sort](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/common/enum.Sort.html)

| Variant | Official Value |
| --- | --- |
| `Asc` | `asc` |
| `Desc` | `desc` |

### `Timestamp`

- Kind: type
- Summary: -
- docs.rs: [Timestamp](https://docs.rs/alpaca-data/latest/alpaca_data/common/type.Timestamp.html)
- Site rustdoc: [Timestamp](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/common/type.Timestamp.html)
- Alias target: `String`


## Related Repository Artifacts

- Examples: -
- Tests: `tests/public_api.rs`
- Benchmarks: -

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
