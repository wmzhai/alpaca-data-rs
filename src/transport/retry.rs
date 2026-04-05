use std::time::Duration;

#[derive(Clone, Debug)]
pub(crate) struct RetryConfig {
    pub(crate) max_retries: u32,
    pub(crate) retry_on_429: bool,
    pub(crate) respect_retry_after: bool,
    pub(crate) base_backoff: Duration,
    pub(crate) max_backoff: Duration,
    pub(crate) jitter: Option<Duration>,
    pub(crate) total_retry_budget: Option<Duration>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RetryDecision {
    DoNotRetry,
    RetryAfter(Duration),
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_on_429: false,
            respect_retry_after: false,
            base_backoff: Duration::from_millis(50),
            max_backoff: Duration::from_millis(250),
            jitter: None,
            total_retry_budget: None,
        }
    }
}

impl RetryConfig {
    pub(crate) fn classify_status(
        &self,
        status: reqwest::StatusCode,
        attempt: u32,
        retry_after: Option<Duration>,
        elapsed: Duration,
    ) -> RetryDecision {
        if attempt >= self.max_retries {
            return RetryDecision::DoNotRetry;
        }

        let mut wait = if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            if !self.retry_on_429 {
                return RetryDecision::DoNotRetry;
            }

            if self.respect_retry_after {
                retry_after.unwrap_or_else(|| self.backoff(attempt + 1))
            } else {
                self.backoff(attempt + 1)
            }
        } else if status.is_server_error() {
            self.backoff(attempt + 1)
        } else {
            return RetryDecision::DoNotRetry;
        };

        wait = wait.min(self.max_backoff);

        if let Some(total_retry_budget) = self.total_retry_budget {
            if elapsed >= total_retry_budget {
                return RetryDecision::DoNotRetry;
            }

            let remaining = total_retry_budget - elapsed;
            if remaining.is_zero() {
                return RetryDecision::DoNotRetry;
            }

            wait = wait.min(remaining);
        }

        RetryDecision::RetryAfter(self.apply_jitter(wait))
    }

    fn backoff(&self, attempt: u32) -> Duration {
        let factor = 1u32
            .checked_shl(attempt.saturating_sub(1))
            .unwrap_or(u32::MAX);
        let millis = self.base_backoff.as_millis();
        let scaled = millis.saturating_mul(u128::from(factor));
        let bounded = scaled.min(self.max_backoff.as_millis());
        let bounded_u64 = u64::try_from(bounded).unwrap_or(u64::MAX);
        Duration::from_millis(bounded_u64)
    }

    fn apply_jitter(&self, wait: Duration) -> Duration {
        let Some(max_jitter) = self.jitter else {
            return wait;
        };

        let jitter_nanos = max_jitter.as_nanos();
        if jitter_nanos == 0 {
            return wait;
        }

        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let extra = (seed % jitter_nanos.saturating_add(1)).min(u128::from(u64::MAX));
        let extra = Duration::from_nanos(u64::try_from(extra).unwrap_or(u64::MAX));

        wait.saturating_add(extra).min(self.max_backoff)
    }
}

#[cfg(test)]
mod tests {
    use super::{RetryConfig, RetryDecision};

    #[test]
    fn default_config_keeps_429_retries_disabled() {
        let config = RetryConfig::default();

        assert_eq!(
            config.classify_status(
                reqwest::StatusCode::TOO_MANY_REQUESTS,
                0,
                Some(std::time::Duration::from_secs(1)),
                std::time::Duration::ZERO,
            ),
            RetryDecision::DoNotRetry
        );
    }

    #[test]
    fn retry_after_is_honored_when_enabled() {
        let config = RetryConfig {
            retry_on_429: true,
            respect_retry_after: true,
            max_backoff: std::time::Duration::from_secs(10),
            ..RetryConfig::default()
        };

        assert_eq!(
            config.classify_status(
                reqwest::StatusCode::TOO_MANY_REQUESTS,
                0,
                Some(std::time::Duration::from_secs(3)),
                std::time::Duration::ZERO,
            ),
            RetryDecision::RetryAfter(std::time::Duration::from_secs(3))
        );
    }
}
