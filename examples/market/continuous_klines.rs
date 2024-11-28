use binance_future_connector::{
    hyper::{BinanceHttpClient, Error},
    market::{self, contract_klines::ContractType, klines::KlineInterval},
};
use env_logger::Builder;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let client = BinanceHttpClient::default();
    let request = market::contract_klines("BNBUSDT", ContractType::Perpetual, KlineInterval::Hours1)
        .start_time(1729042153105)
        .end_time(1739042153105);
    let data = client.send(request).await?.into_body_str().await?;
    log::info!("{}", data);
    Ok(())
}
