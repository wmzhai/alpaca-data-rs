# CHANGELOG.md

本文件记录每个版本提交对应的主要变化。

规则：

- 每次新版本提交都必须更新本文件
- 不只记录结构变化，也记录对外接口、文档、测试、工程配置和内部实现上的重要变化
- 版本号使用三段格式：`MAJOR.MINOR.PATCH`

## v0.5.3

### Added

- Added `docs/api-coverage.md` as the public endpoint-to-method coverage contract
- Added `tools/api-coverage/market-data-api.json` as the machine-readable coverage manifest for future API audits

### Changed

- Updated `README.md`, `docs/layers.md`, and `docs/release-checklist.md` to point at the new coverage contract
- Updated `memory/api/README.md` and `memory/core/system-map.md` to record the new coverage artifacts in the internal navigation docs
- Recorded the current adopted-family coverage gaps explicitly instead of hiding them: `StockAuctions`, `StockAuctionSingle`, and `OptionMetaConditions` remain unimplemented
- Recorded the current crypto mirror parity gap explicitly: the official crypto `loc` enum includes `us-2` and `bs-1`, while local `crypto::Loc` currently exposes only `us`, `us-1`, and `eu-1`
- Bumped the crate version to `0.5.3` for the `Phase 6 / Task 3` API coverage contract commit

### Verification

- `jq empty tools/api-coverage/market-data-api.json`
- `cargo test`
- `git diff --check`

## v0.5.2

### Added

- Added crate-level rustdoc in `src/lib.rs`
- Added public docs for `Client`, `ClientBuilder`, and all root resource accessors in `src/client.rs`
- Added module-level rustdoc for `stocks`, `options`, `crypto`, `news`, and `corporate_actions`
- Added runnable examples under `examples/`: `client_builder`, `stocks_latest_bar`, `stocks_bars_all`, `options_chain`, `crypto_latest_quotes`, `news_list`, and `corporate_actions_list`

### Changed

- Updated `docs/examples.md` and `README.md` to point at the runnable `examples/` directory
- Updated `memory/core/system-map.md` to record the new `examples/` tree as part of the real repository structure
- Bumped the crate version to `0.5.2` for the `Phase 6 / Task 2` examples and rustdoc commit

### Verification

- `cargo fmt`
- `cargo check --examples`
- `cargo test --doc`
- `git diff --check`

## v0.5.1

### Added

- Added a first public English documentation surface under `docs/`: `getting-started.md`, `authentication.md`, `layers.md`, `examples.md`, and `release-checklist.md`
- Added a new public English `README.md` that documents the shipped API surface, auth behavior, test strategy, benchmark targets, and the current `Phase 6` / `Phase 7` release split
- Added the initial `Phase 6` design and implementation documents for release hardening

### Changed

- Reconciled stale repository state docs so `Phase 5` is consistently recorded as completed in `v0.5.0`, not as a pending merge candidate
- Updated `AGENTS.md`, `memory/README.md`, `memory/api/README.md`, `memory/core/invariants.md`, `memory/core/system-map.md`, `memory/core/workflows.md`, and the master roadmap to align on the approved `Phase 6` and `Phase 7` boundaries
- Bumped the crate version to `0.5.1` for the `Phase 6 / Task 1` documentation baseline commit

### Verification

- `cargo test`
- `git diff --check`
- `rg -n "awaiting merge approval|merge candidate|收尾版本候选|merge approval|merge 获批" README.md AGENTS.md memory docs`

## v0.5.0

### Changed

- `Phase 5: News + Corporate Actions` 现在已形成收尾版本候选：`news` 与 `corporate_actions` 已作为第四、第五个完整资源模板进入 phase-complete 状态，覆盖官方 mirror endpoint、convenience layer、真实 API happy-path、mock fault coverage 与本地 benchmark baseline
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/specs/2026-04-04-phase-5-news-corporate-actions-design.md`、`docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 与 Phase 5 plan 现在都已同步到 `v0.5.0` 的 phase-close candidate 状态，并把后续主线推进到 `Phase 6: Release Hardening`
- 将 crate 版本提升到 `0.5.0`，作为 `Phase 5` 的 MINOR 收尾版本候选；在得到用户确认前，不执行 merge / push / 删分支

### Verification

- `cargo fmt --check`
- `set -a && source .env >/dev/null 2>&1 && set +a && export APCA_API_KEY_ID=\"$ALPACA_DATA_API_KEY\" APCA_API_SECRET_KEY=\"$ALPACA_DATA_SECRET_KEY\" ALPACA_LIVE_TESTS=1 && cargo test`
- `cargo bench --no-run`

## v0.4.3

### Added

- 新建 `tests/mock_news_corporate_actions_errors.rs`，覆盖 `news` / `corporate_actions` 的损坏 JSON、分页 merge 与重复 `next_page_token` 回归
- 新建 `benches/news_corporate_actions.rs`，为 `news.list` 与 `corporate_actions.list` 建立本地 `criterion` micro-benchmark baseline
- `Cargo.toml` 现在新增 `news_corporate_actions` bench target，可直接用 `cargo bench --bench news_corporate_actions --no-run` 验证 benchmark 编译链路

### Changed

