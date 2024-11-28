use crate::http::{request::Request, Method};
use super::klines::KlineInterval;


/// `GET /fapi/v1/premiumIndexKlines`
///
/// Premium index kline bars of a symbol. Klines are uniquely identified by their open time.
///

pub struct PremiumIndexKlinesRequest {
    symbol: String,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl PremiumIndexKlinesRequest {
    pub fn new(symbol: &str, interval: KlineInterval) -> Self {
        Self {
            symbol: symbol.to_owned(),
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

impl From<PremiumIndexKlinesRequest> for Request {
    fn from(request: PremiumIndexKlinesRequest) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol),
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
            path: "/fapi/v1/premiumIndexKlines".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
