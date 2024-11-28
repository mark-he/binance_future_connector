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
    let client = BinanceHttpClient::default().credentials(credentials);
    let request = trade::modify_multi_order().add(trade::modify_order("BNBUSDT", Side::Buy, dec!(0.1), dec!(595.0)).order_id(480922730));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
