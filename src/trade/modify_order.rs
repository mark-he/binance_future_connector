use crate::trade::enums::{Side, PriceMatchType};
use crate::http::{request::Request, Method};
use rust_decimal::Decimal;


pub struct ModifyOrderRequest {
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<String>,
    pub symbol: String,
    pub side: Side,
    pub quantity: Decimal,
    pub price: Decimal,
    pub price_match: Option<PriceMatchType>,
    pub recv_window: Option<i64>,
}

impl ModifyOrderRequest {
    pub fn new(symbol: &str, side: Side, quantity: Decimal, price: Decimal) -> Self {
        Self {
            order_id: None,
            orig_client_order_id: None,
            symbol: symbol.to_owned(),
            side,
            quantity,
            price,
            price_match: None,
            recv_window: None,
        }
    }

    pub fn order_id(mut self, order_id: i64) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn orig_client_order_id(mut self, orig_client_order_id: &str) -> Self {
        self.orig_client_order_id = Some(orig_client_order_id.to_owned());
        self
    }

    pub fn price_match(mut self, price_match: PriceMatchType) -> Self {
        self.price_match = Some(price_match);
        self
    }

    pub fn recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("symbol".to_owned(), self.symbol.clone()));
        params.push(("side".to_owned(), self.side.to_string()));
        params.push(("quantity".to_owned(), self.quantity.to_string()));
        params.push(("price".to_owned(), self.price.to_string()));

        if let Some(order_id) = self.order_id {
            params.push(("orderId".to_owned(), order_id.to_string()));
        }

        if let Some(orig_client_order_id) = &self.orig_client_order_id {
            params.push(("origClientOrderId".to_owned(), orig_client_order_id.clone()));
        }

        if let Some(price_match) = &self.price_match {
            params.push(("priceMatch".to_owned(), price_match.to_string()));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }
        params
    }
}

impl From<ModifyOrderRequest> for Request {
    fn from(order: ModifyOrderRequest) -> Request {
        let params = order.get_params();
        Request {
            path: "/fapi/v1/order".to_owned(),
            method: Method::Put,
            params,
            credentials: None,
            sign: true,
        }
    }
}