- `news` 与 `corporate_actions` 现在都具备 mock fault coverage：既能验证损坏 JSON -> `Error::Deserialize`，也能验证 pagination helper 在 merge 和重复 `next_page_token` 场景下的真实客户端行为
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/specs/2026-04-04-phase-5-news-corporate-actions-design.md`、`docs/superpowers/plans/2026-04-04-phase-5-news-corporate-actions.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 5 / Task 3` 完成后的真实状态，并把下一步收敛到 phase completion candidate
- 将 crate 版本提升到 `0.4.3`，对齐 `Phase 5 / Task 3` 的版本提交要求

### Verification

- `cargo test --test mock_news_corporate_actions_errors -- --nocapture`
- `cargo bench --bench news_corporate_actions --no-run`
- `cargo test`

## v0.4.2

### Added

- 新建 `tests/live_corporate_actions.rs`，使用真实 Alpaca API 覆盖 `corporate_actions.list`、`list_all` 与 `list_stream`
- 为 `corporate_actions` 新增 request / response / route / public API 单元测试，覆盖官方 query 单词、`/v1/corporate-actions` route、bucketed wrapper shape 与 typed family merge
- 为共享 pagination helper 新增重复 `next_page_token` 回归测试，覆盖 `collect_all` / `stream_pages` 的死循环防护

### Changed

- `CorporateActionsClient` 现在已接通 `GET /v1/corporate-actions`，并同时打通 `list_all` / `list_stream`
- `corporate_actions::CorporateActionType` 现在覆盖真实 API 接受的全部 15 个 query 值；`CorporateActions` 现在保留官方 bucketed `corporate_actions` wrapper，并将 13 个 documented family 反序列化为 typed arrays，同时为 `contract_adjustments` / `partial_calls` 与未来未知 bucket 提供保守 fallback
- 共享 `collect_all` / `stream_pages` 现在会在服务端重复返回同一个 `next_page_token` 时立即返回 `Error::Pagination`，避免 `corporate_actions` 等分页资源陷入无限循环
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/invariants.md`、`memory/core/system-map.md`、`memory/core/workflows.md`、`docs/superpowers/specs/2026-04-04-phase-5-news-corporate-actions-design.md`、`docs/superpowers/plans/2026-04-04-phase-5-news-corporate-actions.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 5 / Task 2` 完成后的真实状态，并明确 phase 开始确认、phase 内连续执行、phase 收尾前再次停顿确认，以及 `git merge --ff-only` 的收尾规则
- 将 crate 版本提升到 `0.4.2`，对齐 `Phase 5 / Task 2` 的版本提交要求

### Verification

- `cargo test news -- --nocapture`
- `cargo test corporate_actions -- --nocapture`
- `cargo test --test public_api -- --nocapture`
- `cargo test`
- `set -a && source .env && set +a && export APCA_API_KEY_ID=\"$ALPACA_DATA_API_KEY\" APCA_API_SECRET_KEY=\"$ALPACA_DATA_SECRET_KEY\" ALPACA_LIVE_TESTS=1 && cargo test --test live_news -- --nocapture`
- `set -a && source .env && set +a && export APCA_API_KEY_ID=\"$ALPACA_DATA_API_KEY\" APCA_API_SECRET_KEY=\"$ALPACA_DATA_SECRET_KEY\" ALPACA_LIVE_TESTS=1 && cargo test --test live_corporate_actions -- --nocapture`

## v0.4.1

### Added

- 新建 `docs/superpowers/specs/2026-04-04-phase-5-news-corporate-actions-design.md` 与 `docs/superpowers/plans/2026-04-04-phase-5-news-corporate-actions.md`，将 `Phase 5` 的官方 HTTP 设计约束、真实 API 观察与任务拆分落成仓库文档
- 新建 `tests/live_news.rs`，使用真实 Alpaca API 覆盖 `news.list`、`news.list_all` 与 `news.list_stream`
- 为 `news` 新增 request / response / route / public API 单元测试，覆盖官方 query 单词、`/v1beta1/news` route 与 wrapper shape

### Changed

- `NewsClient` 现在已接通 `GET /v1beta1/news`，并同时打通 `list_all` / `list_stream`
- `news::ListRequest` 现在使用 `Option<Vec<String>>` 承载官方 `symbols` CSV query；`ListResponse` 现在忠实反序列化官方 `news` + `next_page_token` wrapper，并支持分页聚合
- `news::NewsItem` 与 `news::NewsImage` 现在补齐官方字段：`author`、`created_at`、`updated_at`、`summary`、`content`、`url`、`images`、`symbols`、`source`
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/specs/2026-04-04-phase-5-news-corporate-actions-design.md`、`docs/superpowers/plans/2026-04-04-phase-5-news-corporate-actions.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 5 / Task 1` 完成后的真实状态
- 将 crate 版本提升到 `0.4.1`，对齐 `Phase 5 / Task 1` 的版本提交要求

### Verification

- `cargo fmt --check`
- `cargo test`
- `set -a && source .env && set +a && export APCA_API_KEY_ID=\"$ALPACA_DATA_API_KEY\" APCA_API_SECRET_KEY=\"$ALPACA_DATA_SECRET_KEY\" && cargo test news -- --nocapture`

## v0.4.0

### Added

- 新建 `benches/crypto.rs`，为 `crypto.snapshots` 建立本地 `criterion` micro-benchmark baseline
- `Cargo.toml` 现在新增 `crypto` bench target，可直接用 `cargo bench --bench crypto --no-run` 验证 crypto benchmark 编译链路

### Changed

