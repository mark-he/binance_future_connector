use crate::http::{request::Request, Method};

use super::enums::MarginType;

/// `GET /fapi/v1/MarginTypeRequest`
///
/// Get trades for a specific account and symbol.
///



pub struct MarginTypeRequest {
    pub symbol: String,
    pub margin_type: MarginType,
    pub recv_window: Option<i64>,
}

impl MarginTypeRequest {
    pub fn new(symbol: &str, margin_type: MarginType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            margin_type,
            recv_window: None,
        }
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("symbol".to_owned(), self.symbol.clone()));
        params.push(("marginType".to_owned(), self.margin_type.to_string()));
        
        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<MarginTypeRequest> for Request {
    fn from(request: MarginTypeRequest) -> Request {
        let params = request.get_params();

        Request {
            path: "/fapi/v1/marginType".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}
