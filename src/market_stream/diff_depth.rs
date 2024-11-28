use crate::websocket::Stream;

use super::enums::UpdateSpeed;

/// Diff. Depth Stream
///
/// Order book price and quantity depth updates used to locally manage an order book.

pub struct DiffDepthStream {
    symbol: String,
    update_speed: Option<UpdateSpeed>,
}

impl DiffDepthStream {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            update_speed: None,
        }
    }
}

impl From<DiffDepthStream> for Stream {
    fn from(stream: DiffDepthStream) -> Stream {
        if let Some(s) = stream.update_speed {
            Stream::new(&format!("{}@depth@{}", stream.symbol, s))
        } else {
            Stream::new(&format!("{}@depth", stream.symbol))
        }
    }
}
