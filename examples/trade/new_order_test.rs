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
    let request = trade::new_order_test(trade::new_order("BNBUSDT", Side::Buy, trade::enums::OrderType::Limit).time_in_force(trade::enums::TimeInForceType::Gtc).price(dec!(535)).quantity(dec!(0.1)));
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
