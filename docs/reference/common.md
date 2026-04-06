---
title: "Common"
description: "Shared query, response, enum, and timestamp primitives used across resource modules."
---

# Common

- Module path: `alpaca_data::common`
- Client type: internal-only module
- docs.rs module: not linked; this is an internal-only generated page and the crate does not ship a public rustdoc module at this path.
- Site rustdoc module: not linked; this is an internal-only generated page and the site does not publish a public rustdoc module at this path.
- Scope: Internal architecture support only

Shared query, response, enum, and timestamp primitives used across resource modules.

## Shared Types

### `Currency`

- Kind: struct
- Summary: -
- docs.rs: not linked; no canonical public rustdoc path is shipped for this item.
- Site rustdoc: not linked; no canonical public rustdoc path is shipped for this item.
- Example constructors: `usd -> USD`

### `Sort`

- Kind: enum
- Summary: -
- docs.rs: not linked; no canonical public rustdoc path is shipped for this item.
- Site rustdoc: not linked; no canonical public rustdoc path is shipped for this item.

| Variant | Official Value |
| --- | --- |
| `Asc` | `asc` |
| `Desc` | `desc` |

### `Timestamp`

- Kind: type
- Summary: -
- docs.rs: not linked; no canonical public rustdoc path is shipped for this item.
- Site rustdoc: not linked; no canonical public rustdoc path is shipped for this item.
- Alias target: `String`


## Related Repository Artifacts

- Examples: -
- Tests: `tests/public_api.rs`
- Benchmarks: -

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
