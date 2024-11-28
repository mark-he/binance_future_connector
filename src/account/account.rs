use crate::http::{request::Request, Method};

/// `GET /fapi/v3/account`
/// 
pub struct AccountRequest {
    pub recv_window: Option<i64>,
}

impl AccountRequest {
    pub fn new() -> Self {
        Self {
            recv_window: None,
        }
    }

    pub fn recv_window(mut self, recv_window: i64) {
        self.recv_window = Some(recv_window);
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<AccountRequest> for Request {
    fn from(request: AccountRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v2/account".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}