use alpaca_data::{Client, crypto};

#[tokio::test]
async fn crypto_loc_variants_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let builder = match std::env::var("APCA_API_DATA_URL") {
        Ok(base_url) => Client::builder().base_url(base_url),
        Err(_) => Client::builder(),
    };

    let client = builder.build().expect("client should build");

    client
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us2),
        })
        .await
        .expect("real Alpaca crypto latest quotes request for us-2 should succeed");

    client
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Bs1),
        })
        .await
        .expect("real Alpaca crypto latest quotes request for bs-1 should succeed");
}
