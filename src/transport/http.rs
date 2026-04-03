use crate::{
    Error,
    auth::Auth,
    transport::{endpoint::Endpoint, rate_limit::RateLimiter, retry::RetryPolicy},
};

#[derive(Clone, Debug)]
pub(crate) struct HttpClient {
    client: reqwest::Client,
    retry_policy: RetryPolicy,
    rate_limiter: RateLimiter,
}

impl HttpClient {
    pub(crate) fn new(
        timeout: std::time::Duration,
        retry_policy: RetryPolicy,
        rate_limiter: RateLimiter,
    ) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(Error::from_reqwest)?;

        Ok(Self {
            client,
            retry_policy,
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
        let path = endpoint.path();
        let url = format!("{}{}", base_url.trim_end_matches('/'), path.as_ref());
        let mut attempt = 0;

        loop {
            let request = self.client.get(&url).query(&query);
            let request = auth.apply(request, endpoint.requires_auth())?;
            let response = request.send().await.map_err(Error::from_reqwest)?;
            let status = response.status();
            let retry_after = parse_retry_after(response.headers());
            let body = response.text().await.map_err(Error::from_reqwest)?;

            if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                return Err(Error::RateLimited {
                    retry_after,
                    body: Some(body),
                });
            }

            if self.retry_policy.should_retry(status, attempt) {
                attempt += 1;
                tokio::time::sleep(self.retry_policy.backoff(attempt)).await;
                continue;
            }

            if !status.is_success() {
                return Err(Error::HttpStatus {
                    status: status.as_u16(),
                    body: Some(body),
                });
            }

            return serde_json::from_str(&body)
                .map_err(|error| Error::Deserialize(error.to_string()));
        }
    }
}

fn parse_retry_after(headers: &reqwest::header::HeaderMap) -> Option<u64> {
    headers
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
}
