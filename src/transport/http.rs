use std::time::{Duration, Instant};

use crate::{
    Error,
    auth::Auth,
    transport::{
        endpoint::Endpoint,
        meta::ResponseMeta,
        observer::ObserverHandle,
        rate_limit::RateLimiter,
        retry::{RetryConfig, RetryDecision},
    },
};

#[derive(Clone, Debug)]
pub(crate) struct HttpClient {
    client: reqwest::Client,
    observer: Option<ObserverHandle>,
    retry_config: RetryConfig,
    rate_limiter: RateLimiter,
}

#[derive(Debug)]
struct ResponseParts {
    meta: ResponseMeta,
    body: String,
}

impl ResponseParts {
    fn retry_after(&self) -> Option<Duration> {
        self.meta.retry_after
    }

    fn status_code(&self) -> reqwest::StatusCode {
        self.meta.status_code()
    }
}

impl HttpClient {
    pub(crate) fn with_client(
        client: reqwest::Client,
        observer: Option<ObserverHandle>,
        retry_config: RetryConfig,
        rate_limiter: RateLimiter,
    ) -> Self {
        Self {
            client,
            observer,
            retry_config,
            rate_limiter,
        }
    }

    pub(crate) fn from_timeout(
        timeout: Duration,
        observer: Option<ObserverHandle>,
        retry_config: RetryConfig,
        rate_limiter: RateLimiter,
    ) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(Error::from_reqwest)?;

        Ok(Self::with_client(
            client,
            observer,
            retry_config,
            rate_limiter,
        ))
    }

    pub(crate) async fn get_json<T>(
        &self,
        base_url: &str,
        endpoint: Endpoint,
        auth: &Auth,
        query: Vec<(String, String)>,
    ) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let _permit = self.rate_limiter.acquire().await?;
        let response = self
            .send_with_retry(base_url, &endpoint, auth, &query)
            .await?;
        let ResponseParts { meta, body } = response;

        if meta.status_code() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::from_rate_limited(meta, body));
        }

        if !meta.status_code().is_success() {
            return Err(Error::from_http_status(meta, body));
        }

        let parsed = self.parse_json_body(&body)?;

        if let Some(observer) = &self.observer {
            observer.on_response(&meta);
        }

        Ok(parsed)
    }

    fn build_get_request(
        &self,
        base_url: &str,
        endpoint: &Endpoint,
        auth: &Auth,
        query: &[(String, String)],
    ) -> Result<(String, reqwest::RequestBuilder), Error> {
        let path = endpoint.path();
        let url = format!("{}{}", base_url.trim_end_matches('/'), path.as_ref());
        let request = self.client.get(&url).query(query);

        auth.apply(request, endpoint.requires_auth())
            .map(|request| (url, request))
    }

    async fn send_once(
        &self,
        endpoint: &Endpoint,
        url: String,
        attempt_count: u32,
        started_at: Instant,
        request: reqwest::RequestBuilder,
    ) -> Result<ResponseParts, Error> {
        let response = request.send().await.map_err(Error::from_reqwest)?;
        let status = response.status();
        let meta = ResponseMeta::from_response(
            endpoint.name(),
            url,
            status,
            response.headers(),
            attempt_count,
            started_at.elapsed(),
        );
        let body = response.text().await.map_err(Error::from_reqwest)?;

        Ok(ResponseParts { meta, body })
    }

    async fn send_with_retry(
        &self,
        base_url: &str,
        endpoint: &Endpoint,
        auth: &Auth,
        query: &[(String, String)],
    ) -> Result<ResponseParts, Error> {
        let started_at = Instant::now();
        let mut attempt = 0;

        loop {
            let (url, request) = self.build_get_request(base_url, endpoint, auth, query)?;
            let response = self
                .send_once(endpoint, url, attempt, started_at, request)
                .await?;
            let retry_after = response.retry_after();
            let elapsed = response.meta.elapsed;

            match self.retry_config.classify_status(
                response.status_code(),
                attempt,
                retry_after,
                elapsed,
            ) {
                RetryDecision::DoNotRetry => return Ok(response),
                RetryDecision::RetryAfter(wait) => {
                    attempt += 1;
                    tokio::time::sleep(wait).await;
                }
            }
        }
    }

    fn parse_json_body<T>(&self, body: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_str(body).map_err(|error| Error::Deserialize(error.to_string()))
    }
}