- `Phase 4: Crypto` 现在正式标记为完成；`crypto` 已成为继 `stocks`、`options` 之后第三个完整资源模板，覆盖官方 mirror endpoint、historical convenience、真实 API happy-path、异常路径 mock 与本地 benchmark baseline
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`、`docs/superpowers/specs/2026-04-04-phase-4-crypto-design.md`、`docs/superpowers/plans/2026-04-04-phase-4-crypto.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 4` 完成后的真实状态，并把下一步推进到 `Phase 5: News + Corporate Actions`
- 将 crate 版本提升到 `0.4.0`，作为 `Phase 4: Crypto` 的 MINOR 收尾版本

### Verification

- `cargo fmt --check`
- `cargo test`
- `cargo bench --bench crypto --no-run`

## v0.3.3

### Added

- 新建 `tests/live_crypto_snapshots.rs`，使用真实 Alpaca API 覆盖 `crypto.snapshots`
- 新建 `tests/mock_crypto_errors.rs`，覆盖 `crypto.snapshots` 与 `crypto.latest_orderbooks` 的损坏 JSON -> `Error::Deserialize`
- 为 `crypto.snapshots` 新增 route、request、response 与 public API 单元测试，覆盖 official path、`symbols` query 与 snapshot wrapper shape

### Changed

- `CryptoClient` 现在已接通 `GET /v1beta3/crypto/{loc}/snapshots`
- `crypto::Snapshot` 现在改为忠实对齐官方 camelCase 字段：`latestTrade`、`latestQuote`、`minuteBar`、`dailyBar`、`prevDailyBar`
- `SnapshotsResponse` 现在忠实反序列化官方顶层 `snapshots` map，不再保留 placeholder snake_case snapshot shape
- `README.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/specs/2026-04-04-phase-4-crypto-design.md`、`docs/superpowers/plans/2026-04-04-phase-4-crypto.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 4 / Task 3` 完成后的真实状态
- 将 crate 版本提升到 `0.3.3`，对齐 `Phase 4 / Task 3` 的版本提交要求

### Verification

- `cargo test --lib crypto -- --nocapture`
- `cargo test --test public_api -- --nocapture`
- `cargo test --test mock_crypto_errors -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_crypto_snapshots -- --nocapture`

## v0.3.2

### Added

- 新建 `tests/live_crypto_latest.rs`，使用真实 Alpaca API 覆盖 `crypto.latest_bars`、`crypto.latest_quotes`、`crypto.latest_trades` 与 `crypto.latest_orderbooks`
- 为 `crypto` latest family 新增 route、request、response 与 public API 单元测试，覆盖官方 latest path、`symbols` query、latest wrapper shape 与 `OrderbookLevel` 公开类型

### Changed

- `CryptoClient` 现在已接通 `GET /v1beta3/crypto/{loc}/latest/bars`、`/latest/quotes`、`/latest/trades` 与 `/latest/orderbooks`
- `LatestBarsResponse`、`LatestQuotesResponse`、`LatestTradesResponse` 与 `LatestOrderbooksResponse` 现在忠实反序列化官方顶层 `bars` / `quotes` / `trades` / `orderbooks` map
- `crypto::Orderbook` 现在补齐官方 `a` / `b` / `t` 字段；`crypto::OrderbookLevel` 现作为公开 typed model 暴露
- `README.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/specs/2026-04-04-phase-4-crypto-design.md`、`docs/superpowers/plans/2026-04-04-phase-4-crypto.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 4 / Task 2` 完成后的真实状态
- 将 crate 版本提升到 `0.3.2`，对齐 `Phase 4 / Task 2` 的版本提交要求

### Verification

- `cargo test --lib crypto -- --nocapture`
- `cargo test --test public_api -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_crypto_latest -- --nocapture`

## v0.3.1

### Added

- 新建 `docs/superpowers/specs/2026-04-04-phase-4-crypto-design.md` 与 `docs/superpowers/plans/2026-04-04-phase-4-crypto.md`，将 `Phase 4: Crypto` 的官方 HTTP 设计约束、真实 API 观察与任务拆分落成仓库文档
- 新建 `tests/live_crypto_historical.rs`，使用真实 Alpaca API 覆盖 `crypto.bars`、`crypto.quotes`、`crypto.trades` 与 `crypto.bars_all`
- 为 `crypto` historical 新增 route、request、response 与 pagination 单元测试，覆盖官方 query 单词、`loc` path word、response wrapper shape 与跨页 merge

### Changed

- `CryptoClient` 现在已接通 `GET /v1beta3/crypto/{loc}/bars`、`/quotes` 与 `/trades`，并同时打通 `bars_all` / `bars_stream`、`quotes_all` / `quotes_stream`、`trades_all` / `trades_stream`
- `crypto::Loc` 现在按官方 path word 序列化为 `us`、`us-1`、`eu-1`；历史 `loc` 继续只参与 path 路由，不进入 query
- `crypto::TimeFrame` 现在改为与 `stocks` / `options` 一致的官方字符串封装；`crypto::Bar`、`crypto::Quote`、`crypto::Trade` 现已补齐真实 typed 字段，并保留 crypto 小数 size / volume 形状
- `src/transport/endpoint.rs` 现在修正了 `us1` 伪 path，crypto historical 与 existing latest quote route 都已统一到官方连字符 location word
- `README.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/specs/2026-04-04-phase-4-crypto-design.md`、`docs/superpowers/plans/2026-04-04-phase-4-crypto.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 4 / Task 1` 完成后的真实状态
- 将 crate 版本提升到 `0.3.1`，对齐 `Phase 4 / Task 1` 的版本提交要求

### Verification

- `cargo fmt --check`
- `cargo test --lib crypto -- --nocapture`
- `cargo test --test public_api resource_clients_expose_core_method_names -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_crypto_historical -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_crypto_latest_quotes_smoke -- --nocapture`
- `cargo test`

## v0.3.0

### Added

- 新建 `benches/options.rs`，为 `options.chain` 建立本地 `criterion` micro-benchmark baseline
- `Cargo.toml` 现在新增 `options` bench target，可直接用 `cargo bench --bench options --no-run` 验证 options benchmark 编译链路

### Changed

- `Phase 3: Options` 现在正式标记为完成；`options` 已成为继 `stocks` 之后第二个完整资源模板，覆盖官方 mirror endpoint、完整 convenience layer、真实 API happy-path、异常路径 mock 与本地 benchmark baseline
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`、`docs/superpowers/specs/2026-04-03-phase-3-options-design.md`、`docs/superpowers/plans/2026-04-03-phase-3-options.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 3` 完成后的真实状态，并把下一步推进到 `Phase 4: Crypto`
- 将 crate 版本提升到 `0.3.0`，作为 `Phase 3: Options` 的 MINOR 收尾版本

