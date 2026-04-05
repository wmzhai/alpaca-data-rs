---
title: "Documentation"
description: "Documentation for the alpaca-data Rust client."
sidebar_position: 1
---

# Documentation

`alpaca-data` is a high-performance async Rust client for the Alpaca Market Data HTTP API. This site covers crate usage, endpoint coverage, repository layout, examples, and rustdoc links.

## Start Here

- Start with [Getting Started](./getting-started.md) for install, client construction, and the two-layer API model.
- Use [Authentication](./authentication.md) for current credential behavior and live-test environment setup.
- Read [Layers](./layers.md) for the mirror-versus-convenience contract.
- Browse [Project Structure](./project-structure.md) for the repository layout.
- Use [API Reference](./reference/index.md) for module-level public API pages with rustdoc deep links.
- Use [API Coverage](./api-coverage.md) for the endpoint-to-method coverage table.
- Use [Examples](./examples.md) for runnable entry points.
- Use [Release Checklist](./release-checklist.md) for release-readiness validation.

## Root Entry Point

- Crate: `alpaca-data`
- Version: `v0.10.2`
- Root client: `alpaca_data::Client`
- Resource accessors: `stocks()`, `options()`, `crypto()`, `news()`, `corporate_actions()`
- Documentation hosts:
  - Site: `https://wmzhai.github.io/alpaca-data-rs/`
  - docs.rs: `https://docs.rs/alpaca-data`
  - Repository: `https://github.com/wmzhai/alpaca-data-rs`

## Resource Families

| Resource | Client | Mirror Methods | Convenience Methods | Examples | Tests |
| --- | --- | ---: | ---: | ---: | ---: |
| [Stocks](./reference/stocks.md) | `StocksClient` | 18 | 16 | 2 | 6 |
| [Options](./reference/options.md) | `OptionsClient` | 8 | 8 | 1 | 5 |
| [Crypto](./reference/crypto.md) | `CryptoClient` | 8 | 6 | 1 | 6 |
| [News](./reference/news.md) | `NewsClient` | 1 | 2 | 1 | 2 |
| [Corporate Actions](./reference/corporate-actions.md) | `CorporateActionsClient` | 1 | 2 | 1 | 2 |

## API Layers

- Mirror layer methods preserve the official HTTP endpoint shape and field words.
- Convenience helpers only add `*_all` and `*_stream` pagination flows.
- Request and response field words use Alpaca's official API terms.
- Public Rust naming stays idiomatic: modules lowercase, types `PascalCase`, methods and fields `snake_case`.

## Documentation Generation

`./tools/docs/generate-doc-site` regenerates `docs/reference/`, `docs/project-structure.md`, `docs/generated/`, and `website/sidebars.ts`.

The published site lives at `https://wmzhai.github.io/alpaca-data-rs/`.
