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
