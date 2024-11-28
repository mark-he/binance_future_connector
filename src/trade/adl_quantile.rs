use crate::http::{request::Request, Method};

/// `GET /fapi/v1/adlQuantile`
/// 
pub struct AdlQuantileRequest {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

impl AdlQuantileRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
            recv_window: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) {
        self.symbol = Some(symbol.to_owned());
    }

    pub fn recv_window(mut self, recv_window: i64) {
        self.recv_window = Some(recv_window);
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(symbol) = &self.symbol {
            params.push(("symbol".to_owned(), symbol.clone()));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<AdlQuantileRequest> for Request {
    fn from(request: AdlQuantileRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v1/adlQuantile".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}