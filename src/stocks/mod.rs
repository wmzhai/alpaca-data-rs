//! Stock market data endpoints.
//!
//! Mirror methods cover historical batch and single-symbol endpoints, latest
//! endpoints, snapshots, and metadata endpoints.
//!
//! Convenience methods add:
//!
//! - `*_all`
//! - `*_stream`
//! - `*_single_all`
//! - `*_single_stream`

mod client;
mod enums;
mod model;
mod request;
mod response;

pub use client::StocksClient;
pub use enums::{Adjustment, Currency, DataFeed, Sort, Tape, TickType, TimeFrame};
pub use model::{Bar, Quote, Snapshot, Trade};
pub use request::{
    BarsRequest, BarsSingleRequest, ConditionCodesRequest, LatestBarRequest, LatestBarsRequest,
    LatestQuoteRequest, LatestQuotesRequest, LatestTradeRequest, LatestTradesRequest,
    QuotesRequest, QuotesSingleRequest, SnapshotRequest, SnapshotsRequest, TradesRequest,
    TradesSingleRequest,
};
pub use response::{
    BarsResponse, BarsSingleResponse, ConditionCodesResponse, ExchangeCodesResponse,
    LatestBarResponse, LatestBarsResponse, LatestQuoteResponse, LatestQuotesResponse,
    LatestTradeResponse, LatestTradesResponse, QuotesResponse, QuotesSingleResponse,
    SnapshotResponse, SnapshotsResponse, TradesResponse, TradesSingleResponse,
};
