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
    transport::{http::HttpClient, rate_limit::RateLimiter, retry::RetryPolicy},
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
    pub(crate) max_retries: u32,
    pub(crate) max_in_flight: Option<usize>,
    pub(crate) http: HttpClient,
}

#[derive(Clone, Debug)]
pub struct ClientBuilder {
    api_key: Option<String>,
    secret_key: Option<String>,
    base_url: Option<String>,
    timeout: Duration,
    max_retries: u32,
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
        max_retries: u32,
        max_in_flight: Option<usize>,
    ) -> Result<Self, Error> {
        let http = HttpClient::new(
            timeout,
            RetryPolicy::new(max_retries),
            RateLimiter::new(max_in_flight),
        )?;

        Ok(Self {
            inner: Arc::new(Inner {
                auth,
                base_url,
                timeout,
                max_retries,
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
            max_retries: 3,
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
        self.max_retries = max_retries;
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
        let auth = Auth::new(self.api_key, self.secret_key)?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://data.alpaca.markets".to_string());

        Client::from_parts(
            auth,
            base_url,
            self.timeout,
            self.max_retries,
            self.max_in_flight,
        )
    }
}
