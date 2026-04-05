use std::time::Duration;

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
