use std::collections::HashMap;

use super::{Bar, ExchangeCode, Quote, Snapshot, Trade};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BarsResponse {
    pub bars: HashMap<String, Vec<Bar>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TradesResponse {
    pub trades: HashMap<String, Vec<Trade>>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestQuotesResponse {
    pub quotes: HashMap<String, Quote>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LatestTradesResponse {
    pub trades: HashMap<String, Trade>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SnapshotsResponse {
    pub snapshots: HashMap<String, Snapshot>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChainResponse {
    pub snapshots: HashMap<String, Snapshot>,
    pub next_page_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExchangeCodesResponse {
    pub exchange_codes: Vec<ExchangeCode>,
}
