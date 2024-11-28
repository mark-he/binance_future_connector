use binance_future_connector::{
    market::klines::KlineInterval, market_stream::kline::KlineStream,
    wss_keepalive::WssKeepalive
};
use env_logger::Builder;

fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();

    let _ = WssKeepalive::new(&binance_future_connector::config::wss_api()).prepare(|conn| {
        conn.subscribe(vec![
            &KlineStream::new("BTCUSDT", KlineInterval::Minutes1).into(),
        ]);
    }).stream(&mut |m| {
        let data = m.into_data();
        let string_data = String::from_utf8(data).expect("Found invalid UTF-8 chars");
        log::info!("{}", &string_data);
        Ok(true)
    }, false);
}

