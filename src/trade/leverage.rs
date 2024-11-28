use crate::http::{request::Request, Method};

/// `GET /fapi/v1/leverage`
///
/// Change user's initial leverage of specific symbol market.

pub struct LeverageRequest {
    pub symbol: String,
    pub leverage: i32,
    pub recv_window: Option<i64>,
}

impl LeverageRequest {
    pub fn new(symbol: &str, leverage: i32) -> Self {
        Self {
            symbol: symbol.to_owned(),
            leverage,
            recv_window: None,
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("symbol".to_owned(), self.symbol.clone()));
        params.push(("leverage".to_owned(), self.leverage.to_string()));

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<LeverageRequest> for Request {
    fn from(request: LeverageRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v1/leverage".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}