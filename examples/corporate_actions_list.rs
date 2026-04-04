use std::error::Error;

use alpaca_data::{Client, corporate_actions};

fn authenticated_client() -> Result<Client, Box<dyn Error>> {
    let mut builder = Client::builder()
        .api_key(std::env::var("APCA_API_KEY_ID")?)
        .secret_key(std::env::var("APCA_API_SECRET_KEY")?);

    if let Ok(base_url) = std::env::var("APCA_API_DATA_URL") {
        builder = builder.base_url(base_url);
    }

    Ok(builder.build()?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = authenticated_client()?;

    let response = client
        .corporate_actions()
        .list(corporate_actions::ListRequest {
            symbols: Some(vec!["AAPL".into()]),
            cusips: None,
            types: Some(vec![corporate_actions::CorporateActionType::CashDividend]),
            start: Some("2024-08-01".into()),
            end: Some("2024-08-31".into()),
            ids: None,
            limit: Some(5),
            sort: Some(corporate_actions::Sort::Desc),
            page_token: None,
        })
        .await?;

    println!("{response:#?}");
    Ok(())
}
