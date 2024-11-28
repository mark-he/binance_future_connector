use binance_future_connector::{
    hyper::{BinanceHttpClient, Error},
    market,
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let testnet_http_client = BinanceHttpClient::default();
    let request = market::exchange_info().symbol("BNBUSDT");
    let data = testnet_http_client
        .send(request)
        .await?
        .into_body_str()
        .await?;
    log::info!("{}", data);
    Ok(())
}
