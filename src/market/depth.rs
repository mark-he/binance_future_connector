use crate::http::{request::Request, Method};

/// `GET /api/v3/depth`
///
/// | Limit               | Weight(IP)  |
/// |---------------------|-------------|
/// | 1-100               | 1           |
/// | 101-500             | 5           |
/// | 501-1000            | 10          |
/// | 1001-5000           | 50          |
///
/// # Example
///
/// ```
/// use binance_spot_connector::market;
///
/// let request = market::depth("BNBUSDT").limit(100);
/// ```
pub struct Depth {
    symbol: String,
    limit: Option<u32>,
}

impl Depth {
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

impl From<Depth> for Request {
    fn from(request: Depth) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        Request {
            path: "/fapi/v1/depth".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}