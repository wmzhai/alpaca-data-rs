use std::time::{Duration, Instant};

use crate::{
    Error,
    auth::Auth,
    transport::{
        endpoint::Endpoint,
        rate_limit::RateLimiter,
        retry::{RetryConfig, RetryDecision},
    },
};

#[derive(Clone, Debug)]
pub(crate) struct HttpClient {
    client: reqwest::Client,
    retry_config: RetryConfig,
    rate_limiter: RateLimiter,
}

#[derive(Debug)]
struct ResponseParts {
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    body: String,
}

impl ResponseParts {
    fn retry_after(&self) -> Option<Duration> {
        parse_retry_after(&self.headers)
    }
}

impl HttpClient {
    pub(crate) fn new(
        timeout: Duration,
        retry_config: RetryConfig,
        rate_limiter: RateLimiter,
    ) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(Error::from_reqwest)?;

        Ok(Self {
            client,
            retry_config,
            rate_limiter,
        })
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
        let retry_after = response.retry_after().map(|value| value.as_secs());
        let ResponseParts { status, body, .. } = response;

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(Error::RateLimited {
                retry_after,
                body: Some(body),
            });
        }

        if !status.is_success() {
            return Err(Error::HttpStatus {
                status: status.as_u16(),
                body: Some(body),
            });
        }

        self.parse_json_body(&body)
    }

    fn build_get_request(
        &self,
        base_url: &str,
        endpoint: &Endpoint,
        auth: &Auth,
        query: &[(String, String)],
    ) -> Result<reqwest::RequestBuilder, Error> {
        let path = endpoint.path();
        let url = format!("{}{}", base_url.trim_end_matches('/'), path.as_ref());
        let request = self.client.get(url).query(query);

        auth.apply(request, endpoint.requires_auth())
    }

    async fn send_once(&self, request: reqwest::RequestBuilder) -> Result<ResponseParts, Error> {
        let response = request.send().await.map_err(Error::from_reqwest)?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.text().await.map_err(Error::from_reqwest)?;

        Ok(ResponseParts {
            status,
            headers,
            body,
        })
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
            let request = self.build_get_request(base_url, endpoint, auth, query)?;
            let response = self.send_once(request).await?;
            let retry_after = response.retry_after();
            let elapsed = started_at.elapsed();

            match self
                .retry_config
                .classify_status(response.status, attempt, retry_after, elapsed)
            {
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

fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_secs)
}
