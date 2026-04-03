use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Sort {
    #[default]
    Asc,
    Desc,
}

impl Display for Sort {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(transparent)]
pub struct Currency(String);

impl Currency {
    pub fn usd() -> Self {
        Self::from("USD")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Currency {
    fn default() -> Self {
        Self::usd()
    }
}

impl From<&str> for Currency {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for Currency {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Currency {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
