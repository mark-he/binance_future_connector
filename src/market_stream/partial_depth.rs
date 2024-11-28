use crate::websocket::Stream;
use super::enums::{Level, UpdateSpeed};

/// Partial Book Depth Stream
///
/// Top bids and asks, Valid are 5, 10, or 20.
pub struct PartialDepthStream {
    symbol: String,
    levels: Level,
    update_speed: Option<UpdateSpeed>,
}

impl PartialDepthStream {
    pub fn new(symbol: &str, levels: Level) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            levels,
            update_speed: None,
        }
    }

    pub fn update_speed(mut self, speed: UpdateSpeed) -> Self {
        self.update_speed = Some(speed);
        self
    }
}

impl From<PartialDepthStream> for Stream {
    fn from(stream: PartialDepthStream) -> Stream {
        if let Some(s) = stream.update_speed {
            Stream::new(&format!("{}@depth{}@{}", stream.symbol, stream.levels, s))
        } else {
            Stream::new(&format!("{}@depth{}", stream.symbol, stream.levels))
        }
    }
}
