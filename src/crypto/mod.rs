//! Crypto market data endpoints.
//!
//! Mirror methods cover historical bars, quotes, trades, latest data, latest
//! orderbooks, and snapshots.
//!
//! Convenience methods add:
//!
//! - `bars_all`
//! - `bars_stream`
//! - `quotes_all`
//! - `quotes_stream`
//! - `trades_all`
//! - `trades_stream`
//!
//! The currently implemented crypto HTTP endpoints can be used without
//! credentials.

mod client;
mod enums;
mod model;
mod request;
mod response;

pub use client::CryptoClient;
pub use enums::{Loc, Sort, TimeFrame};
pub use model::{Bar, Orderbook, OrderbookLevel, Quote, Snapshot, Trade};
pub use request::{
    BarsRequest, LatestBarsRequest, LatestOrderbooksRequest, LatestQuotesRequest,
    LatestTradesRequest, QuotesRequest, SnapshotsRequest, TradesRequest,
};
pub use response::{
    BarsResponse, LatestBarsResponse, LatestOrderbooksResponse, LatestQuotesResponse,
    LatestTradesResponse, QuotesResponse, SnapshotsResponse, TradesResponse,
};
