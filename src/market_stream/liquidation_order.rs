use crate::websocket::Stream;

/// Liquidation Order Streams
///
/// The Liquidation Order Snapshot Streams push force liquidation order information for specific symbol. For each symbol，only the latest one liquidation order within 1000ms will be pushed as the snapshot. If no liquidation happens in the interval of 1000ms, no stream will be pushed.
/// 
pub struct LiquidationOrderStream {
    symbol: Option<String>,
}

impl LiquidationOrderStream {
    pub fn all_symbols() -> Self {
        Self { symbol: None }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        Self {
            symbol: Some(symbol.to_lowercase()),
        }
    }
}

impl From<LiquidationOrderStream> for Stream {
    fn from(stream: LiquidationOrderStream) -> Stream {
        if let Some(symbol) = stream.symbol {
            Stream::new(&format!("{}@forceOrder", symbol))
        } else {
            Stream::new("!forceOrder@arr")
        }
    }
}
