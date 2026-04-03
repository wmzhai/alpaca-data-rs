use alpaca_data::{Client, Error, crypto};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn rate_limit_maps_retry_after_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/quotes"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("retry-after", "3")
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
            retry_after: Some(3),
            ..
        }
    ));
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
