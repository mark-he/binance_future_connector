//! Account/Trade
//!
//! [API Documentation]()
pub mod all_orders;
pub mod cancel_open_orders;
pub mod cancel_order;
pub mod get_order;
pub mod user_trades;
pub mod new_order;
pub mod new_order_test;
pub mod open_orders;
pub mod enums;
pub mod new_multi_order;
pub mod modify_order;
pub mod modify_multi_order;
pub mod order_amendment;
pub mod cancel_multi_order;
pub mod countdown_cancel_all;
pub mod force_orders;
pub mod margin_type;
pub mod position_side;
pub mod get_open_order;
pub mod leverage;
pub mod multi_assets_margin;
pub mod position_margin;
pub mod position_risk;
pub mod adl_quantile;

use adl_quantile::AdlQuantileRequest;
use cancel_multi_order::CancelMultiOrderRequest;
use cancel_open_orders::CancelOpenOrdersRequest;
use countdown_cancel_all::CountdownCancelAllRequest;
use force_orders::ForceOrdersRequest;
use get_open_order::GetOpenOrderRequest;
use leverage::LeverageRequest;
use margin_type::MarginTypeRequest;
use modify_multi_order::ModifyMultiOrderRequest;
use modify_order::ModifyOrderRequest;
use multi_assets_margin::MultiAssetsMarginRequest;
use new_multi_order::NewMultiOrderRequest;
use order_amendment::OrderAmendmentRequest;
use position_margin::PositionMarginRequest;
use position_risk::PositionRiskRequest;
use position_side::PositionSideRequest;
use rust_decimal::Decimal;
use all_orders::AllOrdersRequest;
use cancel_order::CancelOrderRequest;
use get_order::GetOrderRequest;
use user_trades::UserTradesRequest;
use new_order::NewOrderRequest;
use new_order_test::NewOrderTestRequest;
use open_orders::OpenOrdersRequest;
use enums::{MarginAssetMode, MarginType, OrderType, PositionMarginType, PositionMode, Side};

pub fn new_order_test(new_order: NewOrderRequest) -> NewOrderTestRequest {
    NewOrderTestRequest::new(new_order)
}

pub fn get_open_order(symbol: &str) -> GetOpenOrderRequest {
    GetOpenOrderRequest::new(symbol)
}

pub fn get_order(symbol: &str) -> GetOrderRequest {
    GetOrderRequest::new(symbol)
}

pub fn new_order(symbol: &str, side: Side, type_: OrderType) -> NewOrderRequest {
    NewOrderRequest::new(symbol, side, type_)
}

pub fn modify_order(symbol: &str, side: Side, quantity: Decimal, price: Decimal) -> ModifyOrderRequest {
    ModifyOrderRequest::new(symbol, side, quantity, price)
}

pub fn new_multi_order() -> NewMultiOrderRequest {
    NewMultiOrderRequest::new()
}

pub fn modify_multi_order() -> ModifyMultiOrderRequest {
    ModifyMultiOrderRequest::new()
}

pub fn order_amendment(symbol: &str) -> OrderAmendmentRequest {
    OrderAmendmentRequest::new(symbol)
}

pub fn cancel_order(symbol: &str) -> CancelOrderRequest {
    CancelOrderRequest::new(symbol)
}

pub fn cancel_multi_order(symbol: &str) -> CancelMultiOrderRequest {
    CancelMultiOrderRequest::new(symbol)
}

pub fn cancel_open_orders(symbol: &str) -> CancelOpenOrdersRequest {
    CancelOpenOrdersRequest::new(symbol)
}

pub fn open_orders() -> OpenOrdersRequest {
    OpenOrdersRequest::new()
}

pub fn countdown_cancel_all(symbol: &str, countdown_time: i64) -> CountdownCancelAllRequest {
    CountdownCancelAllRequest::new(symbol, countdown_time)
}

pub fn all_orders(symbol: &str) -> AllOrdersRequest {
    AllOrdersRequest::new(symbol)
}

pub fn force_orders() -> ForceOrdersRequest {
    ForceOrdersRequest::new()
}

pub fn margin_type(symbol: &str, margin_type: MarginType) -> MarginTypeRequest {
    MarginTypeRequest::new(symbol, margin_type)
}

pub fn position_side(dual_side_position: PositionMode) -> PositionSideRequest {
    PositionSideRequest::new(dual_side_position)
}

pub fn user_trades(symbol: &str) -> UserTradesRequest {
    UserTradesRequest::new(symbol)
}

pub fn leverage(symbol: &str, leverage: i32) -> LeverageRequest {
    LeverageRequest::new(symbol, leverage)
}

pub fn multi_assets_margin(mode: MarginAssetMode) -> MultiAssetsMarginRequest {
    MultiAssetsMarginRequest::new(mode)
}

pub fn position_margin(symbol: &str, amount: Decimal, type_: PositionMarginType) -> PositionMarginRequest {
    PositionMarginRequest::new(symbol, amount, type_)
}

pub fn position_risk() -> PositionRiskRequest {
    PositionRiskRequest::new()
}

pub fn adl_quantile() -> AdlQuantileRequest {
    AdlQuantileRequest::new()
}
