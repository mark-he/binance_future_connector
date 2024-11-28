use binance_future_connector::{
    config, hyper::{BinanceHttpClient, Error}, market::{self, klines::KlineInterval}
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    config::enable_prod(true);
    let client = BinanceHttpClient::default();
    let request = market::klines("DOGEUSDT", KlineInterval::Minutes15)
        .limit(1000);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
