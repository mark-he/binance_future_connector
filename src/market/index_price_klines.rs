use crate::http::{request::Request, Method};
use super::klines::KlineInterval;


/// `GET /fapi/v1/indexPriceKlines`
///
/// Kline/candlestick bars for the index price of a pair. Klines are uniquely identified by their open time.
///

pub struct IndexPriceKlinesRequest {
    pair: String,
    interval: KlineInterval,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl IndexPriceKlinesRequest {
    pub fn new(pair: &str, interval: KlineInterval) -> Self {
        Self {
            pair: pair.to_owned(),
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

impl From<IndexPriceKlinesRequest> for Request {
    fn from(request: IndexPriceKlinesRequest) -> Request {
        let mut params = vec![
            ("pair".to_owned(), request.pair),
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
            path: "/fapi/v1/indexPriceKlines".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
