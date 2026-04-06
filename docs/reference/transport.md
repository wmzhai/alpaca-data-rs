---
title: "Transport"
description: "Shared HTTP transport, endpoint routing, pagination, retry, and rate-limit infrastructure."
---

# Transport

- Module path: `alpaca_data::transport`
- Client type: internal-only module
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/transport/](https://docs.rs/alpaca-data/latest/alpaca_data/transport/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/)
- Scope: Internal architecture support only

Shared HTTP transport, endpoint routing, pagination, retry, and rate-limit infrastructure.

## Shared Types

### `ObservedResponseMeta`

- Kind: struct
- Summary: Read-only metadata emitted to [`TransportObserver`] implementations.
- docs.rs: [ObservedResponseMeta](https://docs.rs/alpaca-data/latest/alpaca_data/transport/struct.ObservedResponseMeta.html)
- Site rustdoc: [ObservedResponseMeta](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/transport/struct.ObservedResponseMeta.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `endpoint_name` | `endpoint_name` | `&'static str` | Stable endpoint name such as `stocks.latest_bars`. |
| `url` | `url` | `String` | Fully resolved request URL. |
| `status` | `status` | `u16` | Final HTTP status code. |
| `request_id` | `request_id` | `Option<String>` | Alpaca request identifier when the server returned one. |
| `attempt_count` | `attempt_count` | `u32` | Number of retry attempts that happened before this terminal response. |
| `elapsed` | `elapsed` | `Duration` | Total elapsed request time across retries. |


## Related Repository Artifacts

- Examples: -
- Tests: `tests/mock_transport_errors.rs`
- Benchmarks: `benches/shared_core.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
