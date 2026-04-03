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
