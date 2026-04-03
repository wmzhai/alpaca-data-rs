use alpaca_data::{Client, crypto};

#[tokio::test]
async fn crypto_latest_quotes_smoke_uses_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let builder = match std::env::var("APCA_API_DATA_URL") {
        Ok(base_url) => Client::builder().base_url(base_url),
        Err(_) => Client::builder(),
    };

    let response = builder
        .build()
        .expect("client should build")
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await
        .expect("real Alpaca crypto latest quotes request should succeed");

    assert!(response.quotes.contains_key("BTC/USD"));
}
