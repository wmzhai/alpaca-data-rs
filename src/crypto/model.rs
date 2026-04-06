use crate::common::time::Timestamp;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Bar {
    pub t: Option<Timestamp>,
    pub o: Option<rust_decimal::Decimal>,
    pub h: Option<rust_decimal::Decimal>,
    pub l: Option<rust_decimal::Decimal>,
    pub c: Option<rust_decimal::Decimal>,
    pub v: Option<rust_decimal::Decimal>,
    pub n: Option<u64>,
    pub vw: Option<rust_decimal::Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub bp: Option<rust_decimal::Decimal>,
    pub bs: Option<rust_decimal::Decimal>,
    pub ap: Option<rust_decimal::Decimal>,
    #[serde(rename = "as")]
    pub r#as: Option<rust_decimal::Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Trade {
    pub t: Option<Timestamp>,
    pub p: Option<rust_decimal::Decimal>,
    pub s: Option<rust_decimal::Decimal>,
    pub i: Option<u64>,
    pub tks: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct OrderbookLevel {
    pub p: Option<rust_decimal::Decimal>,
    pub s: Option<rust_decimal::Decimal>,
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