### Verification

- `cargo fmt --check`
- `set -a && source .env && set +a && cargo test`
- `cargo bench --bench options --no-run`

## v0.2.4

### Added

- 新建 `tests/live_options_snapshots_chain.rs`，使用真实 Alpaca API 覆盖 `options.snapshots`、`options.chain`、`snapshots_all` / `snapshots_stream`、`chain_all` / `chain_stream`
- 新建 `tests/mock_options_errors.rs`，覆盖 `options` snapshots/chain 的损坏 JSON -> `Error::Deserialize` 与跨页重复 symbol -> `Error::Pagination`
- 为 `options` snapshot / chain 新增 request/response/route/public API 单元测试，覆盖官方 query 单词、route path、response wrapper shape、`greeks` / `impliedVolatility` 字段和分页 merge

### Changed

- `OptionsClient` 现在已接通 `GET /v1beta1/options/snapshots` 与 `GET /v1beta1/options/snapshots/{underlying_symbol}`，并同时打通 `snapshots_all` / `snapshots_stream` 与 `chain_all` / `chain_stream`
- `src/transport/endpoint.rs` 现在已补齐 `options` snapshot / chain route：`/v1beta1/options/snapshots` 与 `/v1beta1/options/snapshots/{underlying_symbol}`
- `options::Snapshot` 现在补齐官方 `greeks` 与 `impliedVolatility` 字段；`options::Greeks` 现作为公开 typed model 暴露
- `SnapshotsResponse` 与 `ChainResponse` 现在忠实反序列化官方顶层 `snapshots` + `next_page_token` wrapper，并在 `*_all` / `*_stream` 聚合时拒绝跨页重复 symbol
- `tests/live_options_snapshots_chain.rs` 现在使用更窄但仍保留分页的官方 chain filter 组合，避免真实 API happy-path 因全链路抓取过重而超时
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/plans/2026-04-03-phase-3-options.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 3 / Task 3` 完成后的真实状态
- 将 crate 版本提升到 `0.2.4`，对齐 `Phase 3 / Task 3` 的版本提交要求

### Verification

- `cargo fmt --check`
- `cargo test`
- `set -a && source .env && set +a && cargo test --test live_options_historical -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_options_latest_metadata -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_options_snapshots_chain -- --nocapture`

## v0.2.3

### Added

- 新建 `tests/live_options_latest_metadata.rs`，使用真实 Alpaca API 覆盖 `options.latest_quotes`、`options.latest_trades` 与 `options.exchange_codes`
- 为 `options` latest + metadata 新增 request/response/route 单元测试，覆盖 `feed` query、latest wrapper shape 与 metadata 顶层 map shape

### Changed

- `OptionsClient` 现在已接通 `GET /v1beta1/options/quotes/latest`、`GET /v1beta1/options/trades/latest` 与 `GET /v1beta1/options/meta/exchanges`
- `src/transport/endpoint.rs` 现在已补齐 `options` latest 与 metadata route：`/v1beta1/options/quotes/latest`、`/v1beta1/options/trades/latest`、`/v1beta1/options/meta/exchanges`
- `LatestQuotesResponse` 与 `LatestTradesResponse` 现在忠实反序列化官方顶层 `quotes` / `trades` map；`ExchangeCodesResponse` 现在改为顶层 `HashMap<String, String>`，不再发明 `exchange_codes` wrapper
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md`、`docs/superpowers/plans/2026-04-03-phase-3-options.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 3 / Task 2` 完成后的真实状态
- 将 crate 版本提升到 `0.2.3`，对齐 `Phase 3 / Task 2` 的版本提交要求

### Verification

- `cargo fmt --check`
- `cargo test`
- `set -a && source .env && set +a && cargo test --test live_options_latest_metadata -- --nocapture`

## v0.2.2

### Added

- 新建 `docs/superpowers/specs/2026-04-03-phase-3-options-design.md` 与 `docs/superpowers/plans/2026-04-03-phase-3-options.md`，把 `Phase 3: Options` 的设计约束、真实 API 观察和任务拆分落成仓库文档
- 新建 `tests/live_options_historical.rs`，使用真实 Alpaca API 覆盖 `options.bars`、`options.trades`、`options.bars_all` 与 `options.trades_stream`
- 为 `options` 历史批量端点新增 request/response/route 单元测试，覆盖官方 query 单词、wrapper shape 与分页 merge

### Changed

