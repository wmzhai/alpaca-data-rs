use crate::common::time::Timestamp;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Bar {
    pub t: Option<Timestamp>,
    pub o: Option<f64>,
    pub h: Option<f64>,
    pub l: Option<f64>,
    pub c: Option<f64>,
    pub v: Option<f64>,
    pub n: Option<u64>,
    pub vw: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub bp: Option<f64>,
    pub bs: Option<f64>,
    pub ap: Option<f64>,
    #[serde(rename = "as")]
    pub r#as: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Trade {
    pub t: Option<Timestamp>,
    pub p: Option<f64>,
    pub s: Option<f64>,
    pub i: Option<u64>,
    pub tks: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct OrderbookLevel {
    pub p: Option<f64>,
    pub s: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Orderbook {
    pub t: Option<Timestamp>,
    pub b: Option<Vec<OrderbookLevel>>,
    pub a: Option<Vec<OrderbookLevel>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub latest_trade: Option<Trade>,
    pub latest_quote: Option<Quote>,
    pub latest_bar: Option<Bar>,
}
