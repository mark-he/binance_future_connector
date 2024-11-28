use crate::http::{request::Request, Method};

/// `GET /fapi/v1/allOrders`
///
/// Get all account orders; active, canceled, or filled..
///
/// * If `orderId` is set, it will get orders &gt;= that `orderId`. Otherwise most recent orders are returned.
/// * For some historical orders `cummulativeQuoteQty` will be &lt; 0, meaning the data is not available at this time.
/// * If `startTime` and/or `endTime` provided, `orderId` is not required
///
pub struct AllOrdersRequest {
    symbol: String,
    order_id: Option<u64>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
    recv_window: Option<u64>,
}

impl AllOrdersRequest {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            recv_window: None,
        }
    }

    pub fn order_id(mut self, order_id: u64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

impl From<AllOrdersRequest> for Request {
    fn from(request: AllOrdersRequest) -> Request {
        let mut params = vec![("symbol".to_owned(), request.symbol.to_string())];

        if let Some(order_id) = request.order_id {
            params.push(("orderId".to_owned(), order_id.to_string()));
        }

        if let Some(start_time) = request.start_time {
            params.push(("startTime".to_owned(), start_time.to_string()));
        }

        if let Some(end_time) = request.end_time {
            params.push(("endTime".to_owned(), end_time.to_string()));
        }

        if let Some(limit) = request.limit {
            params.push(("limit".to_owned(), limit.to_string()));
        }

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/fapi/v1/allOrders".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}
