use alpaca_data::{Client, crypto};

#[tokio::test]
async fn crypto_snapshots_use_real_api_without_credentials() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let builder = match std::env::var("APCA_API_DATA_URL") {
        Ok(base_url) => Client::builder().base_url(base_url),
        Err(_) => Client::builder(),
    };

    let client = builder.build().expect("client should build");

    let us1 = client
        .crypto()
        .snapshots(crypto::SnapshotsRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us1),
        })
        .await
        .expect("real Alpaca crypto snapshots request should succeed without credentials");
    assert!(us1.snapshots.contains_key("BTC/USD"));

    let eu1 = client
        .crypto()
        .snapshots(crypto::SnapshotsRequest {
            symbols: vec!["ETH/USD".into()],
            loc: Some(crypto::Loc::Eu1),
        })
        .await
        .expect("real Alpaca crypto snapshots request should succeed without credentials");
    assert!(eu1.snapshots.contains_key("ETH/USD"));
}
