use crate::http::{request::Request, Method};


/// `GET /fapi/v1/fundingRate`
///
/// Premium index kline bars of a symbol. Klines are uniquely identified by their open time.
///

pub struct FundingRateRequest {
    symbol: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl FundingRateRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
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

impl From<FundingRateRequest> for Request {
    fn from(request: FundingRateRequest) -> Request {
        let mut params = vec![
            ("symbol".to_owned(), request.symbol),
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
            path: "/fapi/v1/fundingRate".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
