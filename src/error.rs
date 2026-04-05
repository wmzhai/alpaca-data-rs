use std::fmt::{self, Display, Formatter};

use crate::transport::meta::ResponseMeta;

const MAX_ERROR_BODY_CHARS: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidConfiguration(String),
    MissingCredentials,
    Transport(String),
    Timeout(String),
    RateLimited {
        endpoint: &'static str,
        retry_after: Option<u64>,
        request_id: Option<String>,
        attempt_count: u32,
        body: Option<String>,
    },
    HttpStatus {
        endpoint: &'static str,
        status: u16,
        request_id: Option<String>,
        attempt_count: u32,
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
            Self::RateLimited {
                endpoint,
                retry_after,
                request_id,
                attempt_count,
                body,
            } => write_transport_error(
                f,
                "rate limited",
                *endpoint,
                Some(("retry_after", retry_after.map(|value| value.to_string()))),
                request_id.as_deref(),
                *attempt_count,
                body.as_deref(),
            ),
            Self::HttpStatus {
                endpoint,
                status,
                request_id,
                attempt_count,
                body,
            } => write_transport_error(
                f,
                "http status error",
                *endpoint,
                Some(("status", Some(status.to_string()))),
                request_id.as_deref(),
                *attempt_count,
                body.as_deref(),
            ),
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
    pub(crate) fn from_rate_limited(meta: ResponseMeta, body: String) -> Self {
        Self::RateLimited {
            endpoint: meta.endpoint_name,
            retry_after: meta.retry_after.map(|value| value.as_secs()),
            request_id: meta.request_id,
            attempt_count: meta.attempt_count,
            body: snippet_body(body),
        }
    }

    pub(crate) fn from_http_status(meta: ResponseMeta, body: String) -> Self {
        Self::HttpStatus {
            endpoint: meta.endpoint_name,
            status: meta.status,
            request_id: meta.request_id,
            attempt_count: meta.attempt_count,
            body: snippet_body(body),
        }
    }

    pub(crate) fn from_reqwest(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            Self::Timeout(error.to_string())
        } else {
            Self::Transport(error.to_string())
        }
    }

    pub fn endpoint(&self) -> Option<&str> {
        match self {
            Self::RateLimited { endpoint, .. } | Self::HttpStatus { endpoint, .. } => {
                Some(endpoint)
            }
            _ => None,
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::RateLimited { request_id, .. } | Self::HttpStatus { request_id, .. } => {
                request_id.as_deref()
            }
            _ => None,
        }
    }
}

fn write_transport_error(
    f: &mut Formatter<'_>,
    label: &str,
    endpoint: &'static str,
    primary_field: Option<(&str, Option<String>)>,
    request_id: Option<&str>,
    attempt_count: u32,
    body: Option<&str>,
) -> fmt::Result {
    write!(f, "{label}: endpoint={endpoint}")?;

    if let Some((field_name, Some(field_value))) = primary_field {
        write!(f, ", {field_name}={field_value}")?;
    }

    if let Some(request_id) = request_id {
        write!(f, ", request_id={request_id}")?;
    }

    write!(f, ", attempt_count={attempt_count}")?;

    if let Some(body) = body {
        write!(f, ", body={body}")?;
    }

    Ok(())
}

fn snippet_body(body: String) -> Option<String> {
    if body.is_empty() {
        return None;
    }

    let mut snippet: String = body.chars().take(MAX_ERROR_BODY_CHARS).collect();
    if snippet.len() < body.len() {
        snippet.push_str("...");
    }

    Some(snippet)
}
