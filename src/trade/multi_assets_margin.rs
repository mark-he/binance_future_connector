use crate::http::{request::Request, Method};

use super::enums::MarginAssetMode;

/// `GET /fapi/v1/multiAssetsMargin`
///
/// Change user's initial leverage of specific symbol market.
pub struct MultiAssetsMarginRequest {
    pub multi_assets_margin: MarginAssetMode,
    pub recv_window: Option<i64>,
}

impl MultiAssetsMarginRequest {
    pub fn new(multi_assets_margin: MarginAssetMode) -> Self {
        Self {
            multi_assets_margin,
            recv_window: None,
        }
    }

    pub fn set_recv_window(mut self, recv_window: i64) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn get_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        params.push(("multiAssetsMargin".to_owned(), self.multi_assets_margin.to_string()));

        if let Some(recv_window) = self.recv_window {
            params.push(("recvWindow".to_owned(), recv_window.to_string()));
        }

        params
    }
}

impl From<MultiAssetsMarginRequest> for Request {
    fn from(request: MultiAssetsMarginRequest) -> Request {
        let params = request.get_params();
        Request {
            path: "/fapi/v1/multiAssetsMargin".to_owned(),
            method: Method::Post,
            params,
            credentials: None,
            sign: true,
        }
    }
}