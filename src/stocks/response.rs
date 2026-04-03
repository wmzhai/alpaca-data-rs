use std::collections::HashMap;

use super::{Bar, ConditionCode, ExchangeCode, Quote, Snapshot, Trade};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BarsResponse {
    pub bars: HashMap<String, Vec<Bar>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BarsSingleResponse {
    pub bars: Vec<Bar>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct QuotesResponse {
    pub quotes: HashMap<String, Vec<Quote>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TradesResponse {
    pub trades: HashMap<String, Vec<Trade>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestBarsResponse {
    pub bars: HashMap<String, Bar>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestQuotesResponse {
    pub quotes: HashMap<String, Quote>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestQuoteResponse {
    pub quote: Quote,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestTradesResponse {
    pub trades: HashMap<String, Trade>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotsResponse {
    pub snapshots: HashMap<String, Snapshot>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotResponse {
    pub snapshot: Snapshot,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConditionCodesResponse {
    pub condition_codes: Vec<ConditionCode>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeCodesResponse {
    pub exchange_codes: Vec<ExchangeCode>,
}
