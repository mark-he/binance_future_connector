use crate::http::{request::Request, Method};

/// `GET /api/v3/time`
///
/// Test connectivity to the Rest API and get the current server time.
///

/// ```
pub struct TimeRequests {}

impl TimeRequests {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<TimeRequests> for Request {
    fn from(_request: TimeRequests) -> Request {
        let params = vec![];

        Request {
            path: "/fapi/v1/time".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: false,
        }
    }
}
