use crate::http::{request::Request, Method};

/// `DELETE /fapi/v1/batchOrders`
///
/// Cancel an active order.
///
/// Either `orderId` or `origClientOrderId` must be sent.
///
pub struct CancelMultiOrderRequest {
    pub symbol: String,
    pub order_id_list: Option<Vec<i64>>,
    pub orig_client_order_id_list: Option<Vec<String>>,
    pub recv_window: Option<i64>,
}

impl CancelMultiOrderRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
            order_id_list: None,
            orig_client_order_id_list: None,
            recv_window: None,
        }
    }

    pub fn order_id_list(mut self, order_id_list: Vec<i64>) -> Self {
        self.order_id_list = Some(order_id_list);
        self
    }

    pub fn orig_client_order_id_list(mut self, orig_client_order_id_list: Vec<String>) -> Self {
        self.orig_client_order_id_list = Some(orig_client_order_id_list);
        self
    }

    pub fn set_recv_window(&mut self, recv_window: i64) {
        self.recv_window = Some(recv_window);
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("symbol".to_owned(), self.symbol.clone()));

        if let Some(order_id_list) = &self.order_id_list {
            let order_id_str = order_id_list.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
            params.push(("orderIdList".to_owned(), format!("[{}]", order_id_str)));
        }

        if let Some(orig_client_order_id_list) = &self.orig_client_order_id_list {
            let orig_client_order_id_str = orig_client_order_id_list.iter().map(|id| id.clone()).collect::<Vec<String>>().join(",");
            params.push(("origClientOrderIdList".to_owned(), format!("[{}]", orig_client_order_id_str)));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<CancelMultiOrderRequest> for Request {
    fn from(request: CancelMultiOrderRequest) -> Request {
        let params = request.get_params();

        Request {
            path: "/fapi/v1/batchOrders".to_owned(),
            method: Method::Delete,
            params,
            credentials: None,
            sign: true,
        }
    }
}
