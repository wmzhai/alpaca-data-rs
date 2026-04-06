use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::{
    auth::Auth,
    corporate_actions::CorporateActionsClient,
    crypto::CryptoClient,
    env,
    error::Error,
    news::NewsClient,
    options::OptionsClient,
    stocks::StocksClient,
    transport::{
        http::HttpClient,
        observer::{ObserverHandle, TransportObserver},
        rate_limit::RateLimiter,
        retry::RetryConfig,
    },
};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

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
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

#[allow(dead_code)]
pub(crate) struct Inner {
    pub(crate) auth: Auth,
    pub(crate) base_url: String,
    pub(crate) timeout: Option<Duration>,
    pub(crate) retry_config: RetryConfig,
    pub(crate) max_in_flight: Option<usize>,
    pub(crate) http: HttpClient,
}

#[derive(Clone)]
pub struct ClientBuilder {
    api_key: Option<String>,
    secret_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    reqwest_client: Option<reqwest::Client>,
    observer: Option<ObserverHandle>,
    retry_config: RetryConfig,
    max_in_flight: Option<usize>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("inner", &self.inner)
            .finish()
    }
}

impl fmt::Debug for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inner")
            .field("auth", &self.auth)
            .field("base_url", &RedactedBaseUrl(&self.base_url))
            .field("timeout", &self.timeout)
            .field("retry_config", &self.retry_config)
            .field("max_in_flight", &self.max_in_flight)
            .field("http", &ConfiguredDebug("HttpClient"))
            .finish()
    }
}

impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("api_key", &RedactedCredential(&self.api_key))
            .field("secret_key", &RedactedCredential(&self.secret_key))
            .field("base_url", &self.base_url.as_deref().map(RedactedBaseUrl))
            .field("timeout", &self.timeout)
            .field(
                "reqwest_client",
                &self
                    .reqwest_client
                    .as_ref()
                    .map(|_| ConfiguredDebug("reqwest::Client")),
            )
            .field("observer", &self.observer)
            .field("retry_config", &self.retry_config)
            .field("max_in_flight", &self.max_in_flight)
            .finish()
    }
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
        timeout: Option<Duration>,
        reqwest_client: Option<reqwest::Client>,
        observer: Option<ObserverHandle>,
        retry_config: RetryConfig,
        max_in_flight: Option<usize>,
    ) -> Result<Self, Error> {
        let http = match reqwest_client {
            Some(client) => HttpClient::with_client(
                client,
                observer,
                retry_config.clone(),
                RateLimiter::new(max_in_flight),
            ),
            None => HttpClient::from_timeout(
                timeout.unwrap_or(DEFAULT_TIMEOUT),
                observer,
                retry_config.clone(),
                RateLimiter::new(max_in_flight),
            )?,
        };

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
            timeout: None,
            reqwest_client: None,
            observer: None,
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

    /// Sets the request timeout for the internally constructed `reqwest::Client`.
    ///
    /// Building fails if `reqwest_client(...)` is also used because the injected
    /// client owns its own timeout configuration.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Injects a preconfigured `reqwest::Client` for advanced transport tuning.
    ///
    /// The injected client owns reqwest-level behavior such as connection
    /// pooling, proxy behavior, default headers, and timeout settings. Build
    /// validation rejects conflicting builder settings such as `timeout(...)`.
    pub fn reqwest_client(mut self, reqwest_client: reqwest::Client) -> Self {
        self.reqwest_client = Some(reqwest_client);
        self
    }

    /// Registers an immutable observer for successful transport responses.
    ///
    /// Observers receive endpoint metadata only. They cannot change request
    /// execution or response shaping.
    pub fn observer(mut self, observer: Arc<dyn TransportObserver>) -> Self {
        self.observer = Some(ObserverHandle::new(observer));
        self
    }

    /// Sets the maximum number of retry attempts for one request.
    ///
    /// This applies to server-error retries by default. HTTP 429 retries only
    /// participate when [`Self::retry_on_429`] is enabled.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.retry_config.max_retries = max_retries;
        self
    }

    /// Enables or disables automatic retries on HTTP 429 responses.
    ///
    /// This setting is disabled by default and affects only 429 responses. It
    /// does not automatically enable honoring `Retry-After`.
    pub fn retry_on_429(mut self, retry_on_429: bool) -> Self {
        self.retry_config.retry_on_429 = retry_on_429;
        self
    }

    /// Enables or disables honoring the `Retry-After` response header.
    ///
    /// This setting only participates when 429 retries are enabled with
    /// [`Self::retry_on_429`]. If a 429 response omits `Retry-After`, the
    /// transport falls back to the configured backoff schedule.
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

    /// Adds a bounded random delay on top of each computed retry wait.
    ///
    /// Jitter helps concurrent callers avoid retrying in lockstep. The
    /// transport keeps the added delay within the configured retry budget and
    /// maximum backoff constraints.
    pub fn retry_jitter(mut self, retry_jitter: Duration) -> Self {
        self.retry_config.jitter = Some(retry_jitter);
        self
    }

    /// Sets an optional elapsed-time budget for one request's retry loop.
    ///
    /// The transport subtracts the request's retry-loop elapsed time from this
    /// budget before each retry decision. The remaining budget then caps each
    /// scheduled retry wait, including waits derived from `Retry-After` and
    /// waits with jitter enabled.
    pub fn total_retry_budget(mut self, total_retry_budget: Duration) -> Self {
        self.retry_config.total_retry_budget = Some(total_retry_budget);
        self
    }

    /// Loads credentials from `APCA_API_KEY_ID` and `APCA_API_SECRET_KEY`.
    ///
    /// If both variables are unset, the builder is left unchanged. If only one
    /// side is set, this returns [`Error::InvalidConfiguration`].
    pub fn credentials_from_env(self) -> Result<Self, Error> {
        self.credentials_from_env_names(env::DEFAULT_API_KEY_ENV, env::DEFAULT_SECRET_KEY_ENV)
    }

    /// Loads credentials from the provided environment variable names.
    ///
    /// If both variables are unset, the builder is left unchanged. If only one
    /// side is set, this returns [`Error::InvalidConfiguration`].
    pub fn credentials_from_env_names(
        mut self,
        api_key_var: &str,
        secret_key_var: &str,
    ) -> Result<Self, Error> {
        if let Some((api_key, secret_key)) =
            env::credentials_from_env_names(api_key_var, secret_key_var)?
        {
            self.api_key = Some(api_key);
            self.secret_key = Some(secret_key);
        }

        Ok(self)
    }

    /// Sets the maximum number of concurrent in-flight requests.
    pub fn max_in_flight(mut self, max_in_flight: usize) -> Self {
        self.max_in_flight = Some(max_in_flight);
        self
    }

    /// Validates configuration and builds a [`Client`].
    ///
    /// Credentials must be provided as a pair or omitted as a pair. Any
    /// provided `api_key` and `secret_key` values must be nonblank and valid
    /// HTTP header values before the client is constructed.
    pub fn build(self) -> Result<Client, Error> {
        if self.retry_config.max_backoff < self.retry_config.base_backoff {
            return Err(Error::InvalidConfiguration(
                "max_backoff must be greater than or equal to base_backoff".into(),
            ));
        }

        if self.reqwest_client.is_some() && self.timeout.is_some() {
            return Err(Error::InvalidConfiguration(
                "reqwest_client owns timeout configuration; remove timeout(...) or configure timeout on the injected reqwest::Client".into(),
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
            self.reqwest_client,
            self.observer,
            self.retry_config,
            self.max_in_flight,
        )
    }
}

struct RedactedCredential<'a>(&'a Option<String>);

