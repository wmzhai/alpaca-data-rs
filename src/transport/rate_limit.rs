#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub(crate) struct RateLimiter {
    pub(crate) max_in_flight: Option<usize>,
}
