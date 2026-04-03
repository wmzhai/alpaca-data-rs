pub use crate::common::enums::{Currency, Sort};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimeFrame {
    #[default]
    Min1,
    Day1,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Adjustment {
    #[default]
    Raw,
    Split,
    Dividend,
    All,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DataFeed {
    #[default]
    Sip,
    Iex,
    Boats,
    Overnight,
}
