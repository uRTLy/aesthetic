use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum TradingApiError {
    Access(String),
    Authentication(String),
    Request(String),
    NotImplemented(String),
    NotFound(String),
}

impl From<reqwest::Error> for TradingApiError {
    fn from(_: reqwest::Error) -> Self {
        todo!()
    }
}
