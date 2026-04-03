pub use crate::common::enums::Sort;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimeFrame {
    #[default]
    Min1,
    Day1,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ContractType {
    #[default]
    Call,
    Put,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OptionsFeed {
    #[default]
    Opra,
    Indicative,
}
