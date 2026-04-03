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
pub struct Orderbook {
    pub t: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Snapshot {
    pub latest_trade: Option<Trade>,
    pub latest_quote: Option<Quote>,
    pub latest_bar: Option<Bar>,
}
