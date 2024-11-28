#![allow(clippy::wrong_self_convention)]

use crate::http::{request::Request, Credentials, Method};

/// `GET /api/v3/historicalTrades`
///
/// Get older market trades.

pub struct HistoricalTradesRequest {
    symbol: String,
    limit: Option<u32>,
    from_id: Option<u64>,
    credentials: Option<Credentials>,
}

impl HistoricalTradesRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            limit: None,
            from_id: None,
            credentials: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn from_id(mut self, from_id: u64) -> Self {
        self.from_id = Some(from_id);
        self
    }

    pub fn credentials(mut self, credentials: &Credentials) -> Self {
        self.credentials = Some(credentials.clone());
        self
    }
}

impl From<HistoricalTradesRequest> for Request {
    fn from(request: HistoricalTradesRequest) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol)];

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(from_id) = request.from_id {
            params.push(("fromId".to_owned(), from_id.to_string()));
        }

        Request {
            path: "/fapi/v1/historicalTrades".to_owned(),
            method: Method::Get,
            params,
            credentials: request.credentials,
            sign: false,
        }
    }
}
