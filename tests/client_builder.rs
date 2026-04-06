use std::time::Duration;
use std::{ffi::OsString, sync::OnceLock};

use alpaca_data::{Client, Error, news};
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
fn builder_rejects_blank_or_whitespace_credentials() {
    for (field, builder) in [
        ("api_key", Client::builder().api_key("").secret_key("secret")),
        ("api_key", Client::builder().api_key("   ").secret_key("secret")),
        ("secret_key", Client::builder().api_key("key").secret_key("")),
        (
            "secret_key",
            Client::builder().api_key("key").secret_key(" \t "),
        ),
    ] {
        let error = builder
            .build()
            .expect_err("blank or whitespace-only credentials must fail");

        assert!(matches!(
            error,
            Error::InvalidConfiguration(message)
                if message.contains(field) && message.contains("blank")
        ));
    }
}

#[test]
fn builder_rejects_header_invalid_credentials() {
    for (field, builder) in [
        (
            "api_key",
            Client::builder()
                .api_key("key\nwrapped")
                .secret_key("secret"),
        ),
        (
            "secret_key",
            Client::builder()
                .api_key("key")
                .secret_key("secret\nwrapped"),
        ),
    ] {
        let error = builder
            .build()
            .expect_err("header-invalid credentials must fail");

        assert!(matches!(
            error,
            Error::InvalidConfiguration(message)
                if message.contains(field) && message.contains("header")
        ));
    }
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

fn env_test_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

struct EnvGuard {
    entries: Vec<(String, Option<OsString>)>,
}

impl EnvGuard {
    fn set(vars: &[(&str, Option<&str>)]) -> Self {
        let entries = vars
            .iter()
            .map(|(name, value)| {
                let previous = std::env::var_os(name);
                match value {
                    Some(value) => unsafe { std::env::set_var(name, value) },
                    None => unsafe { std::env::remove_var(name) },
                }
                (name.to_string(), previous)
            })
            .collect();

        Self { entries }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (name, value) in self.entries.drain(..).rev() {
            match value {
                Some(value) => unsafe { std::env::set_var(&name, value) },
                None => unsafe { std::env::remove_var(&name) },
            }
        }
    }
}

fn news_success_body() -> &'static str {
    r#"{"news":[{"id":24843171,"headline":"Apple headline","author":"Charles Gross","created_at":"2021-12-31T11:08:42Z","updated_at":"2021-12-31T11:08:43Z","summary":"Summary","content":"","url":"https://example.com/article","images":[],"symbols":["AAPL"],"source":"benzinga"}],"next_page_token":null}"#
}

#[tokio::test]
async fn credentials_from_env_loads_default_apca_names() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/news"))
        .and(header("APCA-API-KEY-ID", "env-key"))
        .and(header("APCA-API-SECRET-KEY", "env-secret"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(news_success_body(), "application/json"),
        )
        .mount(&server)
        .await;

    let client = {
        let _lock = env_test_lock()
            .lock()
            .expect("env lock should be available");
        let _env = EnvGuard::set(&[
            ("APCA_API_KEY_ID", Some("env-key")),
            ("APCA_API_SECRET_KEY", Some("env-secret")),
        ]);

        Client::builder()
            .base_url(server.uri())
            .credentials_from_env()
            .expect("env credentials should load")
            .build()
            .expect("client should build")
    };

    let response = client
        .news()
        .list(news::ListRequest::default())
        .await
        .expect("news request should succeed");

    assert_eq!(response.news.len(), 1);
}

#[tokio::test]
async fn credentials_from_env_names_load_custom_variable_names() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta1/news"))
        .and(header("APCA-API-KEY-ID", "custom-key"))
        .and(header("APCA-API-SECRET-KEY", "custom-secret"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(news_success_body(), "application/json"),
        )
        .mount(&server)
        .await;

    let client = {
        let _lock = env_test_lock()
            .lock()
            .expect("env lock should be available");
        let _env = EnvGuard::set(&[
            ("PHASE4_API_KEY", Some("custom-key")),
            ("PHASE4_SECRET_KEY", Some("custom-secret")),
        ]);

        Client::builder()
            .base_url(server.uri())
            .credentials_from_env_names("PHASE4_API_KEY", "PHASE4_SECRET_KEY")
            .expect("custom env credentials should load")
            .build()
            .expect("client should build")
    };

    let response = client
        .news()
        .list(news::ListRequest::default())
        .await
        .expect("news request should succeed");

    assert_eq!(response.news.len(), 1);
}

#[test]
fn credentials_from_env_reject_partial_values() {
    let _lock = env_test_lock()
        .lock()
        .expect("env lock should be available");
    let _env = EnvGuard::set(&[
        ("APCA_API_KEY_ID", Some("env-key")),
        ("APCA_API_SECRET_KEY", None),
    ]);

    let error = Client::builder()
        .credentials_from_env()
        .expect_err("partial env credentials must fail");

    assert!(matches!(
        error,
        Error::InvalidConfiguration(message)
            if message.contains("APCA_API_KEY_ID") && message.contains("APCA_API_SECRET_KEY")
    ));
}
