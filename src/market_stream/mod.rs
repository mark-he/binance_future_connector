//! Binance SPOT Market Websocket Streams
//!
//! A collection of SPOT Market Websocket streams.
pub mod agg_trade;
pub mod book_ticker;
pub mod diff_depth;
pub mod kline;
pub mod mini_ticker;
pub mod partial_depth;
pub mod ticker;
pub mod trade;
pub mod mark_price;
pub mod contract_kline;
pub mod liquidation_order;
pub mod enums;

use crate::market::klines::KlineInterval;

use agg_trade::AggTradeStream;
use book_ticker::BookTickerStream;
use diff_depth::DiffDepthStream;
use enums::Level;
use kline::KlineStream;
use mark_price::MarkPriceStream;
use mini_ticker::MiniTickerStream;
use partial_depth::PartialDepthStream;
use ticker::TickerStream;
use trade::TradeStream;


pub fn agg_trades(symbol: &str) -> AggTradeStream {
    AggTradeStream::new(symbol)
}

pub fn mark_price(symbol: &str) -> MarkPriceStream {
    MarkPriceStream::from_symbol(symbol)
}

pub fn all_market_mark_price() -> MarkPriceStream {
    MarkPriceStream::all_symbols()
}

pub fn individual_symbol_book_ticker(symbol: &str) -> BookTickerStream {
    BookTickerStream::from_symbol(symbol)
}

pub fn all_market_book_ticker() -> BookTickerStream {
    BookTickerStream::all_symbols()
}

pub fn diff_depth(symbol: &str) -> DiffDepthStream {
    DiffDepthStream::new(symbol)
}

pub fn klines(symbol: &str, interval: KlineInterval) -> KlineStream {
    KlineStream::new(symbol, interval)
}

pub fn individual_symbol_mini_ticker(symbol: &str) -> MiniTickerStream {
    MiniTickerStream::from_symbol(symbol)
}

pub fn all_market_mini_ticker() -> MiniTickerStream {
    MiniTickerStream::all_symbols()
}

pub fn liquidation_order(symbol: &str) -> MiniTickerStream {
    MiniTickerStream::from_symbol(symbol)
}

pub fn all_market_liquidation_order() -> MiniTickerStream {
    MiniTickerStream::all_symbols()
}

pub fn partial_depth(symbol: &str, levels: Level) -> PartialDepthStream {
    PartialDepthStream::new(symbol, levels)
}

pub fn individual_symbol_ticker(symbol: &str) -> TickerStream {
    TickerStream::from_symbol(symbol)
}

pub fn all_market_ticker() -> TickerStream {
    TickerStream::all_symbols()
}

pub fn trade_stream(symbol: &str) -> TradeStream {
    TradeStream::new(symbol)
}
