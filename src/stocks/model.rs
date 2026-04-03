use crate::common::time::Timestamp;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Bar {
    pub t: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Quote {
    pub t: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trade {
    pub t: Option<Timestamp>,
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
