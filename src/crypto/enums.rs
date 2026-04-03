pub use crate::common::enums::Sort;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimeFrame {
    #[default]
    Min1,
    Day1,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CryptoFeed {
    #[default]
    Us,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Loc {
    #[default]
    Us,
    Us1,
}
