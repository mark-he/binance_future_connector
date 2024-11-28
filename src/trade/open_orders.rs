use crate::http::{request::Request, Method};

/// `GET /fapi/v1/openOrders`
///
/// Get all open orders on a symbol. Careful when accessing this with no symbol.
///

pub struct OpenOrdersRequest {
    symbol: Option<String>,
    recv_window: Option<u64>,
}

impl OpenOrdersRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
            recv_window: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

impl Default for OpenOrdersRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<OpenOrdersRequest> for Request {
    fn from(request: OpenOrdersRequest) -> Request {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/fapi/v1/openOrders".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}
