use binance_future_connector::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    let credentials = Credentials::from_hmac("<Your Key>".to_owned(), "<Your Secret>".to_owned());
    let client = BinanceHttpClient::default().credentials(credentials);

    let request = trade::leverage("BNBUSDT", 20);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
