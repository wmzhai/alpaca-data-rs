use crate::common::{decimal::Decimal, time::Timestamp};

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Bar {
    pub t: Option<Timestamp>,
    pub o: Option<Decimal>,
    pub h: Option<Decimal>,
    pub l: Option<Decimal>,
    pub c: Option<Decimal>,
    pub v: Option<u64>,
    pub n: Option<u64>,
    pub vw: Option<Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Quote {
    pub t: Option<Timestamp>,
    pub bx: Option<String>,
    pub bp: Option<Decimal>,
    pub bs: Option<u64>,
    pub ax: Option<String>,
    pub ap: Option<Decimal>,
    #[serde(rename = "as")]
    pub r#as: Option<u64>,
    pub c: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Trade {
    pub t: Option<Timestamp>,
    pub x: Option<String>,
    pub p: Option<Decimal>,
    pub s: Option<u64>,
    pub c: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
pub struct Greeks {
    pub delta: Option<Decimal>,
    pub gamma: Option<Decimal>,
    pub rho: Option<Decimal>,
    pub theta: Option<Decimal>,
    pub vega: Option<Decimal>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct Snapshot {
    pub latestTrade: Option<Trade>,
    pub latestQuote: Option<Quote>,
    pub minuteBar: Option<Bar>,
    pub dailyBar: Option<Bar>,
    pub prevDailyBar: Option<Bar>,
    pub greeks: Option<Greeks>,
    pub impliedVolatility: Option<Decimal>,
}
