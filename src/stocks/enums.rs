use std::fmt::{self, Display, Formatter};

pub use crate::common::enums::{Currency, Sort};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Adjustment(String);

impl Adjustment {
    pub fn raw() -> Self {
        Self::from("raw")
    }

    pub fn split() -> Self {
        Self::from("split")
    }

    pub fn dividend() -> Self {
        Self::from("dividend")
    }

    pub fn spin_off() -> Self {
        Self::from("spin-off")
    }

    pub fn all() -> Self {
        Self::from("all")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Adjustment {
    fn default() -> Self {
        Self::raw()
    }
}

impl From<&str> for Adjustment {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Adjustment {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Adjustment {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DataFeed {
    Iex,
    Otc,
    #[default]
    Sip,
    Boats,
}

impl Display for DataFeed {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Iex => "iex",
            Self::Otc => "otc",
            Self::Sip => "sip",
            Self::Boats => "boats",
        })
    }
}
