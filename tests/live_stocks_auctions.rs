use alpaca_data::{Client, stocks};

fn live_test_client() -> Client {
    let mut builder = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"));

    if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder = builder.base_url(base_url);
    }

    builder.build().expect("client should build")
}

#[tokio::test]
async fn stocks_auctions_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();
    let batch = client
        .stocks()
        .auctions(stocks::AuctionsRequest {
            symbols: vec!["AAPL".into()],
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-02T00:00:00Z".into()),
            limit: Some(1),
            feed: Some(stocks::AuctionFeed::Sip),
            ..Default::default()
        })
        .await
        .expect("real Alpaca stocks auctions request should succeed");
    assert_eq!(batch.auctions.get("AAPL").map(Vec::len), Some(1));

    let single = client
        .stocks()
        .auctions_single(stocks::AuctionsSingleRequest {
            symbol: "AAPL".into(),
            start: Some("2024-03-01T00:00:00Z".into()),
            end: Some("2024-03-02T00:00:00Z".into()),
            limit: Some(1),
            feed: Some(stocks::AuctionFeed::Sip),
            ..Default::default()
        })
        .await
        .expect("real Alpaca stocks auctions_single request should succeed");
    assert_eq!(single.symbol, "AAPL");
    assert_eq!(single.auctions.len(), 1);
}
