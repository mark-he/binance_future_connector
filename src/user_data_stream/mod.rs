//! Binance SPOT User Data Websocket  Streams
//!
//! A collection of SPOT User Data Websocket streams.
mod user_data;

pub use user_data::UserDataStream;
pub mod close_listen_key;
pub mod new_listen_key;
pub mod renew_listen_key;

use close_listen_key::CloseListenKey;
use new_listen_key::NewListenKey;
use renew_listen_key::RenewListenKey;

pub fn new_listen_key() -> NewListenKey {
    NewListenKey::new()
}

pub fn renew_listen_key(listen_key: &str) -> RenewListenKey {
    RenewListenKey::new(listen_key)
}

pub fn close_listen_key(listen_key: &str) -> CloseListenKey {
    CloseListenKey::new(listen_key)
}

pub fn user_data(listen_key: &str) -> UserDataStream {
    UserDataStream::new(listen_key)
}
