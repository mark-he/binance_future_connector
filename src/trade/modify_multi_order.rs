use std::collections::HashMap;
use serde_json::json;
use super::modify_order::ModifyOrderRequest;
use crate::http::{request::Request, Method};

pub struct ModifyMultiOrderRequest {
    pub batch_orders: Vec<ModifyOrderRequest>,
    pub recv_window: Option<i64>,
}

impl ModifyMultiOrderRequest {
    pub fn new() -> Self {
        Self {
            batch_orders: vec![],
            recv_window: None,
        }
    }

    pub fn add(mut self, order: ModifyOrderRequest) -> Self {
        self.batch_orders.push(order);
        self
    }
    
    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }
}

impl From<ModifyMultiOrderRequest> for Request {
    fn from(request: ModifyMultiOrderRequest) -> Request {
        let mut v = vec![];
        for order in &request.batch_orders {
            let v_params = order.get_params();
            let mut v_map: HashMap<String, String> = HashMap::new();
            for p in v_params {
                v_map.insert(p.0, p.1);
            }
            v.push(v_map);
        }

        let mut params = Vec::new();
        params.push(("batchOrders".to_owned(), json!(v).to_string()));

        if let Some(recv_window) = request.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        Request {
            path: "/fapi/v1/batchOrders".to_owned(),
            method: Method::Put,
            params,
            credentials: None,
            sign: true,
        }
    }
}