impl fmt::Debug for RedactedCredential<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(_) => f.write_str("\"[REDACTED]\""),
            None => f.write_str("None"),
        }
    }
}

struct RedactedBaseUrl<'a>(&'a str);

impl fmt::Debug for RedactedBaseUrl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&redact_base_url_userinfo(self.0), f)
    }
}

struct ConfiguredDebug(&'static str);

impl fmt::Debug for ConfiguredDebug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{} {{ .. }}\"", self.0)
    }
}

fn redact_base_url_userinfo(base_url: &str) -> String {
    if let Ok(mut url) = reqwest::Url::parse(base_url) {
        if !url.username().is_empty() || url.password().is_some() {
            let _ = url.set_username("");
            let _ = url.set_password(None);
            return url.to_string();
        }
    }

    redact_base_url_userinfo_fallback(base_url)
}

fn redact_base_url_userinfo_fallback(base_url: &str) -> String {
    let (prefix, rest) = if let Some((scheme, rest)) = base_url.split_once("://") {
        (&base_url[..scheme.len() + 3], rest)
    } else if let Some(rest) = base_url.strip_prefix("//") {
        ("//", rest)
    } else {
        ("", base_url)
    };

    let (authority, suffix) = split_authority_and_suffix(rest);

    match authority.rfind('@') {
        Some(index) => format!("{prefix}{}{}", &authority[index + 1..], suffix),
        None => base_url.to_string(),
    }
}

fn split_authority_and_suffix(rest: &str) -> (&str, &str) {
    match rest
        .char_indices()
        .find_map(|(index, ch)| matches!(ch, '/' | '?' | '#').then_some(index))
    {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, ""),
    }
}
