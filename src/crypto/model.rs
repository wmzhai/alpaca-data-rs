use crate::common::time::Timestamp;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bar {
    pub t: Option<Timestamp>,
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trade {
    pub t: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Orderbook {
    pub t: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub latest_trade: Option<Trade>,
    pub latest_quote: Option<Quote>,
    pub latest_bar: Option<Bar>,
}
