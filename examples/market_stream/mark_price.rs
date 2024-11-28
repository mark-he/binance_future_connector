use binance_future_connector::market_stream::mark_price::{UpdateInterval, MarkPriceStream};
use env_logger::Builder;
use binance_future_connector::tungstenite::BinanceWebSocketClient;

fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    // Establish connection
    let mut conn =
        BinanceWebSocketClient::connect_with_url(&binance_future_connector::config::wss_api()).expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &MarkPriceStream::from_symbol("BTCUSDT").update_interval(UpdateInterval::Sec1).into(),
    ]);
    // Read messages
    while let Ok(message) = conn.as_mut().read() {
        let data = message.into_data();
        let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
        log::info!("{}", &string_data);
    }
    // Disconnect
    conn.close().expect("Failed to disconnect");
}
