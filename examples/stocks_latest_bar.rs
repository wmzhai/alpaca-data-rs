use std::error::Error;

use alpaca_data::{Client, stocks};

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
        .stocks()
        .latest_bar(stocks::LatestBarRequest {
            symbol: "AAPL".into(),
            feed: None,
            currency: None,
        })
        .await?;

    println!("{response:#?}");
    Ok(())
}
