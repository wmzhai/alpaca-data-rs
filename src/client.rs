use std::sync::Arc;

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
    pub(crate) base_url: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ClientBuilder {
    api_key: Option<String>,
    secret_key: Option<String>,
    base_url: Option<String>,
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

    pub fn build(self) -> Result<Client, Error> {
        Ok(Client {
            inner: Arc::new(Inner {
                auth: Auth {
                    api_key: self.api_key,
                    secret_key: self.secret_key,
                },
                base_url: self.base_url,
            }),
        })
    }
}
