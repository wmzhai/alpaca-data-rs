use std::time::Duration;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct RetryPolicy {
    pub(crate) max_retries: u32,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self { max_retries: 3 }
    }
}

#[allow(dead_code)]
impl RetryPolicy {
    pub(crate) fn new(max_retries: u32) -> Self {
        Self { max_retries }
    }

    pub(crate) fn should_retry(&self, status: reqwest::StatusCode, attempt: u32) -> bool {
        status.is_server_error() && attempt < self.max_retries
    }

    pub(crate) fn backoff(&self, attempt: u32) -> Duration {
        Duration::from_millis(50 * u64::from(attempt.max(1)))
    }
}
