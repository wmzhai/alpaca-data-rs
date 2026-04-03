use std::sync::Arc;
use std::time::Duration;

use crate::{
    auth::Auth, corporate_actions::CorporateActionsClient, crypto::CryptoClient, error::Error,
    news::NewsClient, options::OptionsClient, stocks::StocksClient,
};

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
    pub fn new() -> Self {
        Self::builder()
            .build()
            .expect("client builder is infallible during bootstrap")
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    pub fn stocks(&self) -> StocksClient {
        StocksClient::new(self.inner.clone())
    }

    pub fn options(&self) -> OptionsClient {
        OptionsClient::new(self.inner.clone())
    }

    pub fn crypto(&self) -> CryptoClient {
        CryptoClient::new(self.inner.clone())
    }

    pub fn news(&self) -> NewsClient {
        NewsClient::new(self.inner.clone())
    }

    pub fn corporate_actions(&self) -> CorporateActionsClient {
        CorporateActionsClient::new(self.inner.clone())
    }

    pub(crate) fn from_parts(
        auth: Auth,
        base_url: String,
        timeout: Duration,
        max_retries: u32,
        max_in_flight: Option<usize>,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                auth,
                base_url,
                timeout,
                max_retries,
                max_in_flight,
            }),
        }
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
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn secret_key(mut self, secret_key: impl Into<String>) -> Self {
        self.secret_key = Some(secret_key.into());
        self
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn max_in_flight(mut self, max_in_flight: usize) -> Self {
        self.max_in_flight = Some(max_in_flight);
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let auth = Auth::new(self.api_key, self.secret_key)?;
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://data.alpaca.markets".to_string());

        Ok(Client::from_parts(
            auth,
            base_url,
            self.timeout,
            self.max_retries,
            self.max_in_flight,
        ))
    }
}
