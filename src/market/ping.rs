use crate::http::{request::Request, Method};

/// `GET /api/v3/ping`
///
/// Test connectivity to the Rest API.
///

pub struct PingRequest {}

impl PingRequest {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<PingRequest> for Request {
    fn from(_request: PingRequest) -> Request {
        let params = vec![];

        Request {
            path: "/fapi/v1/ping".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
