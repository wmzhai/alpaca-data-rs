use std::error::Error;

use alpaca_data::{Client, news};

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
        .news()
        .list(news::ListRequest {
            start: Some("2026-04-01T00:00:00Z".into()),
            end: Some("2026-04-04T00:00:00Z".into()),
            sort: Some(news::Sort::Desc),
            symbols: Some(vec!["AAPL".into()]),
            limit: Some(5),
            include_content: Some(false),
            exclude_contentless: Some(true),
            page_token: None,
        })
        .await?;

    println!("{response:#?}");
    Ok(())
}
