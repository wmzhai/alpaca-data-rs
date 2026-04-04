use std::fmt::{self, Display, Formatter};

pub use crate::common::enums::Sort;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeFrame(String);

impl TimeFrame {
    pub fn min_1() -> Self {
        Self::from("1Min")
    }

    pub fn day_1() -> Self {
        Self::from("1Day")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for TimeFrame {
    fn default() -> Self {
        Self::min_1()
    }
}

impl From<&str> for TimeFrame {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for TimeFrame {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for TimeFrame {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ContractType {
    #[default]
    Call,
    Put,
}

impl ContractType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Call => "call",
            Self::Put => "put",
        }
    }
}

impl Display for ContractType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OptionsFeed {
    #[default]
    Opra,
    Indicative,
}

impl OptionsFeed {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Opra => "opra",
            Self::Indicative => "indicative",
        }
    }
}

impl Display for OptionsFeed {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TickType {
    #[default]
    Trade,
    Quote,
}

impl TickType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Trade => "trade",
            Self::Quote => "quote",
        }
    }
}

impl Display for TickType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
