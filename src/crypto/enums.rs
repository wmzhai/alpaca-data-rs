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
pub enum Loc {
    #[default]
    Us,
    Us1,
    Us2,
    Eu1,
    Bs1,
}

impl Loc {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Us => "us",
            Self::Us1 => "us-1",
            Self::Us2 => "us-2",
            Self::Eu1 => "eu-1",
            Self::Bs1 => "bs-1",
        }
    }
}

impl Display for Loc {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::Loc;

    #[test]
    fn loc_serializes_all_official_values() {
        assert_eq!(Loc::Us.as_str(), "us");
        assert_eq!(Loc::Us1.as_str(), "us-1");
        assert_eq!(Loc::Us2.as_str(), "us-2");
        assert_eq!(Loc::Eu1.as_str(), "eu-1");
        assert_eq!(Loc::Bs1.as_str(), "bs-1");
    }
}
