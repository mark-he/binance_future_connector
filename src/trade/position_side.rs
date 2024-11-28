use crate::http::{request::Request, Method};

use super::enums::PositionMode;

/// `POST /fapi/v1/positionSide/dual`
///
/// Change user's position mode (Hedge Mode or One-way Mode ) on EVERY symbol
///

pub struct PositionSideRequest {
    pub dual_side_position: PositionMode,
    pub recv_window: Option<i64>,
}

impl PositionSideRequest {
    pub fn new(dual_side_position: PositionMode) -> Self {
        Self {
            dual_side_position,
            recv_window: None,
        }
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("dualSidePosition".to_owned(), self.dual_side_position.to_string()));

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<PositionSideRequest> for Request {
    fn from(request: PositionSideRequest) -> Request {
        let params = request.get_params();

        Request {
            path: "/fapi/v1/positionSide/dual".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}
