use crate::http::{request::Request, Method};

/// `GET /fapi/v1/openInterest`
///
/// Query funding rate info for symbols that had FundingRateCap/ FundingRateFloor / fundingIntervalHours adjustment
/// ```
pub struct OpenInterestRequest {
    symbol: String,
}

impl OpenInterestRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
        }
    }
}

impl From<OpenInterestRequest> for Request {
    fn from(request: OpenInterestRequest) -> Request {
        let params = vec![
            ("symbol".to_owned(), request.symbol),
        ];

        Request {
            path: "/fapi/v1/openInterest".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}