- `OptionsClient` 现在已接通 `GET /v1beta1/options/bars` 与 `GET /v1beta1/options/trades`，并同时打通 `bars_all` / `bars_stream` 与 `trades_all` / `trades_stream`
- `src/transport/endpoint.rs` 现在已补齐 `options` historical route：`/v1beta1/options/bars` 与 `/v1beta1/options/trades`
- `options::TimeFrame` 现在改为与 `stocks` 一致的官方字符串封装；`options::OptionsFeed` 与 `options::ContractType` 现在也具备官方字符串序列化
- `options::Bar` 与 `options::Trade` 现在改为真实 typed model，`BarsResponse` 与 `TradesResponse` 现在可直接反序列化官方 body，并支持 batch pagination merge
- `README.md`、`AGENTS.md`、`memory/README.md`、`memory/core/system-map.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 3 / Task 1` 完成后的真实状态
- 将 crate 版本提升到 `0.2.2`，对齐 `Phase 3 / Task 1` 的版本提交要求

### Verification

- `cargo fmt --check`
- `cargo test`
- `set -a && source .env && set +a && cargo test --test live_options_historical -- --nocapture`

## v0.2.1

### Changed

- `AGENTS.md` 与 `memory/core/invariants.md` 现在明确记录：如需使用子代理，模型固定只允许使用 `gpt-5.4`
- `AGENTS.md` 的“项目当前状态”现在已同步到真实仓库状态，明确 `Phase 2: Stocks` 已完成，`stocks` 已成为首个完整资源模板，下一步默认进入 `Phase 3: Options`
- `README.md` 与 `memory/README.md` 的当前工作版本已同步更新到 `v0.2.1`
- 将 crate 版本提升到 `0.2.1`，用于提交本次文档与规则对齐修正

### Verification

- `cargo fmt --check`
- `cargo test`

## v0.2.0

### Added

- 新建 `benches/stocks.rs`，为 `stocks.latest_quote` 建立本地 `criterion` micro-benchmark baseline
- `Cargo.toml` 现在新增 `stocks` bench target，可直接用 `cargo bench --bench stocks --no-run` 验证 benchmark 编译链路

### Changed

- `Phase 2: Stocks` 现在正式标记为完成；`stocks` 已成为第一个完整资源模板，覆盖官方 mirror endpoint、batch + single convenience layer、真实 API happy-path、异常路径 mock 与本地 benchmark baseline
- `tests/live_stocks_batch_historical.rs` 的 batch baseline 现在使用更稳的真实 API 参数组合，避免在 10 秒 client timeout 下拉取过重响应体
- `README.md`、`memory/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`、`docs/superpowers/plans/2026-04-03-phase-2-stocks.md` 与 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已同步到 `Phase 2` 完成后的真实状态，并把下一步推进到 `Phase 3: Options`
- 将 crate 版本提升到 `0.2.0`，作为 `Phase 2: Stocks` 的 MINOR 收尾版本

### Verification

- `cargo fmt --check`
- `cargo test`
- `set -a && source .env && set +a && cargo test --test live_stocks_batch_historical -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_stocks_single_historical -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_stocks_latest_snapshot -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_stocks_metadata -- --nocapture`
- `cargo bench --bench stocks --no-run`

## v0.1.6

### Added

- 为 `stocks` 历史 batch convenience layer 新增 live happy-path 验证：`tests/live_stocks_batch_historical.rs` 现在额外覆盖 `bars_all` / `bars_stream`、`quotes_all` / `quotes_stream`、`trades_all` / `trades_stream`
- 为 batch historical 分页聚合新增 response 单元测试与 mock 异常路径覆盖，验证跨页 symbol bucket 合并、`next_page_token` 清理，以及跨页 `currency` 不一致时返回 `Error::Pagination`

### Changed

- `StocksClient` 现在已接通 `bars_all` / `bars_stream`、`quotes_all` / `quotes_stream`、`trades_all` / `trades_stream`，并统一复用共享 `collect_all` / `stream_pages`
- `BarsRequest`、`QuotesRequest`、`TradesRequest` 现在实现共享 `PaginatedRequest`，历史 batch response 现在实现 `PaginatedResponse`，支持跨页聚合同 symbol 的数组并保留官方 `bars` / `quotes` / `trades` map 形状
- 新增 live test 参数调优：batch convenience 的真实 API 测试现在使用“仍会分页但页数可控”的 limit 组合，避免把 happy-path 回归放大成不必要的长时间抓取
- `README.md`、`memory/README.md`、`memory/core/system-map.md` 与 `docs/superpowers/plans/2026-04-03-phase-2-stocks.md` 现在已同步到 Task 6 完成后的真实状态，并把下一步收敛到 phase 收尾
- 将 crate 版本提升到 `0.1.6`，对齐 `Phase 2 / Task 6` 的版本提交要求

### Tests

- `cargo test batch_historical_merge_combines_symbol_buckets_and_clears_next_page_token --lib -- --nocapture`
- `cargo test batch_historical_merge_rejects_mismatched_currency --lib -- --nocapture`
- `cargo test bars_all_rejects_mismatched_batch_currency_across_pages --test mock_stocks_errors -- --nocapture`
- `cargo test bars_stream_rejects_mismatched_batch_currency_across_pages --test mock_stocks_errors -- --nocapture`
- `set -a && source .env && set +a && cargo test stocks_batch_historical_all_and_stream_use_real_api --test live_stocks_batch_historical -- --nocapture`

## v0.1.5

### Added

- 新建 `tests/live_stocks_metadata.rs`，在 `ALPACA_LIVE_TESTS=1` 时使用真实 Alpaca API 覆盖 `stocks.condition_codes` 与 `stocks.exchange_codes`
- 为 stocks metadata 新增 request/response 单元测试与异常路径 mock 测试，覆盖 `ticktype` / `tape` 官方取值、metadata map 形状，以及损坏 JSON -> `Error::Deserialize`
- `stocks` 公开模块现在新增 `stocks::Tape`，用于忠实表达 `condition_codes` 的官方 `tape` query 参数

