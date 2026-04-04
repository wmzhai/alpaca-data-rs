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

#[tokio::test]
async fn options_condition_codes_endpoint_uses_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();

    let trades = client
        .options()
        .condition_codes(options::ConditionCodesRequest {
            ticktype: options::TickType::Trade,
        })
        .await
        .expect("real Alpaca options condition_codes trade request should succeed");
    assert!(
        trades.contains_key("a"),
        "trade condition codes should include the a key"
    );

    let quotes = client
        .options()
        .condition_codes(options::ConditionCodesRequest {
            ticktype: options::TickType::Quote,
        })
        .await
        .expect("real Alpaca options condition_codes quote request should succeed");
    assert!(
        quotes.contains_key(" "),
        "quote condition codes should include the regular trading key"
    );
}
