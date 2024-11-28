use crate::market::contract_klines::ContractType;
use crate::market::klines::KlineInterval;
use crate::websocket::Stream;

/// Continuous Contract Kline/Candlestick Streams
///
pub struct ContractKlineStream {
    pair: String,
    contract_type: ContractType,
    interval: KlineInterval,
}

impl ContractKlineStream {
    pub fn new(pair: &str, contract_type: ContractType, interval: KlineInterval) -> Self {
        Self {
            pair: pair.to_lowercase(),
            contract_type: contract_type,
            interval,
        }
    }
}

impl From<ContractKlineStream> for Stream {
    fn from(stream: ContractKlineStream) -> Stream {
        Stream::new(&format!("{}_{}@continuousKline_{}", stream.pair, stream.contract_type.to_string().to_lowercase(), stream.interval))
    }
}
