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
        .bars_all(stocks::BarsRequest {
            symbols: vec!["AAPL".into()],
            timeframe: stocks::TimeFrame::from("1Min"),
            start: Some("2026-04-04T13:30:00Z".into()),
            end: Some("2026-04-04T13:35:00Z".into()),
            limit: Some(2),
            adjustment: None,
            feed: None,
            sort: Some(stocks::Sort::Asc),
            asof: None,
            currency: None,
            page_token: None,
        })
        .await?;

    println!("{response:#?}");
    Ok(())
}
