use crate::common::{decimal::Decimal, time::Timestamp};

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Bar {
    pub t: Option<Timestamp>,
    pub o: Option<Decimal>,
    pub h: Option<Decimal>,
    pub l: Option<Decimal>,
    pub c: Option<Decimal>,
    pub v: Option<Decimal>,
    pub n: Option<u64>,
    pub vw: Option<Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub bp: Option<Decimal>,
    pub bs: Option<Decimal>,
    pub ap: Option<Decimal>,
    #[serde(rename = "as")]
    pub r#as: Option<Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Trade {
    pub t: Option<Timestamp>,
    pub p: Option<Decimal>,
    pub s: Option<Decimal>,
    pub i: Option<u64>,
    pub tks: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct OrderbookLevel {
    pub p: Option<Decimal>,
    pub s: Option<Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Orderbook {
    pub t: Option<Timestamp>,
    pub b: Option<Vec<OrderbookLevel>>,
    pub a: Option<Vec<OrderbookLevel>>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Snapshot {
    pub latestTrade: Option<Trade>,
    pub latestQuote: Option<Quote>,
    pub minuteBar: Option<Bar>,
    pub dailyBar: Option<Bar>,
    pub prevDailyBar: Option<Bar>,
}
