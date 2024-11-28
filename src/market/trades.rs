use crate::http::{request::Request, Method};

/// `GET /api/v3/trades`
///
/// Get recent trades.

pub struct TradesRequest {
    symbol: String,
    limit: Option<u32>,
}

impl TradesRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            limit: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl From<TradesRequest> for Request {
    fn from(request: TradesRequest) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/fapi/v1/trades".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
