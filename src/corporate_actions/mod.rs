//! Corporate actions endpoints.
//!
//! Mirror method:
//!
//! - `list`
//!
//! Convenience methods:
//!
//! - `list_all`
//! - `list_stream`
//!
//! The response preserves the official bucketed `corporate_actions` wrapper.

mod client;
mod model;
mod request;
mod response;

pub use crate::common::enums::Sort;
pub use client::CorporateActionsClient;
pub use model::{
    CashDividend, CashMerger, CorporateActions, ForwardSplit, NameChange, Redemption, ReverseSplit,
    RightsDistribution, SpinOff, StockAndCashMerger, StockDividend, StockMerger, UnitSplit,
    UnknownCorporateAction, WorthlessRemoval,
};
pub use request::{CorporateActionType, ListRequest};
pub use response::ListResponse;
