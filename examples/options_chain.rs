use std::error::Error;

use alpaca_data::{Client, options};

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
        .options()
        .chain(options::ChainRequest {
            underlying_symbol: "AAPL".into(),
            feed: None,
            r#type: Some(options::ContractType::Call),
            strike_price_gte: Some(180.0),
            strike_price_lte: Some(220.0),
            expiration_date: None,
            expiration_date_gte: Some("2026-04-01".into()),
            expiration_date_lte: Some("2026-06-30".into()),
            root_symbol: None,
            updated_since: None,
            limit: Some(5),
            page_token: None,
        })
        .await?;

    println!("{response:#?}");
    Ok(())
}
