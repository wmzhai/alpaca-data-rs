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
async fn stocks_metadata_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();

    let condition_codes = client
        .stocks()
        .condition_codes(stocks::ConditionCodesRequest {
            ticktype: stocks::TickType::Trade,
            tape: stocks::Tape::A,
        })
        .await
        .expect("real Alpaca stocks condition_codes request should succeed");
    assert!(
        !condition_codes.is_empty(),
        "condition_codes response should include at least one official code"
    );

    let exchange_codes = client
        .stocks()
        .exchange_codes()
        .await
        .expect("real Alpaca stocks exchange_codes request should succeed");
    assert!(
        exchange_codes.contains_key("V"),
        "exchange_codes response should include the IEX exchange code"
    );
}
