use crate::http::{request::Request, Method};

/// `GET /fapi/v1/ticker/24hr`
///
/// 24 hour rolling window price change statistics. Careful when accessing this with no symbol.
///
/// * If the symbol is not sent, tickers for all symbols will be returned in an array.
///
pub struct Ticker24hrRequest {
    symbol: Option<String>,
}

impl Ticker24hrRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }
}

impl From<Ticker24hrRequest> for Request {
    fn from(request: Ticker24hrRequest) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        Request {
            path: "/fapi/v1/ticker/24hr".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
