use crate::websocket::Stream;
use strum::Display;


/// Mark Price Stream
///
/// Mark price and funding rate for a single symbol pushed every 3 seconds or every second.
///
/// Update Speed: Real-time.
///
/// [API Documentation](https://developers.binance.com/docs/derivatives/usds-margined-futures/websocket-market-streams/Mark-Price-Stream)
///

#[derive(Copy, Clone, Display)]
pub enum UpdateInterval {
    #[strum(serialize = "1s")]
    Sec1,
    #[strum(serialize = "3s")]
    Sec3,
}
pub struct MarkPriceStream {
    symbol: Option<String>,
    update_interval: Option<UpdateInterval>,
}

impl MarkPriceStream {
    pub fn all_symbols() -> Self {
        Self { 
            symbol: None,
            update_interval: None,
        }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        Self {
            symbol: Some(symbol.to_lowercase()),
            update_interval: None,
        }
    }

    pub fn update_interval(mut self, interval: UpdateInterval) -> Self {
        self.update_interval = Some(interval);
        self
    }
}

impl From<MarkPriceStream> for Stream {
    fn from(stream: MarkPriceStream) -> Stream {
        if let Some(symbol) = stream.symbol {
            let mut s = format!("{}@markPrice", symbol);
            if let Some(interval) = stream.update_interval {
                s = format!("{}@{}", s, interval.to_owned());
            }
            Stream::new(&s)
        } else {
            Stream::new("!markPrice@arr")
        }
    }
}
