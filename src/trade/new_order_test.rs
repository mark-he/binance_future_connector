use crate::http::{request::Request, Method};
use super::new_order::NewOrderRequest;


pub struct NewOrderTestRequest {
    pub new_order: NewOrderRequest,
}

impl NewOrderTestRequest {
    pub fn new(new_order: NewOrderRequest) -> Self {
        Self {
            new_order
        }
    }
}

impl From<NewOrderTestRequest> for Request {
    fn from(request: NewOrderTestRequest) -> Request {
        let params = request.new_order.get_params();
        Request {
            path: "/fapi/v1/order/test".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}
