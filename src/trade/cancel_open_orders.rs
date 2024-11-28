use crate::http::{request::Request, Method};

/// `DELETE /fapi/v1/allOpenOrders`
///
/// Cancels all active orders on a symbol.
/// This includes OCO orders.

pub struct CancelOpenOrdersRequest {
    symbol: String,
    recv_window: Option<u64>,
}

impl CancelOpenOrdersRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            recv_window: None,
        }
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

impl From<CancelOpenOrdersRequest> for Request {
    fn from(request: CancelOpenOrdersRequest) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/fapi/v1/allOpenOrders".to_owned(),
            method: Method::Delete,
            params,
            credentials: None,
            sign: true,
        }
    }
}
