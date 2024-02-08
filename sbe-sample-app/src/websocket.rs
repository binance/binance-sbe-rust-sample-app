use crate::exchange_info::{ErrorResponse, ExchangeInfo};
use crate::rate_limit::RateLimit;
use serde::Serialize;

#[derive(Serialize)]
enum WebSocketResult {
    Unset,
    Error(ErrorResponse),
    ExchangeInfo(ExchangeInfo),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMetadata {
    status: u16,
    rate_limits: Vec<RateLimit>,
    id: String,
    result: WebSocketResult,
}

impl WebSocketMetadata {
    pub fn new(status: u16, rate_limits: Vec<RateLimit>, id: String) -> Self {
        Self {
            status,
            rate_limits,
            id,
            result: WebSocketResult::Unset,
        }
    }

    pub fn set_error(&mut self, error: ErrorResponse) {
        self.result = WebSocketResult::Error(error);
    }

    pub fn set_exchange_info(&mut self, exchange_info: ExchangeInfo) {
        self.result = WebSocketResult::ExchangeInfo(exchange_info);
    }
}