### Changed

- `StocksClient` 现在已接通 `GET /v2/stocks/meta/conditions/{ticktype}?tape=...` 与 `GET /v2/stocks/meta/exchanges` 两个 metadata endpoint，并继续复用共享 async transport
- `stocks::ConditionCodesRequest` 现在保留官方字段 `ticktype` 和 `tape`；`stocks::TickType` 现在序列化为官方 singular path 值 `trade|quote`
- `stocks::ConditionCodesResponse` 与 `stocks::ExchangeCodesResponse` 现在忠实对齐官方返回体，直接保持顶层 map 形状，不再发明 `condition_codes` / `exchange_codes` wrapper 字段
- `stocks` 模块移除了先前不符合官方 payload 的 metadata object 占位模型，避免继续暴露伪造结构
- `README.md`、`memory/README.md`、`memory/api/README.md`、`memory/core/system-map.md` 与 `docs/superpowers/plans/2026-04-03-phase-2-stocks.md` 现在已同步到 Task 5 完成后的真实状态，并把下一步收敛到历史 batch 便利层
- 将 crate 版本提升到 `0.1.5`，对齐 `Phase 2 / Task 5` 的版本提交要求

### Tests

- `cargo test stocks_ticktype_and_tape_serialize_to_official_strings --lib -- --nocapture`
- `cargo test metadata_request_serializes_official_query_words --lib -- --nocapture`
- `cargo test metadata_responses_deserialize_official_map_shapes --lib -- --nocapture`
- `cargo test malformed_metadata_json_maps_to_deserialize_error --test mock_stocks_errors -- --nocapture`
- `cargo test --test public_api stocks_module_exposes_batch_and_single_type_names -- --nocapture`
- `cargo test endpoint_routes_all_stocks_dynamic_paths_and_marks_them_authenticated --lib -- --nocapture`
- `set -a && source .env && set +a && cargo test --test live_stocks_metadata -- --nocapture`

## v0.1.4

### Added

- 为 `stocks` latest + snapshot family 新增完整公开类型：`LatestBarRequest` / `LatestBarResponse`、`LatestTradeRequest` / `LatestTradeResponse`，并将 batch/single latest 与 snapshot request/response 全部补齐为可实际使用的官方镜像层
- 新建 `tests/live_stocks_latest_snapshot.rs`，在 `ALPACA_LIVE_TESTS=1` 时使用真实 Alpaca API 覆盖 `latest_bars` / `latest_bar`、`latest_quotes` / `latest_quote`、`latest_trades` / `latest_trade`、`snapshots` / `snapshot`

### Changed

- `StocksClient` 现在已接通 `GET /v2/stocks/bars|quotes|trades/latest`、`GET /v2/stocks/{symbol}/bars|quotes|trades/latest`、`GET /v2/stocks/snapshots` 与 `GET /v2/stocks/{symbol}/snapshot`，并全部复用共享 async transport
- `stocks::DataFeed` 现在增加 `delayed_sip` 官方 latest feed 取值，避免 latest/snapshot feed 词汇继续欠建模
- latest batch 响应现在保留官方顶层 `bars` / `quotes` / `trades` 与可选 `currency`，single latest 响应现在保留官方顶层 `symbol`、`bar|quote|trade` 与可选 `currency`
- snapshot 响应现在忠实对齐官方 body：batch `SnapshotsResponse` 直接公开顶层 symbol-keyed object，不再发明 `snapshots` wrapper；single `SnapshotResponse` 直接公开 `symbol`、可选 `currency` 与官方 camelCase snapshot 字段 `latestTrade`、`latestQuote`、`minuteBar`、`dailyBar`、`prevDailyBar`
- `tests/public_api.rs` 现在覆盖全部 stocks latest + snapshot 请求/响应类型与 `latest_bar` / `latest_trade` 方法名
- 将 crate 版本提升到 `0.1.4`，对齐 `Phase 2 / Task 4` 的版本提交要求

### Tests

- `cargo test --test public_api -- --nocapture`
- `cargo test stocks::request::tests --lib -- --nocapture`
- `cargo test stocks::response::tests --lib -- --nocapture`
- `set -a && source .env && set +a && ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_latest_snapshot -- --nocapture`

## v0.1.3

### Added

- 为 `stocks` single historical family 新增 `QuotesSingleRequest`、`TradesSingleRequest`、`QuotesSingleResponse`、`TradesSingleResponse` 公开类型，并将 `BarsSingleResponse` 对齐为官方 single wrapper：包含顶层 `symbol`、分页 `next_page_token` 与可选 `currency`
- 新建 `tests/live_stocks_single_historical.rs`，在 `ALPACA_LIVE_TESTS=1` 时用真实 Alpaca API 覆盖 `bars_single` / `quotes_single` / `trades_single` 及其 `*_all`、`*_stream` 便利层
- 新建 `tests/mock_stocks_errors.rs`，覆盖 single historical 损坏 JSON -> `Error::Deserialize`，以及跨页 `symbol` / `currency` 不一致时返回 `Error::Pagination`

### Changed

