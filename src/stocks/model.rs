use crate::common::time::Timestamp;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Auction {
    pub t: Option<Timestamp>,
    pub x: Option<String>,
    pub p: Option<rust_decimal::Decimal>,
    pub s: Option<u64>,
    pub c: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct DailyAuction {
    pub d: Option<String>,
    pub o: Vec<Auction>,
    pub c: Vec<Auction>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Bar {
    pub t: Option<Timestamp>,
    pub o: Option<rust_decimal::Decimal>,
    pub h: Option<rust_decimal::Decimal>,
    pub l: Option<rust_decimal::Decimal>,
    pub c: Option<rust_decimal::Decimal>,
    pub v: Option<u64>,
    pub n: Option<u64>,
    pub vw: Option<rust_decimal::Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub bx: Option<String>,
    pub bp: Option<rust_decimal::Decimal>,
    pub bs: Option<u64>,
    pub ax: Option<String>,
    pub ap: Option<rust_decimal::Decimal>,
    #[serde(rename = "as")]
    pub r#as: Option<u64>,
    pub c: Option<Vec<String>>,
    pub z: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Trade {
    pub t: Option<Timestamp>,
    pub x: Option<String>,
    pub p: Option<rust_decimal::Decimal>,
    pub s: Option<u64>,
    pub i: Option<u64>,
    pub c: Option<Vec<String>>,
    pub z: Option<String>,
    pub u: Option<String>,
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
