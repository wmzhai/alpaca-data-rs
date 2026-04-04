---
name: alpaca-market-data-sync
description: Use when auditing or updating the local alpaca-data Market Data mirror against the latest official Alpaca Market Data HTTP API, especially before release work or API parity work
---

# Alpaca Market Data Sync

## Overview

Audit local API parity against the official Alpaca Market Data HTTP API.

The official HTTP API is the only semantic standard. The mirror layer is checked first. The convenience layer is derivative and is only re-validated after mirror drift is resolved.

## Required Sources

Always use these inputs together:

- Official reference root: `https://docs.alpaca.markets/reference/api-references`
- Official OpenAPI: `https://docs.alpaca.markets/openapi/market-data-api.json`
- Local human contract: `docs/api-coverage.md`
- Local machine contract: `tools/api-coverage/market-data-api.json`

Do not use third-party SDKs or secondary docs as semantic authority.

## Audit Order

1. Read `tools/api-coverage/market-data-api.json` and `docs/api-coverage.md`.
2. Fetch the latest official OpenAPI and inspect the relevant reference pages.
3. Compare official paths, operation IDs, tags, parameters, enums, and response fields against the local coverage contract and the real Rust implementation under `src/`.
4. Classify every difference into one of these buckets:
   - missing mirror endpoint in an adopted family
   - new official resource family not yet adopted locally
   - parameter drift
   - response-field drift
   - convenience-layer compatibility note
5. Fix or recommend mirror-layer changes first.
6. Only after mirror parity is restored, re-check `*_all` and `*_stream` compatibility for the affected resource.
7. Sync `docs/api-coverage.md`, `tools/api-coverage/market-data-api.json`, `README.md`, `CHANGELOG.md`, and affected `memory/` docs to the new reality.

## Suggested Commands

Use the repository audit entry point first:

```bash
./scripts/api-sync-audit
```

If you need to inspect the raw source separately, use the official OpenAPI directly. A minimal path inventory looks like:

```bash
curl -fsSL https://docs.alpaca.markets/openapi/market-data-api.json \
  | jq -r '.paths | to_entries[] | [.key, (.value.get.operationId // ""), ((.value.get.tags // []) | join(","))] | @tsv'
```

Inspect the local machine contract with:

```bash
jq '.' tools/api-coverage/market-data-api.json
```

When code changes are needed, inspect:

- `src/transport/endpoint.rs`
- the affected resource module under `src/`
- `tests/public_api.rs`
- the affected live and mock tests under `tests/`

## Expected Audit Output

Every audit result should report:

- official source date and URL set
- affected official path or family
- local mirror method or missing method
- drift type
- whether the drift blocks release parity
- whether convenience helpers are now incompatible or need re-validation
- recommended next action

Prefer a short severity-ordered report. Mirror drift is higher priority than convenience notes.

## Baseline Gaps To Keep Surfacing

Unless they have been implemented already, the audit should continue to report:

- `StockAuctions` at `/v2/stocks/auctions`
- `StockAuctionSingle` at `/v2/stocks/{symbol}/auctions`
- `OptionMetaConditions` at `/v1beta1/options/meta/conditions/{ticktype}`
- official crypto `loc` enum values `us-2` and `bs-1` missing from local `crypto::Loc`

## Fix Order

When a drift is confirmed:

1. Update the mirror layer first.
2. Update request/response models so official words and enums remain exact.
3. Update route coverage and the affected real happy-path tests.
4. Update the coverage contract files.
5. Re-check convenience helpers for the same resource.
6. Only then claim parity.

Do not treat convenience compatibility as valid while mirror drift is still open.

## Release Use

Run this audit before:

- `Phase 6` or `Phase 7` release preparation work
- any task that claims official API parity
- any version bump intended to tighten coverage or release readiness

If the audit finds only documentation drift, update the docs. If it finds mirror drift, fix the code first.
