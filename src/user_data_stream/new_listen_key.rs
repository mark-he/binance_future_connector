use crate::http::{request::Request, Method};

/// `POST /sapi/v1/userDataStream`
///
/// Start a new user data stream.
/// The stream will close after 60 minutes unless a keepalive is sent. If the account has an active `listenKey`, that `listenKey` will be returned and its validity will be extended for 60 minutes.
///
/// Weight: 1
///
/// # Example
///
/// ```
/// use binance_spot_connector::margin_stream;
///
/// let request = margin_stream::new_listen_key();
/// ```
pub struct NewListenKey {
}

impl NewListenKey {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl From<NewListenKey> for Request {
    fn from(_request: NewListenKey) -> Request {
        let params = vec![];

        Request {
            path: "/fapi/v1/listenKey".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: false,
        }
    }
}
