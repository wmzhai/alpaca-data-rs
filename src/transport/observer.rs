use std::{fmt, sync::Arc, time::Duration};

use crate::transport::meta::ResponseMeta;

/// Immutable callback hook for successful HTTP responses.
pub trait TransportObserver: Send + Sync {
    /// Receives metadata about a successful response after transport retries finish.
    fn on_response(&self, meta: &ObservedResponseMeta);
}

/// Read-only metadata emitted to [`TransportObserver`] implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedResponseMeta {
    /// Stable endpoint name such as `stocks.latest_bars`.
    pub endpoint_name: &'static str,
    /// Fully resolved request URL.
    pub url: String,
    /// Final HTTP status code.
    pub status: u16,
    /// Alpaca request identifier when the server returned one.
    pub request_id: Option<String>,
    /// Number of retry attempts that happened before this terminal response.
    pub attempt_count: u32,
    /// Total elapsed request time across retries.
    pub elapsed: Duration,
}

impl From<&ResponseMeta> for ObservedResponseMeta {
    fn from(meta: &ResponseMeta) -> Self {
        Self {
            endpoint_name: meta.endpoint_name,
            url: meta.url.clone(),
            status: meta.status,
            request_id: meta.request_id.clone(),
            attempt_count: meta.attempt_count,
            elapsed: meta.elapsed,
        }
    }
}

#[derive(Clone)]
pub(crate) struct ObserverHandle {
    observer: Arc<dyn TransportObserver>,
}

impl ObserverHandle {
    pub(crate) fn new(observer: Arc<dyn TransportObserver>) -> Self {
        Self { observer }
    }

    pub(crate) fn on_response(&self, meta: &ResponseMeta) {
        let observed = ObservedResponseMeta::from(meta);
        self.observer.on_response(&observed);
    }
}

impl fmt::Debug for ObserverHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ObserverHandle(..)")
    }
}
