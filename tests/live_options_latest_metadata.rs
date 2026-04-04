use alpaca_data::{Client, options};

fn live_test_client() -> Client {
    let mut builder = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"));

    if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder = builder.base_url(base_url);
    }

    builder.build().expect("client should build")
}

async fn active_option_contract() -> String {
    let base_url = std::env::var("APCA_API_DATA_URL")
        .unwrap_or_else(|_| "https://data.alpaca.markets".to_string());
    let response = reqwest::Client::new()
        .get(format!(
            "{}/v1beta1/options/snapshots/AAPL",
            base_url.trim_end_matches('/')
        ))
        .header(
            "APCA-API-KEY-ID",
            std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"),
        )
        .header(
            "APCA-API-SECRET-KEY",
            std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"),
        )
        .query(&[("limit", "1")])
        .send()
        .await
        .expect("seed options chain request should succeed")
        .error_for_status()
        .expect("seed options chain response should be successful")
        .json::<serde_json::Value>()
        .await
        .expect("seed options chain response should decode");

    response["snapshots"]
        .as_object()
        .and_then(|items| items.keys().next())
        .expect("seed options chain should return at least one snapshot")
        .to_string()
}

#[tokio::test]
async fn options_latest_and_metadata_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();
    let symbol = active_option_contract().await;

    let latest_quotes = client
        .options()
        .latest_quotes(options::LatestQuotesRequest {
            symbols: vec![symbol.clone()],
            feed: Some(options::OptionsFeed::Indicative),
        })
        .await
        .expect("real Alpaca options latest_quotes request should succeed");
    let latest_quote = latest_quotes
        .quotes
        .get(&symbol)
        .expect("latest_quotes response should include the seeded symbol");
    assert!(
        latest_quote.t.is_some(),
        "latest quote should decode a timestamp"
    );
    assert!(
        latest_quote.bp.is_some(),
        "latest quote should decode a bid price"
    );

    let latest_trades = client
        .options()
        .latest_trades(options::LatestTradesRequest {
            symbols: vec![symbol.clone()],
            feed: Some(options::OptionsFeed::Indicative),
        })
        .await
        .expect("real Alpaca options latest_trades request should succeed");
    let latest_trade = latest_trades
        .trades
        .get(&symbol)
        .expect("latest_trades response should include the seeded symbol");
    assert!(
        latest_trade.t.is_some(),
        "latest trade should decode a timestamp"
    );
    assert!(
        latest_trade.p.is_some(),
        "latest trade should decode a trade price"
    );

    let exchange_codes = client
        .options()
        .exchange_codes()
        .await
        .expect("real Alpaca options exchange_codes request should succeed");
    assert!(
        exchange_codes.contains_key("A"),
        "exchange_codes should include the A venue"
    );
    assert!(
        exchange_codes.contains_key("O"),
        "exchange_codes should include the O venue"
    );
}
