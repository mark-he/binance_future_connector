use crate::http::{request::Request, Method};
use super::enums::{PositionMarginType, PositionSide};
use rust_decimal::Decimal;

/// `GET /fapi/v1/positionMargin`

pub struct PositionMarginRequest {
    pub symbol: String,
    pub position_side: Option<PositionSide>,
    pub amount: Decimal,
    pub type_: PositionMarginType,
    pub recv_window: Option<i64>,
}

impl PositionMarginRequest {
    pub fn new(symbol: &str, amount: Decimal, type_: PositionMarginType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            position_side: None,
            amount,
            type_,
            recv_window: None,
        }
    }

    pub fn position_side(mut self, position_side: PositionSide) -> Self {
        self.position_side = Some(position_side);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("symbol".to_owned(), self.symbol.clone()));
        params.push(("amount".to_owned(), self.amount.to_string()));
        params.push(("type".to_owned(), self.type_.to_string()));

        if let Some(position_side) = &self.position_side {
            params.push(("positionSide".to_owned(), position_side.to_string()));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<PositionMarginRequest> for Request {
    fn from(request: PositionMarginRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v1/positionMargin".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}