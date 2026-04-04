use alpaca_data::{Client, corporate_actions};
use futures_util::StreamExt;

fn live_test_client() -> Client {
    let mut builder = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID").expect("APCA_API_KEY_ID is required"))
        .secret_key(std::env::var("APCA_API_SECRET_KEY").expect("APCA_API_SECRET_KEY is required"));

    if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder = builder.base_url(base_url);
    }

    builder.build().expect("client should build")
}

fn total_items(actions: &corporate_actions::CorporateActions) -> usize {
    actions.forward_splits.len()
        + actions.reverse_splits.len()
        + actions.unit_splits.len()
        + actions.stock_dividends.len()
        + actions.cash_dividends.len()
        + actions.spin_offs.len()
        + actions.cash_mergers.len()
        + actions.stock_mergers.len()
        + actions.stock_and_cash_mergers.len()
        + actions.redemptions.len()
        + actions.name_changes.len()
        + actions.worthless_removals.len()
        + actions.rights_distributions.len()
        + actions.contract_adjustments.len()
        + actions.partial_calls.len()
        + actions.other.values().map(Vec::len).sum::<usize>()
}

#[tokio::test]
async fn corporate_actions_endpoints_use_real_api() {
    if std::env::var("ALPACA_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping live test; set ALPACA_LIVE_TESTS=1 to enable");
        return;
    }

    let client = live_test_client();

    let list = client
        .corporate_actions()
        .list(corporate_actions::ListRequest {
            symbols: None,
            cusips: None,
            types: Some(vec![
                corporate_actions::CorporateActionType::CashDividend,
                corporate_actions::CorporateActionType::NameChange,
            ]),
            start: Some("2024-08-01".into()),
            end: Some("2024-08-20".into()),
            ids: None,
            limit: Some(2),
            sort: Some(corporate_actions::Sort::Desc),
            page_token: None,
        })
        .await
        .expect("real Alpaca corporate actions list request should succeed");
    assert!(
        total_items(&list.corporate_actions) > 0,
        "corporate actions list should return at least one action"
    );

    let all = client
        .corporate_actions()
        .list_all(corporate_actions::ListRequest {
            symbols: None,
            cusips: None,
            types: None,
            start: None,
            end: None,
            ids: Some(vec![
                "564620f3-dac4-4558-a227-5c8dd6f4d82e".into(),
                "a93b4426-09f5-48a1-b1bc-c8914d1eda06".into(),
            ]),
            limit: None,
            sort: Some(corporate_actions::Sort::Desc),
            page_token: None,
        })
        .await
        .expect("real Alpaca corporate actions list_all request should succeed");
    assert!(
        total_items(&all.corporate_actions) > 0,
        "corporate actions list_all should collect at least one action"
    );
    assert_eq!(all.next_page_token, None);

    let pages = client
        .corporate_actions()
        .list_stream(corporate_actions::ListRequest {
            symbols: None,
            cusips: None,
            types: Some(vec![corporate_actions::CorporateActionType::NameChange]),
            start: Some("2024-08-13".into()),
            end: Some("2024-08-13".into()),
            ids: None,
            limit: Some(1),
            sort: Some(corporate_actions::Sort::Asc),
            page_token: None,
        })
        .take(2)
        .collect::<Vec<_>>()
        .await;
    assert!(
        pages.len() == 2,
        "corporate actions list_stream should yield at least two real pages"
    );
    assert!(pages.iter().all(|page| page.is_ok()));
}
