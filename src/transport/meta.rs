use std::time::Duration;

const MAX_ERROR_BODY_CHARS: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResponseMeta {
    pub(crate) endpoint_name: &'static str,
    pub(crate) url: String,
    pub(crate) status: u16,
    pub(crate) request_id: Option<String>,
    pub(crate) attempt_count: u32,
    pub(crate) elapsed: Duration,
    pub(crate) retry_after: Option<Duration>,
}

impl ResponseMeta {
    pub(crate) fn from_response(
        endpoint_name: &'static str,
        url: String,
        status: reqwest::StatusCode,
        headers: &reqwest::header::HeaderMap,
        attempt_count: u32,
        elapsed: Duration,
    ) -> Self {
        Self {
            endpoint_name,
            url,
            status: status.as_u16(),
            request_id: parse_request_id(headers),
            attempt_count,
            elapsed,
            retry_after: parse_retry_after(headers),
        }
    }

    pub(crate) fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::from_u16(self.status)
            .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TransportErrorMeta {
    pub(crate) endpoint: &'static str,
    pub(crate) status: u16,
    pub(crate) retry_after: Option<u64>,
    pub(crate) request_id: Option<String>,
    pub(crate) attempt_count: u32,
    pub(crate) body: Option<String>,
}

impl TransportErrorMeta {
    pub(crate) fn from_response_meta(meta: ResponseMeta, body: String) -> Self {
        Self {
            endpoint: meta.endpoint_name,
            status: meta.status,
            retry_after: meta.retry_after.map(|value| value.as_secs()),
            request_id: meta.request_id,
            attempt_count: meta.attempt_count,
            body: snippet_body(body),
        }
    }
}

fn parse_request_id(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get("apca-request-id")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_secs)
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

#[cfg(test)]
mod tests {
    use super::{ResponseMeta, TransportErrorMeta};
    use std::time::Duration;

    #[test]
    fn transport_error_meta_keeps_request_fields_and_snips_body() {
        let response_meta = ResponseMeta {
            endpoint_name: "crypto.latest_quotes",
            url: "https://example.test/v1beta3/crypto/us/latest/quotes".to_owned(),
            status: 429,
            request_id: Some("req-429".to_owned()),
            attempt_count: 2,
            elapsed: Duration::from_millis(150),
            retry_after: Some(Duration::from_secs(3)),
        };
        let long_body = "x".repeat(300);

        let error_meta = TransportErrorMeta::from_response_meta(response_meta, long_body);

        assert_eq!(error_meta.endpoint, "crypto.latest_quotes");
        assert_eq!(error_meta.status, 429);
        assert_eq!(error_meta.retry_after, Some(3));
        assert_eq!(error_meta.request_id.as_deref(), Some("req-429"));
        assert_eq!(error_meta.attempt_count, 2);

        let body = error_meta.body.expect("body snippet should be present");
        assert_eq!(body.chars().count(), 259);
        assert!(body.ends_with("..."));
        assert_eq!(body.trim_end_matches("..."), "x".repeat(256));
    }
}
