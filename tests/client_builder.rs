use std::time::Duration;

use alpaca_data::{Client, Error};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn builder_allows_public_crypto_only_clients() {
    let client = Client::builder()
        .build()
        .expect("public crypto client should build");

    let _ = client.crypto();
}

#[test]
fn builder_rejects_partial_credentials() {
    let error = Client::builder()
        .api_key("key-only")
        .build()
        .expect_err("partial credentials must fail");

    assert!(matches!(
        error,
        Error::InvalidConfiguration(message)
            if message.contains("api_key") && message.contains("secret_key")
    ));
}

#[test]
fn builder_accepts_explicit_shared_runtime_settings() {
    let client = Client::builder()
        .api_key("key")
        .secret_key("secret")
        .base_url("https://data.alpaca.markets")
        .timeout(Duration::from_secs(5))
        .max_retries(2)
        .max_in_flight(32)
        .build()
        .expect("configured client should build");

    let _ = client.stocks();
}

#[test]
fn builder_accepts_structured_retry_settings() {
    let client = Client::builder()
        .base_url("https://data.alpaca.markets")
        .max_retries(2)
        .retry_on_429(true)
        .respect_retry_after(true)
        .base_backoff(Duration::from_millis(10))
        .max_backoff(Duration::from_millis(20))
        .retry_jitter(Duration::from_millis(5))
        .total_retry_budget(Duration::from_millis(50))
        .build()
        .expect("configured retry client should build");

    let _ = client.crypto();
}

#[test]
fn builder_accepts_custom_reqwest_client() {
    let reqwest_client = reqwest::Client::builder()
        .build()
        .expect("custom reqwest client should build");

    let client = Client::builder()
        .reqwest_client(reqwest_client)
        .build()
        .expect("client should build with injected reqwest client");

    let _ = client.crypto();
}

#[test]
fn builder_rejects_timeout_when_custom_reqwest_client_is_injected() {
    let reqwest_client = reqwest::Client::builder()
        .build()
        .expect("custom reqwest client should build");

    let error = Client::builder()
        .timeout(Duration::from_secs(5))
        .reqwest_client(reqwest_client)
        .build()
        .expect_err("timeout should conflict with injected reqwest client");

    assert!(matches!(
        error,
        Error::InvalidConfiguration(message)
            if message.contains("reqwest_client") && message.contains("timeout")
    ));
}

#[tokio::test]
async fn custom_reqwest_client_can_be_used_with_retry_controls() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .and(header("x-custom-client", "phase3"))
        .respond_with(ResponseTemplate::new(500).set_body_string("server error"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .and(header("x-custom-client", "phase3"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"quotes":{"BTC/USD":{"ap":67005.5,"as":1.26733,"bp":66894.8,"bs":2.56753,"t":"2026-04-04T00:00:04.184229364Z"}}}"#,
            "application/json",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let reqwest_client = reqwest::Client::builder()
        .default_headers(
            [(
                reqwest::header::HeaderName::from_static("x-custom-client"),
                reqwest::header::HeaderValue::from_static("phase3"),
            )]
            .into_iter()
            .collect(),
        )
        .build()
        .expect("custom reqwest client should build");

    let response = Client::builder()
        .base_url(server.uri())
        .reqwest_client(reqwest_client)
        .max_retries(1)
        .base_backoff(Duration::from_millis(1))
        .max_backoff(Duration::from_millis(1))
        .build()
        .expect("client should build with injected reqwest client")
        .crypto()
        .latest_quotes(alpaca_data::crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(alpaca_data::crypto::Loc::Us),
        })
        .await
        .expect("request should succeed after retry");

    assert!(response.quotes.contains_key("BTC/USD"));
}
