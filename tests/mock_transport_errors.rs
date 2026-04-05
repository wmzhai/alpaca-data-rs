use std::time::Duration;

use alpaca_data::{Client, Error, crypto};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn latest_quotes_body() -> &'static str {
    r#"{"quotes":{"BTC/USD":{"ap":67005.5,"as":1.26733,"bp":66894.8,"bs":2.56753,"t":"2026-04-04T00:00:04.184229364Z"}}}"#
}

#[tokio::test]
async fn rate_limit_maps_retry_after_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "3")
                .insert_header("apca-request-id", "req-429")
                .set_body_string("too many requests"),
        )
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(
        error,
        Error::RateLimited {
            endpoint: "crypto.latest_quotes",
            retry_after: Some(3),
            request_id: Some(ref request_id),
            attempt_count: 0,
            ..
        } if request_id == "req-429"
    ));

    assert_eq!(error.endpoint(), Some("crypto.latest_quotes"));
    assert_eq!(error.request_id(), Some("req-429"));
}

#[tokio::test]
async fn malformed_json_maps_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn retry_on_429_can_succeed_when_enabled() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "0")
                .set_body_string("too many requests"),
        )
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(latest_quotes_body(), "application/json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let response = Client::builder()
        .base_url(server.uri())
        .max_retries(1)
        .retry_on_429(true)
        .respect_retry_after(true)
        .base_backoff(Duration::from_millis(1))
        .max_backoff(Duration::from_millis(1))
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect("request should succeed after retry");

    assert!(response.quotes.contains_key("BTC/USD"));
    assert_eq!(
        server
            .received_requests()
            .await
            .expect("requests should be recorded")
            .len(),
        2
    );
}

#[tokio::test]
async fn server_errors_retry_and_then_succeed_within_budget() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(ResponseTemplate::new(500).set_body_string("server error"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(latest_quotes_body(), "application/json"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let response = Client::builder()
        .base_url(server.uri())
        .max_retries(1)
        .base_backoff(Duration::from_millis(1))
        .max_backoff(Duration::from_millis(1))
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect("request should succeed after retry");

    assert!(response.quotes.contains_key("BTC/USD"));
}

#[tokio::test]
async fn server_errors_return_terminal_status_after_retry_budget_is_exhausted() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(500)
                .insert_header("apca-request-id", "req-500")
                .set_body_string("server error"),
        )
        .expect(2)
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .max_retries(1)
        .base_backoff(Duration::from_millis(1))
        .max_backoff(Duration::from_millis(1))
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail after retry budget is exhausted");

    assert!(matches!(
        error,
        Error::HttpStatus {
            endpoint: "crypto.latest_quotes",
            status: 500,
            request_id: Some(ref request_id),
            attempt_count: 1,
            ..
        } if request_id == "req-500"
    ));
    assert_eq!(error.endpoint(), Some("crypto.latest_quotes"));
    assert_eq!(error.request_id(), Some("req-500"));
}

#[tokio::test]
async fn error_bodies_are_snippets_and_display_transport_metadata() {
    let server = MockServer::start().await;
    let long_body = "x".repeat(1024);

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(500)
                .insert_header("apca-request-id", "req-long")
                .set_body_string(long_body.clone()),
        )
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail");

    match &error {
        Error::HttpStatus {
            endpoint,
            request_id,
            body,
            ..
        } => {
            assert_eq!(*endpoint, "crypto.latest_quotes");
            assert_eq!(request_id.as_deref(), Some("req-long"));
            let body = body.as_deref().expect("body snippet should be preserved");
            assert!(body.len() < long_body.len());
            assert!(body.ends_with("..."));
        }
        other => panic!("expected HttpStatus error, got {other:?}"),
    }

    let display = error.to_string();
    assert!(display.contains("crypto.latest_quotes"));
    assert!(display.contains("req-long"));
}
