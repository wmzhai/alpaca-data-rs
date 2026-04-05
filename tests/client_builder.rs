use std::time::Duration;

use alpaca_data::{Client, Error};

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
