use crate::common::time::Timestamp;

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Bar {
    pub t: Option<Timestamp>,
    pub o: Option<f64>,
    pub h: Option<f64>,
    pub l: Option<f64>,
    pub c: Option<f64>,
    pub v: Option<u64>,
    pub n: Option<u64>,
    pub vw: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub bx: Option<String>,
    pub bp: Option<f64>,
    pub bs: Option<u64>,
    pub ax: Option<String>,
    pub ap: Option<f64>,
    #[serde(rename = "as")]
    pub r#as: Option<u64>,
    pub c: Option<Vec<String>>,
    pub z: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Trade {
    pub t: Option<Timestamp>,
    pub x: Option<String>,
    pub p: Option<f64>,
    pub s: Option<u64>,
    pub i: Option<u64>,
    pub c: Option<Vec<String>>,
    pub z: Option<String>,
    pub u: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub latest_trade: Option<Trade>,
    pub latest_quote: Option<Quote>,
    pub minute_bar: Option<Bar>,
    pub daily_bar: Option<Bar>,
    pub prev_daily_bar: Option<Bar>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ConditionCode {
    pub code: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExchangeCode {
    pub code: String,
    pub name: String,
}
