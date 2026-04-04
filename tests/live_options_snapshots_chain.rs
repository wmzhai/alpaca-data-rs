use alpaca_data::{Client, options};
use futures_util::StreamExt;

#[derive(Clone, Debug)]
struct SeedContract {
    symbols: Vec<String>,
    expiration_date: String,
    contract_type: options::ContractType,
    strike_price: f64,
}

fn live_test_client() -> Client {
    let mut builder = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"));

    if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder = builder.base_url(base_url);
    }

    builder.build().expect("client should build")
}

async fn seed_contract(underlying_symbol: &str, limit: usize) -> SeedContract {
    let base_url = std::env::var("APCA_API_DATA_URL")
        .unwrap_or_else(|_| "https://data.alpaca.markets".to_string());
    let response = reqwest::Client::new()
        .get(format!(
            "{}/v1beta1/options/snapshots/{}",
            base_url.trim_end_matches('/'),
            underlying_symbol
        ))
        .header(
            "APCA-API-KEY-ID",
            std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"),
        )
        .header(
            "APCA-API-SECRET-KEY",
            std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"),
        )
        .query(&[("limit", limit.to_string())])
        .send()
        .await
        .expect("seed options chain request should succeed")
        .error_for_status()
        .expect("seed options chain response should be successful")
        .json::<serde_json::Value>()
        .await
        .expect("seed options chain response should decode");

    let symbols = response["snapshots"]
        .as_object()
        .expect("seed options chain should return a snapshots map")
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    let first_symbol = symbols
        .first()
        .expect("seed options chain should return at least one symbol");
    let root_len = underlying_symbol.len();
    let expiration = &first_symbol[root_len..root_len + 6];
    let expiration_date = format!(
        "20{}-{}-{}",
        &expiration[0..2],
        &expiration[2..4],
        &expiration[4..6]
    );
    let contract_type = match &first_symbol[root_len + 6..root_len + 7] {
        "C" => options::ContractType::Call,
        "P" => options::ContractType::Put,
        value => panic!("unexpected option contract type marker: {value}"),
    };
    let strike_price = first_symbol[first_symbol.len() - 8..]
        .parse::<u32>()
        .expect("seed symbol should include an OCC strike suffix") as f64
        / 1000.0;

    SeedContract {
        symbols,
        expiration_date,
        contract_type,
        strike_price,
    }
}

#[tokio::test]
async fn options_snapshots_and_chain_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();
    let seed = seed_contract("AAPL", 5).await;
    let symbols = seed.symbols.clone();
    assert!(
        symbols.len() >= 3,
        "seed options chain should return at least three symbols"
    );

    let snapshots_request = options::SnapshotsRequest {
        symbols: symbols.clone(),
        feed: None,
        limit: Some(2),
        page_token: None,
    };
    let snapshots = client
        .options()
        .snapshots(snapshots_request.clone())
        .await
        .expect("real Alpaca options snapshots request should succeed");
    assert!(
        !snapshots.snapshots.is_empty(),
        "snapshots response should include at least one symbol"
    );
    assert!(
        snapshots.next_page_token.is_some(),
        "snapshots response should paginate when limit is smaller than symbol count"
    );

    let snapshots_all = client
        .options()
        .snapshots_all(snapshots_request.clone())
        .await
        .expect("real Alpaca options snapshots_all request should succeed");
    assert_eq!(snapshots_all.next_page_token, None);
    for symbol in &symbols {
        assert!(
            snapshots_all.snapshots.contains_key(symbol),
            "snapshots_all response should include every seeded symbol"
        );
    }

    let snapshot_pages = client
        .options()
        .snapshots_stream(snapshots_request)
        .collect::<Vec<_>>()
        .await;
    assert!(
        snapshot_pages.len() >= 2,
        "snapshots_stream should yield multiple pages when pagination is present"
    );
    assert!(snapshot_pages.iter().all(|page| page.is_ok()));

    let chain_request = options::ChainRequest {
        underlying_symbol: "AAPL".into(),
        feed: None,
        r#type: Some(seed.contract_type),
        strike_price_gte: Some(seed.strike_price + 20.0),
        strike_price_lte: Some(seed.strike_price + 50.0),
        expiration_date: Some(seed.expiration_date.clone()),
        expiration_date_gte: None,
        expiration_date_lte: None,
        root_symbol: None,
        updated_since: None,
        limit: Some(3),
        page_token: None,
    };
    let chain = client
        .options()
        .chain(chain_request.clone())
        .await
        .expect("real Alpaca options chain request should succeed");
    assert!(
        !chain.snapshots.is_empty(),
        "chain response should include at least one contract snapshot"
    );
    assert!(
        chain.next_page_token.is_some(),
        "chain response should paginate with a small limit"
    );

    let chain_all = client
        .options()
        .chain_all(chain_request.clone())
        .await
        .expect("real Alpaca options chain_all request should succeed");
    assert_eq!(chain_all.next_page_token, None);
    assert!(
        chain_all.snapshots.len() > chain.snapshots.len(),
        "chain_all should aggregate more contracts than the first page"
    );
    assert!(
        chain_all
            .snapshots
            .values()
            .any(|snapshot| snapshot.latestQuote.is_some()),
        "chain_all should decode latestQuote fields"
    );
    assert!(
        chain_all
            .snapshots
            .values()
            .any(|snapshot| snapshot.greeks.is_some() || snapshot.impliedVolatility.is_some()),
        "chain_all should decode greeks or impliedVolatility when present"
    );

    let chain_pages = client
        .options()
        .chain_stream(chain_request)
        .collect::<Vec<_>>()
        .await;
    assert!(
        chain_pages.len() >= 2,
        "chain_stream should yield multiple pages when pagination is present"
    );
    assert!(chain_pages.iter().all(|page| page.is_ok()));
}
