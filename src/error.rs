use std::fmt::{self, Display, Formatter};

use crate::transport::meta::TransportErrorMeta;

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
    pub(crate) fn from_rate_limited(meta: TransportErrorMeta) -> Self {
        Self::RateLimited {
            endpoint: meta.endpoint,
            retry_after: meta.retry_after,
            request_id: meta.request_id,
            attempt_count: meta.attempt_count,
            body: meta.body,
        }
    }

    pub(crate) fn from_http_status(meta: TransportErrorMeta) -> Self {
        Self::HttpStatus {
            endpoint: meta.endpoint,
            status: meta.status,
            request_id: meta.request_id,
            attempt_count: meta.attempt_count,
            body: meta.body,
        }
    }

    pub(crate) fn from_reqwest(error: reqwest::Error) -> Self {
        let message = sanitize_reqwest_error_message(&error.to_string());

        if error.is_timeout() {
            Self::Timeout(message)
        } else {
            Self::Transport(message)
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

fn sanitize_reqwest_error_message(message: &str) -> String {
    if !message.contains('@') {
        return message.to_string();
    }

    let mut sanitized = String::with_capacity(message.len());
    let mut segment_start = 0;
    let mut in_segment = false;

    for (index, ch) in message.char_indices() {
        if is_message_delimiter(ch) {
            if in_segment {
                sanitized.push_str(&redact_urlish_userinfo(&message[segment_start..index]));
                in_segment = false;
            }

            sanitized.push(ch);
        } else if !in_segment {
            segment_start = index;
            in_segment = true;
        }
    }

    if in_segment {
        sanitized.push_str(&redact_urlish_userinfo(&message[segment_start..]));
    }

    sanitized
}

fn is_message_delimiter(ch: char) -> bool {
    ch.is_whitespace()
        || matches!(
            ch,
            '"' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | ',' | ';'
        )
}

fn redact_urlish_userinfo(segment: &str) -> String {
    if let Ok(mut url) = reqwest::Url::parse(segment) {
        if !url.username().is_empty() || url.password().is_some() {
            let _ = url.set_username("");
            let _ = url.set_password(None);
            return url.to_string();
        }
    }

    redact_urlish_userinfo_fallback(segment)
}

fn redact_urlish_userinfo_fallback(segment: &str) -> String {
    let (prefix, rest) = if let Some((scheme, rest)) = segment.split_once("://") {
        (&segment[..scheme.len() + 3], rest)
    } else if let Some(rest) = segment.strip_prefix("//") {
        ("//", rest)
    } else {
        ("", segment)
    };

    let (authority, suffix) = split_authority_and_suffix(rest);

    match authority.rfind('@') {
        Some(index) => format!("{prefix}{}{}", &authority[index + 1..], suffix),
        None => segment.to_string(),
    }
}

fn split_authority_and_suffix(rest: &str) -> (&str, &str) {
    match rest
        .char_indices()
        .find_map(|(index, ch)| matches!(ch, '/' | '?' | '#').then_some(index))
    {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, ""),
    }
}
