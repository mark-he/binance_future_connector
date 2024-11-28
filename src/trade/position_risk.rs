use crate::http::{request::Request, Method};

/// `GET /fapi/v3/positionRisk`
/// 
pub struct PositionRiskRequest {
    pub symbol: Option<String>,
    pub recv_window: Option<i64>,
}

impl PositionRiskRequest {
    pub fn new() -> Self {
        Self {
            symbol: None,
            recv_window: None,
        }
    }

    pub fn symbol(mut self, symbol: &str) {
        self.symbol = Some(symbol.to_owned());
    }

    pub fn recv_window(mut self, recv_window: i64) {
        self.recv_window = Some(recv_window);
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(symbol) = &self.symbol {
            params.push(("symbol".to_owned(), symbol.clone()));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<PositionRiskRequest> for Request {
    fn from(request: PositionRiskRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v3/positionRisk".to_owned(),
            method: Method::Get,
            params,
            credentials: None,
            sign: true,
        }
    }
}