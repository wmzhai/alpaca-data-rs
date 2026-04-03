use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    MissingCredentials,
    Transport(String),
    Timeout(String),
    RateLimited { retry_after: Option<u64> },
    HttpStatus { status: u16, body: Option<String> },
    Deserialize(String),
    InvalidRequest(String),
    Pagination(String),
    NotImplemented { operation: &'static str },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCredentials => write!(f, "missing credentials"),
            Self::Transport(message) => write!(f, "transport error: {message}"),
            Self::Timeout(message) => write!(f, "timeout error: {message}"),
            Self::RateLimited { retry_after } => match retry_after {
                Some(value) => write!(f, "rate limited, retry_after={value}"),
                None => write!(f, "rate limited"),
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
