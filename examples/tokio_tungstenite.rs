use binance_future_connector::{
    market::klines::KlineInterval, market_stream::kline::KlineStream,
    tokio_tungstenite::BinanceWebSocketClient,
};
use env_logger::Builder;
use futures_util::StreamExt;


#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    // Establish connection
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &KlineStream::new("BTCUSDT", KlineInterval::Minutes1).into()
    ])
    .await;
    // Read messages
    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                let data = message.into_data();
                let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
                log::info!("{}", &string_data);
            }
            Err(_) => break,
        }
    }
    // Disconnect
    conn.close().await.expect("Failed to disconnect");
}
