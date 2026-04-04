---
title: "Documentation Home"
description: "Public documentation home for the alpaca-data Rust client."
sidebar_position: 1
---

# Documentation Home

`alpaca-data` is an async Rust client for the Alpaca Market Data HTTP API. This site is the public narrative companion to the crate, while API deep links point to both `docs.rs` and the project-hosted rustdoc bundle.

## Site Map

- Start with [Getting Started](./getting-started.md) for install, client construction, and the two-layer API model.
- Use [Authentication](./authentication.md) for current credential behavior and live-test environment setup.
- Read [Layers](./layers.md) for the mirror-versus-convenience contract.
- Browse [Project Structure](./project-structure.md) for the repository layout and generated structure inventory.
- Use [API Reference](./reference/index.md) for module-level public API pages with rustdoc deep links.
- Use [API Coverage](./api-coverage.md) for the strict endpoint-to-method parity ledger.
- Use [Examples](./examples.md) for runnable entry points.
- Use [Release Checklist](./release-checklist.md) for release-readiness validation.

## Root Entry Point

- Crate: `alpaca-data`
- Version: `v0.8.0`
- Root client: `alpaca_data::Client`
- Resource accessors: `stocks()`, `options()`, `crypto()`, `news()`, `corporate_actions()`
- Documentation hosts:
  - Site: `https://wmzhai.github.io/alpaca-data-rs/`
  - docs.rs: `https://docs.rs/alpaca-data`
  - Repository: `https://github.com/wmzhai/alpaca-data-rs`

## Resource Families

| Resource | Client | Mirror Methods | Convenience Methods | Examples | Tests |
| --- | --- | ---: | ---: | ---: | ---: |
| [Stocks](./reference/stocks.md) | `StocksClient` | 16 | 12 | 2 | 5 |
| [Options](./reference/options.md) | `OptionsClient` | 7 | 8 | 1 | 4 |
| [Crypto](./reference/crypto.md) | `CryptoClient` | 8 | 6 | 1 | 5 |
| [News](./reference/news.md) | `NewsClient` | 1 | 2 | 1 | 2 |
| [Corporate Actions](./reference/corporate-actions.md) | `CorporateActionsClient` | 1 | 2 | 1 | 2 |

## API Layers

- Mirror layer methods preserve the official HTTP endpoint shape and field words.
- Convenience helpers only add `*_all` and `*_stream` pagination flows.
- Request and response field words stay aligned with Alpaca's official API.
- Public Rust naming stays idiomatic: modules lowercase, types `PascalCase`, methods and fields `snake_case`.

## Documentation Generation

The generated pages under `docs/reference/`, `docs/project-structure.md`, `docs/generated/`, and `website/sidebars.ts` are produced by `./tools/docs/generate-doc-site`.

The generated site is designed for GitHub Pages project hosting at `https://wmzhai.github.io/alpaca-data-rs/`.
