use super::{Loc, Sort, TimeFrame};

#[derive(Clone, Debug, Default)]
pub struct BarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub loc: Option<Loc>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct QuotesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub loc: Option<Loc>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TradesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub sort: Option<Sort>,
    pub loc: Option<Loc>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestBarsRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuotesRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestTradesRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestOrderbooksRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotsRequest {
    pub symbols: Vec<String>,
    pub loc: Option<Loc>,
}
