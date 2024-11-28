use crate::http::{request::Request, Method};

/// `DELETE /fapi/v1/order`
///
/// Cancel an active order.
///
/// Either `orderId` or `origClientOrderId` must be sent.
///
#[derive(Debug)]
pub struct CancelOrderRequest {
    symbol: String,
    order_id: Option<u64>,
    orig_client_order_id: Option<String>,
    recv_window: Option<u64>,
}

impl CancelOrderRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: None,
            recv_window: None,
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id.to_owned());
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = vec![("symbol".to_owned(), self.symbol.to_string())];

        if let Some(order_id) = self.order_id {
            params.push(("orderId".to_owned(), order_id.to_string()));
        }

        if let Some(orig_client_order_id) = self.orig_client_order_id.as_ref() {
            params.push(("origClientOrderId".to_owned(), orig_client_order_id.clone()));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<CancelOrderRequest> for Request {
    fn from(request: CancelOrderRequest) -> Request {
        let params = request.get_params();

        Request {
            path: "/fapi/v1/order".to_owned(),
            method: Method::Delete,
            params,
            credentials: None,
            sign: true,
        }
    }
}
