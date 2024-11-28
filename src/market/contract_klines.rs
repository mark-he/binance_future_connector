use crate::http::{request::Request, Method};
use strum::Display;
use super::klines::KlineInterval;


#[derive(Copy, Clone, Display)]
pub enum ContractType {
    #[strum(serialize = "PERPETUAL")]
    Perpetual,
    #[strum(serialize = "CURRENT_QUARTER")]
    CurrentQuarter,
    #[strum(serialize = "NEXT_QUARTER")]
    NextQuarter
}


/// `GET /fapi/v1/ContractKlines`
///
/// Kline/candlestick bars for a specific contract type. Klines are uniquely identified by their open time.
///
/// * If `startTime` and `endTime` are not sent, the most recent klines are returned.
///
/// Weight(IP): based on parameter LIMIT, 5 by default.
///
/// # Example
///
/// ```
/// use binance_spot_connector::market::{self, klines::KlineInterval};
///
/// let request = market::contract_klines("BTCUSDT", ContractType.Perpetual, KlineInterval::Minutes1)
///     .start_time(1654079109000)
///     .end_time(1654079209000);
/// ```
pub struct ContractKlinesRequest {
    pair: String,
    contract_type: ContractType,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl ContractKlinesRequest {
    pub fn new(pair: &str, contract_type: ContractType, interval: KlineInterval) -> Self {
        Self {
            pair: pair.to_owned(),
            contract_type,
            interval,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl From<ContractKlinesRequest> for Request {
    fn from(request: ContractKlinesRequest) -> Request {
        let mut params = vec![
            ("pair".to_owned(), request.pair),
            ("contractType".to_owned(), request.contract_type.to_string()),
            ("interval".to_owned(), request.interval.to_string()),
        ];

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/fapi/v1/ContractKlines".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
