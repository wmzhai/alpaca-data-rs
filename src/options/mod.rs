//! Options market data endpoints.
//!
//! Mirror methods cover historical bars and trades, latest quotes and trades,
//! snapshots, chain lookups, and exchange metadata.
//!
//! Convenience methods add:
//!
//! - `bars_all`
//! - `bars_stream`
//! - `trades_all`
//! - `trades_stream`
//! - `snapshots_all`
//! - `snapshots_stream`
//! - `chain_all`
//! - `chain_stream`

mod client;
mod enums;
mod model;
mod request;
mod response;

pub use client::OptionsClient;
pub use enums::{ContractType, OptionsFeed, Sort, TimeFrame};
pub use model::{Bar, Greeks, Quote, Snapshot, Trade};
pub use request::{
    BarsRequest, ChainRequest, LatestQuotesRequest, LatestTradesRequest, SnapshotsRequest,
    TradesRequest,
};
pub use response::{
    BarsResponse, ChainResponse, ExchangeCodesResponse, LatestQuotesResponse, LatestTradesResponse,
    SnapshotsResponse, TradesResponse,
};
