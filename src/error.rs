use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidConfiguration(String),
    MissingCredentials,
    Transport(String),
    Timeout(String),
    RateLimited {
        retry_after: Option<u64>,
        body: Option<String>,
    },
    HttpStatus {
        status: u16,
        body: Option<String>,
    },
    Deserialize(String),
    InvalidRequest(String),
    Pagination(String),
    NotImplemented {
        operation: &'static str,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid configuration: {message}")
            }
            Self::MissingCredentials => write!(f, "missing credentials"),
            Self::Transport(message) => write!(f, "transport error: {message}"),
            Self::Timeout(message) => write!(f, "timeout error: {message}"),
            Self::RateLimited { retry_after, body } => match (retry_after, body) {
                (Some(value), Some(body)) => {
                    write!(f, "rate limited, retry_after={value}, body={body}")
                }
                (Some(value), None) => write!(f, "rate limited, retry_after={value}"),
                (None, Some(body)) => write!(f, "rate limited, body={body}"),
                (None, None) => write!(f, "rate limited"),
            },
            Self::HttpStatus { status, body } => match body {
                Some(body) => write!(f, "http status error: {status}, body={body}"),
                None => write!(f, "http status error: {status}"),
            },
            Self::Deserialize(message) => write!(f, "deserialize error: {message}"),
            Self::InvalidRequest(message) => write!(f, "invalid request: {message}"),
            Self::Pagination(message) => write!(f, "pagination error: {message}"),
            Self::NotImplemented { operation } => {
                write!(f, "operation not implemented: {operation}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn from_reqwest(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::Timeout(error.to_string())
        } else {
            Self::Transport(error.to_string())
        }
    }
}
