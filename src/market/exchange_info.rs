use crate::http::{request::Request, Method};

/// `GET /api/v3/exchangeInfo`
///
/// Current exchange trading rules and symbol information
///

pub struct ExchangeInfoRequest {
    symbol: Option<String>,
    symbols: Option<Vec<String>>,
}

impl ExchangeInfoRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbols: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn symbols(mut self, symbols: Vec<&str>) -> Self {
        self.symbols = Some(symbols.iter().map(|s| s.to_string()).collect());
        self
    }
}

impl From<ExchangeInfoRequest> for Request {
    fn from(request: ExchangeInfoRequest) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(symbols) = request.symbols {
            params.push((
                "symbols".to_owned(),
                format!("[\"{}\"]", symbols.join("\",\"")),
            ));
        }

        Request {
            path: "/fapi/v1/exchangeInfo".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}

