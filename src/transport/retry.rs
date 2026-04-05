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

        let wait = if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
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

        match self.finalize_wait(wait, elapsed, current_jitter_seed()) {
            Some(wait) => RetryDecision::RetryAfter(wait),
            None => RetryDecision::DoNotRetry,
        }
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

    fn finalize_wait(
        &self,
        wait: Duration,
        elapsed: Duration,
        jitter_seed: u128,
    ) -> Option<Duration> {
        let wait = wait.min(self.max_backoff);

        let remaining_budget = match self.total_retry_budget {
            Some(total_retry_budget) => {
                let remaining_budget = total_retry_budget.checked_sub(elapsed)?;
                if remaining_budget.is_zero() {
                    return None;
                }

                Some(remaining_budget)
            }
            None => None,
        };

        let wait = if let Some(remaining_budget) = remaining_budget {
            if remaining_budget.is_zero() {
                return None;
            }

            self.apply_jitter_with_seed(wait.min(remaining_budget), jitter_seed)
                .min(remaining_budget)
        } else {
            self.apply_jitter_with_seed(wait, jitter_seed)
        };

        Some(wait)
    }

    fn apply_jitter_with_seed(&self, wait: Duration, jitter_seed: u128) -> Duration {
        let Some(max_jitter) = self.jitter else {
            return wait;
        };

        let jitter_nanos = max_jitter.as_nanos();
        if jitter_nanos == 0 {
            return wait;
        }

        let extra = (jitter_seed % jitter_nanos.saturating_add(1)).min(u128::from(u64::MAX));
        let extra = Duration::from_nanos(u64::try_from(extra).unwrap_or(u64::MAX));

        wait.saturating_add(extra).min(self.max_backoff)
    }
}

fn current_jitter_seed() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

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
            max_backoff: Duration::from_secs(10),
            ..RetryConfig::default()
        };

        assert_eq!(
            config.classify_status(
                reqwest::StatusCode::TOO_MANY_REQUESTS,
                0,
                Some(Duration::from_secs(3)),
                Duration::ZERO,
            ),
            RetryDecision::RetryAfter(Duration::from_secs(3))
        );
    }

    #[test]
    fn budget_without_jitter_respects_remaining_budget() {
        let config = RetryConfig {
            total_retry_budget: Some(Duration::from_millis(100)),
            ..RetryConfig::default()
        };

        assert_eq!(
            config.finalize_wait(Duration::from_millis(50), Duration::from_millis(90), 0),
            Some(Duration::from_millis(10))
        );
    }

    #[test]
    fn budget_with_jitter_still_does_not_exceed_remaining_budget() {
        let config = RetryConfig {
            jitter: Some(Duration::from_millis(50)),
            total_retry_budget: Some(Duration::from_millis(100)),
            max_backoff: Duration::from_secs(1),
            ..RetryConfig::default()
        };

        assert_eq!(
            config.finalize_wait(
                Duration::from_millis(10),
                Duration::from_millis(90),
                Duration::from_millis(50).as_nanos(),
            ),
            Some(Duration::from_millis(10))
        );
    }

    #[test]
    fn exhausted_budget_returns_do_not_retry() {
        let config = RetryConfig {
            total_retry_budget: Some(Duration::from_millis(100)),
            ..RetryConfig::default()
        };

        assert_eq!(
            config.classify_status(
                reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                0,
                None,
                Duration::from_millis(100),
            ),
            RetryDecision::DoNotRetry
        );
    }
}
