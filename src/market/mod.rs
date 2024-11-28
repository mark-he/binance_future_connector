//! Market Data

pub mod agg_trades;
pub mod book_ticker;
pub mod depth;
pub mod exchange_info;
pub mod historical_trades;
pub mod klines;
pub mod ping;
pub mod ticker_price;
pub mod ticker_twenty_four_hr;
pub mod time;
pub mod trades;
pub mod contract_klines;
pub mod index_price_klines;
pub mod premium_index_klines;
pub mod premium_index;
pub mod funding_rate;
pub mod funding_info;
pub mod open_interest;

use agg_trades::AggTradesRequest;
use book_ticker::BookTickerRequest;
use contract_klines::{ContractKlinesRequest, ContractType};
use depth::Depth;
use exchange_info::ExchangeInfoRequest;
use funding_info::FundingInfoRequest;
use funding_rate::FundingRateRequest;
use historical_trades::HistoricalTradesRequest;
use index_price_klines::IndexPriceKlinesRequest;
use klines::{KlineInterval, KlinesRequest};
use open_interest::OpenInterestRequest;
use ping::PingRequest;
use premium_index::PremiumIndexRequest;
use premium_index_klines::PremiumIndexKlinesRequest;
use ticker_price::TickerPriceRequest;
use ticker_twenty_four_hr::Ticker24hrRequest;
use time::TimeRequests;
use trades::TradesRequest;

pub fn ping() -> PingRequest {
    PingRequest::new()
}

pub fn time() -> TimeRequests {
    TimeRequests::new()
}

pub fn exchange_info() -> ExchangeInfoRequest {
    ExchangeInfoRequest::new()
}

pub fn depth(symbol: &str) -> Depth {
    Depth::new(symbol)
}

pub fn trades(symbol: &str) -> TradesRequest {
    TradesRequest::new(symbol)
}

pub fn historical_trades(symbol: &str) -> HistoricalTradesRequest {
    HistoricalTradesRequest::new(symbol)
}

pub fn agg_trades(symbol: &str) -> AggTradesRequest {
    AggTradesRequest::new(symbol)
}

pub fn klines(symbol: &str, interval: KlineInterval) -> KlinesRequest {
    KlinesRequest::new(symbol, interval)
}

pub fn contract_klines(pair: &str, contract_type: ContractType, interval: KlineInterval) -> ContractKlinesRequest {
    ContractKlinesRequest::new(pair, contract_type, interval)
}

pub fn index_price_klines(pair: &str, interval: KlineInterval) -> IndexPriceKlinesRequest {
    IndexPriceKlinesRequest::new(pair, interval)
}

pub fn premium_index_klines(pair: &str, interval: KlineInterval) -> PremiumIndexKlinesRequest {
    PremiumIndexKlinesRequest::new(pair, interval)
}

pub fn premium_index(symbol: &str) -> PremiumIndexRequest {
    PremiumIndexRequest::new(symbol)
}

pub fn funding_rate(symbol: &str) -> FundingRateRequest {
    FundingRateRequest::new(symbol)
}

pub fn funding_info() -> FundingInfoRequest {
    FundingInfoRequest::new()
}

pub fn ticker_twenty_four_hr() -> Ticker24hrRequest {
    Ticker24hrRequest::new()
}

pub fn ticker_price() -> TickerPriceRequest {
    TickerPriceRequest::new()
}

pub fn book_ticker() -> BookTickerRequest {
    BookTickerRequest::new()
}

pub fn open_interest(symbol: &str) -> OpenInterestRequest {
    OpenInterestRequest::new(symbol)
}
