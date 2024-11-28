use crate::http::{request::Request, Method};

/// `GET /fapi/v1/ticker/bookTicker`
///
/// Best price/qty on the order book for a symbol or symbols.
///
/// * If the symbol is not sent, bookTickers for all symbols will be returned in an array.
///
/// Weight(IP):
/// * `1` for a single symbol;
/// * `2` when the symbol parameter is omitted;
///
/// # Example
///
/// ```
/// use binance_spot_connector::market;
///
/// let request = market::book_ticker().symbol("BNBUSDT").symbols(vec!["BTCUSDT","BNBBTC"]);
/// ```
pub struct BookTickerRequest {
    symbol: Option<String>,
}

impl BookTickerRequest {
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

impl Default for BookTickerRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BookTickerRequest> for Request {
    fn from(request: BookTickerRequest) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        Request {
            path: "/fapi/v1/ticker/bookTicker".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
