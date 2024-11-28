use crate::http::{request::Request, Method};


/// `GET /fapi/v1/premiumIndex`
///
/// Premium index kline bars of a symbol. Klines are uniquely identified by their open time.
///
pub struct PremiumIndexRequest {
    symbol: String,
}

impl PremiumIndexRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

impl From<PremiumIndexRequest> for Request {
    fn from(request: PremiumIndexRequest) -> Request {
        let params = vec![
            ("symbol".to_owned(), request.symbol),
        ];


        Request {
            path: "/fapi/v1/premiumIndex".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
