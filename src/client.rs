use std::sync::Arc;
use std::time::Duration;

use crate::{
    auth::Auth,
    corporate_actions::CorporateActionsClient,
    crypto::CryptoClient,
    error::Error,
    news::NewsClient,
    options::OptionsClient,
    stocks::StocksClient,
    transport::{http::HttpClient, rate_limit::RateLimiter, retry::RetryConfig},
};

/// Root async client for Alpaca Market Data HTTP APIs.
///
/// Build a client once, then obtain resource clients with [`Self::stocks`],
/// [`Self::options`], [`Self::crypto`], [`Self::news`], and
/// [`Self::corporate_actions`].
///
/// # Examples
///
/// ```no_run
/// use alpaca_data::Client;
///
/// let client = Client::builder().build()?;
/// # let _ = client;
/// # Ok::<(), alpaca_data::Error>(())
/// ```
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Inner {
    pub(crate) auth: Auth,
    pub(crate) base_url: String,
    pub(crate) timeout: Duration,
    pub(crate) retry_config: RetryConfig,
    pub(crate) max_in_flight: Option<usize>,
    pub(crate) http: HttpClient,
}

#[derive(Clone, Debug)]
pub struct ClientBuilder {
    api_key: Option<String>,
    secret_key: Option<String>,
    base_url: Option<String>,
    timeout: Duration,
    retry_config: RetryConfig,
    max_in_flight: Option<usize>,
}

impl Client {
    /// Builds a client with default runtime settings and no credentials.
    ///
    /// This is useful for the currently implemented public crypto endpoints.
    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("client builder is infallible during bootstrap")
    }

    /// Starts a [`ClientBuilder`] for explicit runtime configuration.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Returns the stocks resource client.
    pub fn stocks(&self) -> StocksClient {
        StocksClient::new(self.inner.clone())
    }

    /// Returns the options resource client.
    pub fn options(&self) -> OptionsClient {
        OptionsClient::new(self.inner.clone())
    }

    /// Returns the crypto resource client.
    pub fn crypto(&self) -> CryptoClient {
        CryptoClient::new(self.inner.clone())
    }

    /// Returns the news resource client.
    pub fn news(&self) -> NewsClient {
        NewsClient::new(self.inner.clone())
    }

    /// Returns the corporate actions resource client.
    pub fn corporate_actions(&self) -> CorporateActionsClient {
        CorporateActionsClient::new(self.inner.clone())
    }

    pub(crate) fn from_parts(
        auth: Auth,
        base_url: String,
        timeout: Duration,
        retry_config: RetryConfig,
        max_in_flight: Option<usize>,
    ) -> Result<Self, Error> {
        let http = HttpClient::new(
            timeout,
            retry_config.clone(),
            RateLimiter::new(max_in_flight),
        )?;

        Ok(Self {
            inner: Arc::new(Inner {
                auth,
                base_url,
                timeout,
                retry_config,
                max_in_flight,
                http,
            }),
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            secret_key: None,
            base_url: None,
            timeout: Duration::from_secs(10),
            retry_config: RetryConfig::default(),
            max_in_flight: None,
        }
    }
}

impl ClientBuilder {
    /// Sets the Alpaca API key.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the Alpaca API secret key.
    pub fn secret_key(mut self, secret_key: impl Into<String>) -> Self {
        self.secret_key = Some(secret_key.into());
        self
    }

    /// Overrides the default data API base URL.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the retry budget for the shared HTTP transport.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.retry_config.max_retries = max_retries;
        self
    }

    /// Enables or disables automatic retries on HTTP 429 responses.
    pub fn retry_on_429(mut self, retry_on_429: bool) -> Self {
        self.retry_config.retry_on_429 = retry_on_429;
        self
    }

    /// Enables or disables honoring the `Retry-After` response header.
    pub fn respect_retry_after(mut self, respect_retry_after: bool) -> Self {
        self.retry_config.respect_retry_after = respect_retry_after;
        self
    }

    /// Sets the base retry backoff used by the shared HTTP transport.
    pub fn base_backoff(mut self, base_backoff: Duration) -> Self {
        self.retry_config.base_backoff = base_backoff;
        self
    }

    /// Sets the maximum retry backoff used by the shared HTTP transport.
    pub fn max_backoff(mut self, max_backoff: Duration) -> Self {
        self.retry_config.max_backoff = max_backoff;
        self
    }

    /// Sets an optional jitter window applied to retry waits.
    pub fn retry_jitter(mut self, retry_jitter: Duration) -> Self {
        self.retry_config.jitter = Some(retry_jitter);
        self
    }

    /// Sets an optional total retry time budget for a request.
    pub fn total_retry_budget(mut self, total_retry_budget: Duration) -> Self {
        self.retry_config.total_retry_budget = Some(total_retry_budget);
        self
    }

    /// Sets the maximum number of concurrent in-flight requests.
    pub fn max_in_flight(mut self, max_in_flight: usize) -> Self {
        self.max_in_flight = Some(max_in_flight);
        self
    }

    /// Validates configuration and builds a [`Client`].
    ///
    /// Credentials must be provided as a pair or omitted as a pair.
    pub fn build(self) -> Result<Client, Error> {
        if self.retry_config.max_backoff < self.retry_config.base_backoff {
            return Err(Error::InvalidConfiguration(
                "max_backoff must be greater than or equal to base_backoff".into(),
            ));
        }

        let auth = Auth::new(self.api_key, self.secret_key)?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://data.alpaca.markets".to_string());

        Client::from_parts(
            auth,
            base_url,
            self.timeout,
            self.retry_config,
            self.max_in_flight,
        )
    }
}
