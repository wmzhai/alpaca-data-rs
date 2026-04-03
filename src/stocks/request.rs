use super::{Adjustment, Currency, DataFeed, Sort, TimeFrame};

#[derive(Clone, Debug, Default)]
pub struct BarsRequest {
    pub symbols: Vec<String>,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub adjustment: Option<Adjustment>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BarsSingleRequest {
    pub symbol: String,
    pub timeframe: TimeFrame,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub adjustment: Option<Adjustment>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct QuotesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct TradesRequest {
    pub symbols: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub limit: Option<u32>,
    pub feed: Option<DataFeed>,
    pub sort: Option<Sort>,
    pub asof: Option<String>,
    pub currency: Option<Currency>,
    pub page_token: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestBarsRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuotesRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestQuoteRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct LatestTradesRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotsRequest {
    pub symbols: Vec<String>,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct SnapshotRequest {
    pub symbol: String,
    pub feed: Option<DataFeed>,
    pub currency: Option<Currency>,
}

#[derive(Clone, Debug, Default)]
pub struct ConditionCodesRequest {
    pub ticktype: TickType,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TickType {
    #[default]
    Trade,
    Quote,
}
