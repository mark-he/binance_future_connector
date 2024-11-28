use crate::http::{request::Request, Method};

/// `GET /fapi/v1/order`
///
/// Check an order's status.
///
/// * Either `orderId` or `origClientOrderId` must be sent.
/// * For some historical orders `cummulativeQuoteQty` will be &lt; 0, meaning the data is not available at this time.

pub struct GetOrderRequest {
    symbol: String,
    order_id: Option<u64>,
    orig_client_order_id: Option<String>,
    recv_window: Option<u64>,
}

impl GetOrderRequest {
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
}

impl From<GetOrderRequest> for Request {
    fn from(request: GetOrderRequest) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(order_id) = request.order_id {
            params.push(("orderId".to_owned(), order_id.to_string()));
        }

        if let Some(orig_client_order_id) = request.orig_client_order_id {
            params.push(("origClientOrderId".to_owned(), orig_client_order_id));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/fapi/v1/order".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}