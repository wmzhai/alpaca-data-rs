//! Async Rust client for the Alpaca Market Data HTTP API.
//!
//! The root entrypoint is [`Client`]. It exposes five resource clients:
//!
//! - [`Client::stocks`]
//! - [`Client::options`]
//! - [`Client::crypto`]
//! - [`Client::news`]
//! - [`Client::corporate_actions`]
//!
//! The crate keeps two layers:
//!
//! - Mirror layer: direct wrappers for the official HTTP endpoints
//! - Convenience layer: `*_all` and `*_stream` helpers for paginated endpoints
//!
//! # Example
//!
//! ```no_run
//! use alpaca_data::{Client, stocks};
//!
//! # async fn demo() -> Result<(), alpaca_data::Error> {
//! let client = Client::builder()
//!     .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
//!     .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"))
//!     .build()?;
//!
//! let _response = client
//!     .stocks()
//!     .latest_bars(stocks::LatestBarsRequest {
//!         symbols: vec!["AAPL".into()],
//!         feed: None,
//!         currency: None,
//!     })
//!     .await?;
//! # Ok(())
//! # }
//! ```

mod auth;
mod client;
mod common;
mod env;
mod error;
mod transport;

pub mod corporate_actions;
pub mod crypto;
pub mod news;
pub mod options;
pub mod stocks;

pub use client::{Client, ClientBuilder};
pub use error::Error;
pub use rust_decimal::Decimal;
pub use transport::observer::{ObservedResponseMeta, TransportObserver};