- `StocksClient` 现在已接通 `GET /v2/stocks/{symbol}/bars|quotes|trades` 三个 single historical endpoint，并新增 `bars_single_all` / `bars_single_stream`、`quotes_single_all` / `quotes_single_stream`、`trades_single_all` / `trades_single_stream`
- `stocks` single historical 请求现在按官方 query 单词序列化：`timeframe`、`start`、`end`、`limit`、`adjustment`、`asof`、`feed`、`currency`、`page_token`、`sort`
- single historical 分页聚合现在复用共享 `collect_all` / `stream_pages`，并在跨页 `symbol` 或 `currency` 不一致时立即返回 `Error::Pagination`，避免静默拼接错误数据
- `*_single_stream` 现在也会在跨页 `symbol` 或 `currency` 不一致时返回 `Error::Pagination`，与 `*_single_all` 保持相同的一致性约束
- `tests/public_api.rs` 现在覆盖全部 single historical request/response 类型与新方法名称
- single historical live quotes/trades 覆盖现在避开开盘首分钟的大流量窗口，并使用更克制的分页形状，降低真实 API 回归成本
- 将 crate 版本提升到 `0.1.3`，对齐 `Phase 2 / Task 3` 的版本提交要求

### Tests

- `cargo test --test public_api -- --nocapture`
- `cargo test stocks::request::tests --lib -- --nocapture`
- `cargo test single_historical --lib -- --nocapture`
- `cargo test --test mock_stocks_errors malformed_single_historical_json_maps_to_deserialize_error -- --nocapture`
- `cargo test --test mock_stocks_errors bars_single_all_rejects_mismatched_symbol_across_pages -- --nocapture`
- `cargo test --test mock_stocks_errors bars_single_all_rejects_mismatched_currency_across_pages -- --nocapture`
- `set -a && source .env && set +a && ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_single_historical -- --nocapture`

## v0.1.2

### Added

- 为 `stocks` 历史 batch endpoint 新增真实 API baseline：`tests/live_stocks_batch_historical.rs` 会在 `ALPACA_LIVE_TESTS=1` 时用真实 Alpaca API 覆盖 `bars`、`quotes` 和 `trades`
- 为 `stocks` 历史请求新增 query 序列化单元测试，覆盖官方 `symbols`、`timeframe`、`start`、`end`、`limit`、`adjustment`、`asof`、`feed`、`currency`、`page_token`、`sort` 单词

### Changed

- `StocksClient` 现在已接通 `bars`、`quotes`、`trades` 三个历史 batch GET `/v2/stocks/*` 端点，并复用共享 async transport/query 基础设施
- `stocks::TimeFrame`、`stocks::Adjustment` 和 `stocks::Currency` 现在改为官方字符串封装，`stocks::DataFeed` 与 `stocks::Sort` 现在按官方值序列化，并保留 `overnight` feed 兼容覆盖，避免继续编码过窄或错误的参数取值
- `stocks` 历史 batch 响应现在使用带 serde 解码的 typed model/response 结构，覆盖官方 `bars` / `quotes` / `trades` 包装字段、`next_page_token` 与可选 `currency`
- 将 crate 版本提升到 `0.1.2`，对齐 `Phase 2 / Task 2` 的版本提交要求

### Tests

- `cargo test stocks::request::tests --lib -- --nocapture`
- `cargo test stocks_data_feed_serializes_to_official_strings --lib -- --nocapture`
- `set -a && source .env && set +a && ALPACA_LIVE_TESTS=1 cargo test --test live_stocks_batch_historical -- --nocapture`

## v0.1.1

### Added

- 为 `stocks` 新增 Task 1 公开骨架类型：`BarsSingleRequest`、`LatestQuoteRequest`、`SnapshotRequest`、`ConditionCodesRequest`
- 为 `stocks` 新增 Task 1 响应骨架类型：`BarsSingleResponse`、`LatestQuoteResponse`、`SnapshotResponse`
- 为 `Endpoint` 新增 stocks batch / single skeleton 路由变体和基础 path 覆盖

### Changed

- `StocksClient` 现在公开 `bars_single`、`latest_quote`、`snapshot` 和接收 request 的 `condition_codes` skeleton 方法，当前仍返回 `Error::NotImplemented`
- `Endpoint::path()` 现在对静态路由返回 borrowed path，仅在 symbol / ticktype 动态路由上构造 owned path，避免 Task 1 新增 stocks 路由在共享 hot path 上引入不必要分配
- 将 crate 版本提升到 `0.1.1`，对齐 `Phase 2 / Task 1` 的版本提交要求

### Tests

- 更新 `tests/public_api.rs`，覆盖 stocks batch / single public type 与 method 名称
- 为 `src/transport/endpoint.rs` 增加 stocks batch / single path 路由和 `requires_auth` 覆盖，并校验静态 path 保持 borrowed

## v0.1.0

### Added

- 新建 `benches/shared_core.rs`，建立 `crypto.latest_quotes` 共享通路的本地 benchmark baseline
- 为仓库规则补充 phase 级收尾约束：phase 完成后必须跑完整验证、同步所有文档、自动做 MINOR 版本升级，并在完成后合并 `main` / push / 删除开发分支

### Changed

