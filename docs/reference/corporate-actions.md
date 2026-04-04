---
title: "Corporate Actions"
description: "Corporate actions endpoints. Mirror method:"
---

# Corporate Actions

- Module path: `alpaca_data::corporate_actions`
- Client type: `CorporateActionsClient`
- docs.rs module: [https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/)
- Site rustdoc module: [https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/)
- Scope: Public API surface

Corporate actions endpoints. Mirror method:

## Methods

| Method | Kind | Async | Request | Return | docs.rs | Site rustdoc |
| --- | --- | --- | --- | --- | --- | --- |
| `list` | mirror | yes | `ListRequest` | `Result<ListResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.CorporateActionsClient.html#method.list) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.CorporateActionsClient.html#method.list) |
| `list_all` | convenience | yes | `ListRequest` | `Result<ListResponse, Error>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.CorporateActionsClient.html#method.list_all) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.CorporateActionsClient.html#method.list_all) |
| `list_stream` | convenience | no | `ListRequest` | `ResponseStream<Result<ListResponse, Error>>` | [docs.rs](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.CorporateActionsClient.html#method.list_stream) | [site](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.CorporateActionsClient.html#method.list_stream) |

## Requests

### `CorporateActionType`

- Kind: enum
- Summary: -
- docs.rs: [CorporateActionType](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/enum.CorporateActionType.html)
- Site rustdoc: [CorporateActionType](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/enum.CorporateActionType.html)

| Variant | Official Value |
| --- | --- |
| `ForwardSplit` | `forward_split` |
| `ReverseSplit` | `reverse_split` |
| `UnitSplit` | `unit_split` |
| `StockDividend` | `stock_dividend` |
| `CashDividend` | `cash_dividend` |
| `SpinOff` | `spin_off` |
| `CashMerger` | `cash_merger` |
| `StockMerger` | `stock_merger` |
| `StockAndCashMerger` | `stock_and_cash_merger` |
| `Redemption` | `redemption` |
| `NameChange` | `name_change` |
| `WorthlessRemoval` | `worthless_removal` |
| `RightsDistribution` | `rights_distribution` |
| `ContractAdjustment` | `contract_adjustment` |
| `PartialCall` | `partial_call` |

### `ListRequest`

- Kind: struct
- Summary: -
- docs.rs: [ListRequest](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.ListRequest.html)
- Site rustdoc: [ListRequest](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.ListRequest.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `symbols` | `symbols` | `Option<Vec<String>>` | - |
| `cusips` | `cusips` | `Option<Vec<String>>` | - |
| `types` | `types` | `Option<Vec<CorporateActionType>>` | - |
| `start` | `start` | `Option<String>` | - |
| `end` | `end` | `Option<String>` | - |
| `ids` | `ids` | `Option<Vec<String>>` | - |
| `limit` | `limit` | `Option<u32>` | - |
| `sort` | `sort` | `Option<Sort>` | - |
| `page_token` | `page_token` | `Option<String>` | - |


## Responses

### `ListResponse`

- Kind: struct
- Summary: -
- docs.rs: [ListResponse](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.ListResponse.html)
- Site rustdoc: [ListResponse](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.ListResponse.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `corporate_actions` | `corporate_actions` | `CorporateActions` | - |
| `next_page_token` | `next_page_token` | `Option<String>` | - |


## Models

### `CashDividend`

- Kind: struct
- Summary: -
- docs.rs: [CashDividend](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.CashDividend.html)
- Site rustdoc: [CashDividend](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.CashDividend.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `symbol` | `symbol` | `String` | - |
| `cusip` | `cusip` | `String` | - |
| `rate` | `rate` | `f64` | - |
| `special` | `special` | `bool` | - |
| `foreign` | `foreign` | `bool` | - |
| `process_date` | `process_date` | `String` | - |
| `ex_date` | `ex_date` | `String` | - |
| `record_date` | `record_date` | `Option<String>` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |
| `due_bill_on_date` | `due_bill_on_date` | `Option<String>` | - |
| `due_bill_off_date` | `due_bill_off_date` | `Option<String>` | - |

### `CashMerger`

- Kind: struct
- Summary: -
- docs.rs: [CashMerger](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.CashMerger.html)
- Site rustdoc: [CashMerger](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.CashMerger.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `acquirer_symbol` | `acquirer_symbol` | `Option<String>` | - |
| `acquirer_cusip` | `acquirer_cusip` | `Option<String>` | - |
| `acquiree_symbol` | `acquiree_symbol` | `String` | - |
| `acquiree_cusip` | `acquiree_cusip` | `String` | - |
| `rate` | `rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `effective_date` | `effective_date` | `String` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |

### `CorporateActions`

- Kind: struct
- Summary: -
- docs.rs: [CorporateActions](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.CorporateActions.html)
- Site rustdoc: [CorporateActions](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.CorporateActions.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `forward_splits` | `forward_splits` | `Vec<ForwardSplit>` | - |
| `reverse_splits` | `reverse_splits` | `Vec<ReverseSplit>` | - |
| `unit_splits` | `unit_splits` | `Vec<UnitSplit>` | - |
| `stock_dividends` | `stock_dividends` | `Vec<StockDividend>` | - |
| `cash_dividends` | `cash_dividends` | `Vec<CashDividend>` | - |
| `spin_offs` | `spin_offs` | `Vec<SpinOff>` | - |
| `cash_mergers` | `cash_mergers` | `Vec<CashMerger>` | - |
| `stock_mergers` | `stock_mergers` | `Vec<StockMerger>` | - |
| `stock_and_cash_mergers` | `stock_and_cash_mergers` | `Vec<StockAndCashMerger>` | - |
| `redemptions` | `redemptions` | `Vec<Redemption>` | - |
| `name_changes` | `name_changes` | `Vec<NameChange>` | - |
| `worthless_removals` | `worthless_removals` | `Vec<WorthlessRemoval>` | - |
| `rights_distributions` | `rights_distributions` | `Vec<RightsDistribution>` | - |
| `contract_adjustments` | `contract_adjustments` | `Vec<UnknownCorporateAction>` | - |
| `partial_calls` | `partial_calls` | `Vec<UnknownCorporateAction>` | - |
| `other` | `other` | `BTreeMap<String, Vec<UnknownCorporateAction>>` | - |

### `ForwardSplit`

- Kind: struct
- Summary: -
- docs.rs: [ForwardSplit](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.ForwardSplit.html)
- Site rustdoc: [ForwardSplit](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.ForwardSplit.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `symbol` | `symbol` | `String` | - |
| `cusip` | `cusip` | `String` | - |
| `new_rate` | `new_rate` | `f64` | - |
| `old_rate` | `old_rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `ex_date` | `ex_date` | `String` | - |
| `record_date` | `record_date` | `Option<String>` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |
| `due_bill_redemption_date` | `due_bill_redemption_date` | `Option<String>` | - |

### `NameChange`

- Kind: struct
- Summary: -
- docs.rs: [NameChange](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.NameChange.html)
- Site rustdoc: [NameChange](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.NameChange.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `old_symbol` | `old_symbol` | `String` | - |
| `old_cusip` | `old_cusip` | `String` | - |
| `new_symbol` | `new_symbol` | `String` | - |
| `new_cusip` | `new_cusip` | `String` | - |
| `process_date` | `process_date` | `String` | - |

### `Redemption`

- Kind: struct
- Summary: -
- docs.rs: [Redemption](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.Redemption.html)
- Site rustdoc: [Redemption](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.Redemption.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `symbol` | `symbol` | `String` | - |
| `cusip` | `cusip` | `String` | - |
| `rate` | `rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |

### `ReverseSplit`

- Kind: struct
- Summary: -
- docs.rs: [ReverseSplit](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.ReverseSplit.html)
- Site rustdoc: [ReverseSplit](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.ReverseSplit.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `symbol` | `symbol` | `String` | - |
| `old_cusip` | `old_cusip` | `String` | - |
| `new_cusip` | `new_cusip` | `String` | - |
| `new_rate` | `new_rate` | `f64` | - |
| `old_rate` | `old_rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `ex_date` | `ex_date` | `String` | - |
| `record_date` | `record_date` | `Option<String>` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |

### `RightsDistribution`

- Kind: struct
- Summary: -
- docs.rs: [RightsDistribution](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.RightsDistribution.html)
- Site rustdoc: [RightsDistribution](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.RightsDistribution.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `source_symbol` | `source_symbol` | `String` | - |
| `source_cusip` | `source_cusip` | `String` | - |
| `new_symbol` | `new_symbol` | `String` | - |
| `new_cusip` | `new_cusip` | `String` | - |
| `rate` | `rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `ex_date` | `ex_date` | `String` | - |
| `record_date` | `record_date` | `Option<String>` | - |
| `payable_date` | `payable_date` | `String` | - |
| `expiration_date` | `expiration_date` | `Option<String>` | - |

### `SpinOff`

- Kind: struct
- Summary: -
- docs.rs: [SpinOff](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.SpinOff.html)
- Site rustdoc: [SpinOff](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.SpinOff.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `source_symbol` | `source_symbol` | `String` | - |
| `source_cusip` | `source_cusip` | `String` | - |
| `source_rate` | `source_rate` | `f64` | - |
| `new_symbol` | `new_symbol` | `String` | - |
| `new_cusip` | `new_cusip` | `String` | - |
| `new_rate` | `new_rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `ex_date` | `ex_date` | `String` | - |
| `record_date` | `record_date` | `Option<String>` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |
| `due_bill_redemption_date` | `due_bill_redemption_date` | `Option<String>` | - |

### `StockAndCashMerger`

- Kind: struct
- Summary: -
- docs.rs: [StockAndCashMerger](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.StockAndCashMerger.html)
- Site rustdoc: [StockAndCashMerger](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.StockAndCashMerger.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `acquirer_symbol` | `acquirer_symbol` | `String` | - |
| `acquirer_cusip` | `acquirer_cusip` | `String` | - |
| `acquirer_rate` | `acquirer_rate` | `f64` | - |
| `acquiree_symbol` | `acquiree_symbol` | `String` | - |
| `acquiree_cusip` | `acquiree_cusip` | `String` | - |
| `acquiree_rate` | `acquiree_rate` | `f64` | - |
| `cash_rate` | `cash_rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `effective_date` | `effective_date` | `String` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |

### `StockDividend`

- Kind: struct
- Summary: -
- docs.rs: [StockDividend](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.StockDividend.html)
- Site rustdoc: [StockDividend](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.StockDividend.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `symbol` | `symbol` | `String` | - |
| `cusip` | `cusip` | `String` | - |
| `rate` | `rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `ex_date` | `ex_date` | `String` | - |
| `record_date` | `record_date` | `Option<String>` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |

### `StockMerger`

- Kind: struct
- Summary: -
- docs.rs: [StockMerger](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.StockMerger.html)
- Site rustdoc: [StockMerger](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.StockMerger.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `acquirer_symbol` | `acquirer_symbol` | `String` | - |
| `acquirer_cusip` | `acquirer_cusip` | `String` | - |
| `acquirer_rate` | `acquirer_rate` | `f64` | - |
| `acquiree_symbol` | `acquiree_symbol` | `String` | - |
| `acquiree_cusip` | `acquiree_cusip` | `String` | - |
| `acquiree_rate` | `acquiree_rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `effective_date` | `effective_date` | `String` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |

### `UnitSplit`

- Kind: struct
- Summary: -
- docs.rs: [UnitSplit](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.UnitSplit.html)
- Site rustdoc: [UnitSplit](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.UnitSplit.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `old_symbol` | `old_symbol` | `String` | - |
| `old_cusip` | `old_cusip` | `String` | - |
| `old_rate` | `old_rate` | `f64` | - |
| `new_symbol` | `new_symbol` | `String` | - |
| `new_cusip` | `new_cusip` | `String` | - |
| `new_rate` | `new_rate` | `f64` | - |
| `alternate_symbol` | `alternate_symbol` | `String` | - |
| `alternate_cusip` | `alternate_cusip` | `String` | - |
| `alternate_rate` | `alternate_rate` | `f64` | - |
| `process_date` | `process_date` | `String` | - |
| `effective_date` | `effective_date` | `String` | - |
| `payable_date` | `payable_date` | `Option<String>` | - |

### `UnknownCorporateAction`

- Kind: type
- Summary: -
- docs.rs: [UnknownCorporateAction](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/type.UnknownCorporateAction.html)
- Site rustdoc: [UnknownCorporateAction](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/type.UnknownCorporateAction.html)
- Alias target: `BTreeMap<String, serde_json::Value>`

### `WorthlessRemoval`

- Kind: struct
- Summary: -
- docs.rs: [WorthlessRemoval](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/struct.WorthlessRemoval.html)
- Site rustdoc: [WorthlessRemoval](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/struct.WorthlessRemoval.html)

| Field | Official Word | Type | Notes |
| --- | --- | --- | --- |
| `id` | `id` | `String` | - |
| `symbol` | `symbol` | `String` | - |
| `cusip` | `cusip` | `String` | - |
| `process_date` | `process_date` | `String` | - |


## Enums

### `Sort`

- Kind: enum
- Summary: -
- docs.rs: [Sort](https://docs.rs/alpaca-data/latest/alpaca_data/corporate_actions/enum.Sort.html)
- Site rustdoc: [Sort](https://wmzhai.github.io/alpaca-data-rs/api/alpaca_data/corporate_actions/enum.Sort.html)

| Variant | Official Value |
| --- | --- |
| `Asc` | `asc` |
| `Desc` | `desc` |


## Related Repository Artifacts

- Examples: `examples/corporate_actions_list.rs`
- Tests: `tests/live_corporate_actions.rs`, `tests/mock_news_corporate_actions_errors.rs`
- Benchmarks: `benches/news_corporate_actions.rs`

## Coverage Notes

- The strict endpoint parity ledger for this module lives in [API Coverage](../api-coverage.md).
- Generated reference pages mirror the shipped Rust surface and do not claim unimplemented Alpaca endpoints as available.
