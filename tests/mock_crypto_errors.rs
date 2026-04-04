use alpaca_data::{Client, Error, crypto};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn malformed_snapshots_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/snapshots"))
        .and(query_param("symbols", "BTC/USD"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .crypto()
        .snapshots(crypto::SnapshotsRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}

#[tokio::test]
async fn malformed_latest_orderbooks_json_maps_to_deserialize_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1beta3/crypto/us/latest/orderbooks"))
        .and(query_param("symbols", "BTC/USD"))
        .respond_with(ResponseTemplate::new(200).set_body_raw("not-json", "application/json"))
        .mount(&server)
        .await;

    let error = Client::builder()
        .base_url(server.uri())
        .build()
        .expect("client should build")
        .crypto()
        .latest_orderbooks(crypto::LatestOrderbooksRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect_err("request should fail");

    assert!(matches!(error, Error::Deserialize(_)));
}
