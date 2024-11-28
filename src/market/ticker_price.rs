use crate::http::{request::Request, Method};

/// `GET /fapi/v2/ticker/price`
///
/// Latest price for a symbol or symbols.
///
/// * If the symbol is not sent, prices for all symbols will be returned in an array.
///
pub struct TickerPriceRequest {
    symbol: Option<String>,
}

impl TickerPriceRequest {
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

impl From<TickerPriceRequest> for Request {
    fn from(request: TickerPriceRequest) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        Request {
            path: "/fapi/v2/ticker/price".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
