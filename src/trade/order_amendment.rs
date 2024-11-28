use crate::http::{request::Request, Method};

/// `GET /fapi/v1/orderAmendment`
///
/// Get Order Modify History
///
pub struct OrderAmendmentRequest {
    pub symbol: String,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
    pub recv_window: Option<i64>,
}

impl OrderAmendmentRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            orig_client_order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        }
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id.to_owned());
        self
    }

    pub fn start_time(mut self, start_time: i64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: i64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("symbol".to_owned(), self.symbol.clone()));

        if let Some(order_id) = self.order_id {
            params.push(("orderId".to_owned(), order_id.to_string()));
        }

        if let Some(orig_client_order_id) = &self.orig_client_order_id {
            params.push(("origClientOrderId".to_owned(), orig_client_order_id.clone()));
        }

        if let Some(start_time) = self.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = self.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = self.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }

}

impl From<OrderAmendmentRequest> for Request {
    fn from(request: OrderAmendmentRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v1/orderAmendment".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}