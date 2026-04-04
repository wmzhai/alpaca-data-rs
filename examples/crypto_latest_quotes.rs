use std::error::Error;

use alpaca_data::{Client, crypto};

fn crypto_client() -> Result<Client, alpaca_data::Error> {
    let builder = match std::env::var("APCA_API_DATA_URL") {
        Ok(base_url) => Client::builder().base_url(base_url),
        Err(_) => Client::builder(),
    };

    builder.build()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = crypto_client()?;

    let response = client
        .crypto()
        .latest_quotes(crypto::LatestQuotesRequest {
            symbols: vec!["BTC/USD".into()],
            loc: Some(crypto::Loc::Us),
        })
        .await?;

    println!("{response:#?}");
    Ok(())
}