- 将 crate 版本提升到 `0.1.0`，标记 `Phase 1: Shared Core` 完成
- benchmark 现在复用同一个 `Client`，更聚焦共享请求路径本身，而不把 builder 初始化成本混入每次采样
- `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 现在已将 `Phase 1` 标记为完成，并把下一步收敛到 `Phase 2: Stocks`
- `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md` 的 Task 6 与最终校验清单现在已全部对齐为完成状态

### Docs

- 更新 `README.md`，补充当前实现状态以及已落地的 live test / benchmark 命令
- 更新 `AGENTS.md`、`memory/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`、`memory/core/invariants.md`
- 同步 Phase 1 完成后的项目状态、工作流和版本规则

### Verification

- `cargo fmt --check`
- `cargo test`
- `ALPACA_LIVE_TESTS=1 cargo test --test live_crypto_latest_quotes_smoke -- --nocapture`
- `cargo bench --bench shared_core --no-run`

## v0.0.6

### Added

- 新建 `tests/live_crypto_latest_quotes_smoke.rs`，用真实 Alpaca API 验证 `crypto.latest_quotes`
- 为 `tests/public_api.rs` 增加 `crypto.latest_quotes` 的公开 API 覆盖

### Changed

- `Phase 1 / Task 5` 的 canary endpoint 现在已通过真实 API smoke test 验证
- live test 现在支持通过 `ALPACA_LIVE_TESTS=1` 开关启用，并支持 `APCA_API_DATA_URL` 覆盖 base URL

### Docs

- 更新 `AGENTS.md` 当前项目状态
- 更新 `memory/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.5

### Added

- 为分页框架新增 `PaginatedRequest`、`PaginatedResponse`、`collect_all` 和 `stream_pages`
- 为分页 helper 新增单元测试，覆盖聚合和按页流

### Changed

- `common::response::ResponseStream` 现在升级为 boxed async stream 类型
- `empty_stream()` 现在返回 boxed 空 stream，以兼容后续共享分页流接口
- Phase 1 计划和 roadmap 已同步到 Task 4 完成后的当前状态

### Docs

- 更新 `AGENTS.md` 当前项目状态
- 更新 `memory/README.md` 与 `memory/core/system-map.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.4

### Added

- 接入 `reqwest`、`serde`、`serde_json`、`tokio` 与 `wiremock` 作为共享 transport 和异常测试基础依赖
- 新建 `tests/mock_transport_errors.rs`，覆盖 429 retry-after 和损坏 JSON 的异常映射
- 为 `Auth` 增加 request header 注入能力
- 为 `HttpClient` 增加 async JSON GET 通路

### Changed

- `Client` 现在持有共享 `HttpClient`，并根据 builder runtime config 初始化 timeout、retry 和 rate limiting
- `RetryPolicy` 和 `RateLimiter` 现在具备最小实际行为，而不再只是占位结构
- `Error::RateLimited` 现在同时保留 `retry_after` 与 `body`
- `crypto.latest_quotes` 现在已经接到共享 transport，可用于 mock 异常测试链路

### Docs

- 更新 `AGENTS.md` 当前项目状态
- 更新 `memory/README.md` 与 `memory/core/system-map.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.3

### Added

- 新建 `src/common/query.rs`，提供 `QueryWriter` 以统一构造 query 参数
- 新建 `src/transport/endpoint.rs`，提供 `Endpoint` 以统一官方 Market Data 路径路由
- 为 query 序列化和 endpoint 路由新增对应的单元测试

### Changed

- `src/common/mod.rs` 现在导出 `query` 模块
- `src/transport/mod.rs` 现在导出 `endpoint` 模块
- Phase 1 计划和 roadmap 已同步到 Task 2 完成后的当前状态

### Docs

- 更新 `memory/core/system-map.md` 记录新的共享基础层文件
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md`
- 更新 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md`

## v0.0.2

### Added

- 建立 `tests/client_builder.rs`，覆盖 public crypto client、partial credentials 校验和显式 runtime config 构建
- 为 `ClientBuilder` 增加 `timeout`、`max_retries`、`max_in_flight` 三个最小共享运行时配置入口
- 为 `Error` 增加 `InvalidConfiguration`，用于配置层错误建模

### Changed

- `ClientBuilder::build()` 现在默认使用 `https://data.alpaca.markets`
- `Auth` 现在强制 `api_key` 和 `secret_key` 成对出现，不允许半配置状态通过构建
- 项目文档与 memory 已更新到“共享基础层 Task 1 已落地”的当前事实
- 提交流程规则已明确：提交前必须全面检查代码、测试和文档是否彼此对齐，发现问题先修正再提交

### Docs

- 更新 `AGENTS.md` 的任务提交、分支使用和提交流程校验规则
- 更新 `memory/README.md`、`memory/core/system-map.md`、`memory/core/workflows.md`、`memory/core/invariants.md`
- 更新 `docs/superpowers/plans/2026-04-03-full-project-roadmap.md` 和 `docs/superpowers/plans/2026-04-03-phase-1-shared-core.md` 以反映当前进度

## v0.0.1

### Added

- 初始化 `alpaca-data` Rust library crate，包名为 `alpaca-data`
- 建立根入口 `alpaca_data::Client`
- 建立 `stocks`、`options`、`crypto`、`news`、`corporate_actions` 五个资源模块骨架
- 建立 `transport` 和 `common` 最小公共层
- 建立 `tests/public_api.rs`，验证公开 API 形状
- 建立 `AGENTS.md` 和 `memory/` 项目记忆体系
- 建立 `docs/superpowers/plans/2026-04-03-crate-bootstrap.md`

### Changed

- 将 crate 版本设为 `0.0.1`
- 明确 crates.io 包名使用 `alpaca-data`，代码导入名使用 `alpaca_data`
- 明确 commit message 采用英文 Conventional Commits 风格

### Docs

- 写入项目最终设计方案到 `README.md`
- 写入版本提交必须同步更新 `CHANGELOG.md` 的仓库规则

### Notes

- 当前代码仍为最小公开骨架，真实 Alpaca HTTP API 逻辑尚未开始实现
