use crate::http::{request::Request, Method};

use super::enums::AutoCloseType;

/// `GET /fapi/v1/openOrders`
///
/// Get all open orders on a symbol. Careful when accessing this with no symbol.
///


pub struct ForceOrdersRequest {
    pub symbol: Option<String>,
    pub auto_close_type: Option<AutoCloseType>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i32>,
    pub recv_window: Option<i64>,
}

impl ForceOrdersRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
            auto_close_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        }
    }

    pub fn symbol(&mut self, symbol: &str) {
        self.symbol = Some(symbol.to_owned());
    }

    pub fn auto_close_type(&mut self, auto_close_type: AutoCloseType) {
        self.auto_close_type = Some(auto_close_type);
    }

    pub fn start_time(&mut self, start_time: i64) {
        self.start_time = Some(start_time);
    }

    pub fn end_time(&mut self, end_time: i64) {
        self.end_time = Some(end_time);
    }

    pub fn limit(&mut self, limit: i32) {
        self.limit = Some(limit);
    }

    pub fn recv_window(&mut self, recv_window: i64) {
        self.recv_window = Some(recv_window);
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(symbol) = &self.symbol {
            params.push(("symbol".to_owned(), symbol.clone()));
        }

        if let Some(auto_close_type) = &self.auto_close_type {
            params.push(("autoCloseType".to_owned(), auto_close_type.to_string()));
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
impl From<ForceOrdersRequest> for Request {
    fn from(request: ForceOrdersRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v1/forceOrders".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}
