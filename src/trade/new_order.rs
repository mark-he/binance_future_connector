use crate::http::{request::Request, Method};
use crate::trade::enums::{NewOrderResponseType, Side, TimeInForceType, OrderType, PositionSide, PriceMatchType};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

/// `POST /api/v3/order`
///
/// Send in a new order.
///
/// * `LIMIT_MAKER` are `LIMIT` orders that will be rejected if they would immediately match and trade as a taker.
/// * `STOP_LOSS` and `TAKE_PROFIT` will execute a `MARKET` order when the `stopPrice` is reached.
/// * Any `LIMIT` or `LIMIT_MAKER` type order can be made an iceberg order by sending an `icebergQty`.
/// * Any order with an `icebergQty` MUST have `timeInForce` set to `GTC`.
/// * `MARKET` orders using `quantity` specifies how much a user wants to buy or sell based on the market price.
/// * `MARKET` orders using `quoteOrderQty` specifies the amount the user wants to spend (when buying) or receive (when selling) of the quote asset; the correct quantity will be determined based on the market liquidity and `quoteOrderQty`.
/// * `MARKET` orders using `quoteOrderQty` will not break `LOT_SIZE` filter rules; the order will execute a quantity that will have the notional value as close as possible to `quoteOrderQty`.
/// * same `newClientOrderId` can be accepted only when the previous one is filled, otherwise the order will be rejected.
///
/// Trigger order price rules against market price for both `MARKET` and `LIMIT` versions:
///
/// * Price above market price: `STOP_LOSS` `BUY`, `TAKE_PROFIT` `SELL`
/// * Price below market price: `STOP_LOSS` `SELL`, `TAKE_PROFIT` `BUY`
///

#[derive(Debug, Clone, Serialize, Deserialize,)]
pub struct NewOrderRequest {
    pub symbol: String,
    pub side: Side,
    pub position_side: Option<PositionSide>,
    pub type_: OrderType,
    pub reduce_only: Option<String>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<Decimal>,
    pub close_position: Option<String>,
    pub activation_price: Option<Decimal>,
    pub callback_rate: Option<Decimal>,
    pub time_in_force: Option<TimeInForceType>,
    pub working_type: Option<String>,
    pub price_protect: Option<String>,
    pub new_order_resp_type: Option<NewOrderResponseType>,
    pub price_match: Option<PriceMatchType>,
    pub self_trade_prevention_mode: Option<String>,
    pub good_till_date: Option<i64>,
    pub recv_window: Option<i64>,
}

impl NewOrderRequest {
    pub fn new(symbol: &str, side: Side, type_: OrderType) -> Self {
        Self {
            symbol: symbol.to_owned(),
            side,
            position_side: None,
            type_,
            reduce_only: None,
            quantity: None,
            price: None,
            new_client_order_id: None,
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            time_in_force: None,
            working_type: None,
            price_protect: None,
            new_order_resp_type: None,
            price_match: None,
            self_trade_prevention_mode: None,
            good_till_date: None,
            recv_window: None
        }
    }

    pub fn position_side(mut self, position_side: PositionSide) -> Self {
        self.position_side = Some(position_side);
        self
    }

    pub fn reduce_only(mut self, reduce_only: &str) -> Self {
        self.reduce_only = Some(reduce_only.to_owned());
        self
    }

    pub fn quantity(mut self, quantity: Decimal) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn price(mut self, price: Decimal) -> Self {
        self.price = Some(price);
        self
    }

    pub fn new_client_order_id(mut self, new_client_order_id: &str) -> Self {
        self.new_client_order_id = Some(new_client_order_id.to_owned());
        self
    }

    pub fn stop_price(mut self, stop_price: Decimal) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn close_position(mut self, close_position: &str) -> Self {
        self.close_position = Some(close_position.to_owned());
        self
    }

    pub fn activation_price(mut self, activation_price: Decimal) -> Self {
        self.activation_price = Some(activation_price);
        self
    }

    pub fn callback_rate(mut self, callback_rate: Decimal) -> Self {
        self.callback_rate = Some(callback_rate);
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForceType) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    pub fn working_type(mut self, working_type: &str) -> Self {
        self.working_type = Some(working_type.to_owned());
        self
    }

    pub fn price_protect(mut self, price_protect: &str) -> Self {
        self.price_protect = Some(price_protect.to_owned());
        self
    }

    pub fn new_order_resp_type(mut self, new_order_resp_type: NewOrderResponseType) -> Self {
        self.new_order_resp_type = Some(new_order_resp_type);
        self
    }

    pub fn price_match(mut self, price_match: PriceMatchType) -> Self {
        self.price_match = Some(price_match);
        self
    }

    pub fn self_trade_prevention_mode(mut self, self_trade_prevention_mode: &str) -> Self {
        self.self_trade_prevention_mode = Some(self_trade_prevention_mode.to_owned());
        self
    }

    pub fn good_till_date(mut self, good_till_date: i64) -> Self {
        self.good_till_date = Some(good_till_date);
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
        params.push(("type".to_owned(), self.type_.to_string()));

        if let Some(reduce_only) = self.reduce_only.as_ref() {
            params.push(("reduceOnly".to_owned(), reduce_only.to_string()));
        }

        if let Some(time_in_force) = self.time_in_force {
            params.push(("timeInForce".to_owned(), time_in_force.to_string()));
        }

        if let Some(position_side) = self.position_side {
            params.push(("positionSide".to_owned(), position_side.to_string()));
        }

        if let Some(quantity) = self.quantity {
            params.push(("quantity".to_owned(), quantity.to_string()));
        }

        if let Some(price) = self.price {
            params.push(("price".to_owned(), price.to_string()));
        }

        if let Some(new_client_order_id) = self.new_client_order_id.as_ref() {
            params.push(("newClientOrderId".to_owned(), new_client_order_id.clone()));
        }

        if let Some(stop_price) = self.stop_price {
            params.push(("stopPrice".to_owned(), stop_price.to_string()));
        }

        if let Some(close_position) = self.close_position.as_ref() {
            params.push(("closePosition".to_owned(), close_position.clone()));
        }

        if let Some(activation_price) = self.activation_price {
            params.push(("activationPrice".to_owned(), activation_price.to_string()));
        }

        if let Some(callback_rate) = self.callback_rate {
            params.push(("callbackRate".to_owned(), callback_rate.to_string()));
        }

        if let Some(working_type) = self.working_type.as_ref() {
            params.push(("workingType".to_owned(), working_type.clone()));
        }

        if let Some(price_protect) = self.price_protect.as_ref() {
            params.push(("priceProtect".to_owned(), price_protect.clone()));
        }

        if let Some(new_order_resp_type) = self.new_order_resp_type {
            params.push(("newOrderRespType".to_owned(), new_order_resp_type.to_string()));
        }

        if let Some(price_match) = self.price_match.as_ref() {
            params.push(("priceMatch".to_owned(), price_match.to_string()));
        }

        if let Some(self_trade_prevention_mode) = self.self_trade_prevention_mode.as_ref() {
            params.push(("selfTradePreventionMode".to_owned(), self_trade_prevention_mode.clone()));
        }

        if let Some(good_till_date) = self.good_till_date {
            params.push(("goodTillDate".to_owned(), good_till_date.to_string()));
        }

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }
        params
    }
}

impl From<NewOrderRequest> for Request {
    fn from(request: NewOrderRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v1/order".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}
