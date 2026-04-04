use alpaca_data::{Client, crypto};

#[tokio::test]
async fn crypto_historical_endpoints_use_real_api_without_credentials() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let builder = match std::env::var("APCA_API_DATA_URL") {
        Ok(base_url) => Client::builder().base_url(base_url),
        Err(_) => Client::builder(),
    };

    let client = builder.build().expect("client should build");

    let bars = client
        .crypto()
        .bars(crypto::BarsRequest {
            symbols: vec!["BTC/USD".into()],
            timeframe: crypto::TimeFrame::from("1Min"),
            start: Some("2026-04-04T00:00:00Z".into()),
            end: Some("2026-04-04T00:02:00Z".into()),
            limit: Some(2),
            sort: Some(crypto::Sort::Asc),
            loc: Some(crypto::Loc::Us),
            page_token: None,
        })
        .await
        .expect("real Alpaca crypto bars request should succeed without credentials");
    assert!(bars.bars.contains_key("BTC/USD"));

    let quotes = client
        .crypto()
        .quotes(crypto::QuotesRequest {
            symbols: vec!["BTC/USD".into()],
            start: Some("2026-04-04T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:05Z".into()),
            limit: Some(1),
            sort: Some(crypto::Sort::Asc),
            loc: Some(crypto::Loc::Us1),
            page_token: None,
        })
        .await
        .expect("real Alpaca crypto quotes request should succeed without credentials");
    assert!(quotes.quotes.contains_key("BTC/USD"));

    let trades = client
        .crypto()
        .trades(crypto::TradesRequest {
            symbols: vec!["BTC/USD".into()],
            start: Some("2026-04-04T00:01:00Z".into()),
            end: Some("2026-04-04T00:01:03Z".into()),
            limit: Some(1),
            sort: Some(crypto::Sort::Asc),
            loc: Some(crypto::Loc::Eu1),
            page_token: None,
        })
        .await
        .expect("real Alpaca crypto trades request should succeed without credentials");
    assert!(trades.trades.contains_key("BTC/USD"));
}

#[tokio::test]
async fn crypto_historical_convenience_layer_collects_real_pages() {
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
        .bars_all(crypto::BarsRequest {
            symbols: vec!["BTC/USD".into()],
            timeframe: crypto::TimeFrame::from("1Min"),
            start: Some("2026-04-04T00:00:00Z".into()),
            end: Some("2026-04-04T00:03:00Z".into()),
            limit: Some(1),
            sort: Some(crypto::Sort::Asc),
            loc: Some(crypto::Loc::Us),
            page_token: None,
        })
        .await
        .expect("real Alpaca crypto bars_all request should succeed without credentials");

    assert!(response.bars.contains_key("BTC/USD"));
    assert_eq!(response.next_page_token, None);
}
