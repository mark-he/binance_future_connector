use crate::http::{request::Request, Method};

/// `DELETE /sapi/v1/userDataStream`
///
/// Close out a user data stream.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector::margin_stream;
///
/// let request = margin_stream::close_listen_key("listen-key");
/// ```
pub struct CloseListenKey {
    listen_key: String,
}

impl CloseListenKey {
    pub fn new(listen_key: &str) -> Self {
        Self {
            listen_key: listen_key.to_owned(),
        }
    }
}

impl From<CloseListenKey> for Request {
    fn from(request: CloseListenKey) -> Request {
        let params = vec![("listenKey".to_owned(), request.listen_key.to_string())];

        Request {
            path: "/fapi/v1/listenKey".to_owned(),
            method: Method::Delete,
            params,
            credentials: None,
            sign: false,
        }
    }
}