use binance_future_connector::{
    http::Credentials,
    hyper::{BinanceHttpClient, Error},
    trade::{self, enums::Side},
};
use env_logger::Builder;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    let credentials = Credentials::from_hmac("<Your Key>".to_owned(), "<Your Secret>".to_owned());
    let client =
        BinanceHttpClient::default().credentials(credentials);
    let request = trade::new_multi_order().add(trade::new_order("BNBUSDT", Side::Sell, trade::enums::OrderType::Market).quantity(dec!(0.1)));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
