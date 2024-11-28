use crate::http::{request::Request, Method};

/// `PUT /sapi/v1/userDataStream`
///
/// Keepalive a user data stream to prevent a time out. User data streams will close after 60 minutes. It's recommended to send a ping about every 30 minutes.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector::margin_stream;
///
/// let request = margin_stream::renew_listen_key("listen-key");
/// ```
pub struct RenewListenKey {
    listen_key: String,
}

impl RenewListenKey {
    pub fn new(listen_key: &str) -> Self {
        Self {
            listen_key: listen_key.to_owned(),
        }
    }

}

impl From<RenewListenKey> for Request {
    fn from(request: RenewListenKey) -> Request {
        let params = vec![("listenKey".to_owned(), request.listen_key.to_string())];

        Request {
            path: "/fapi/v1/listenKey".to_owned(),
            method: Method::Put,
            params,
            credentials: None,
            sign: false,
        }
    }
}
