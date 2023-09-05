use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ExchangeName {
    Kraken,
    KrakenFutures,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exchange {
    pub name: ExchangeName,
    pub url: String,
    pub api_url: String,
    pub testnet_url: Option<String>,
}

impl Exchange {
    fn new(name: ExchangeName, url: &str, api_url: &str, testnet_url: Option<&str>) -> Exchange {
        Exchange {
            name,
            url: url.to_string(),
            api_url: api_url.to_string(),
            testnet_url: Some(testnet_url.unwrap().to_string()),
        }
    }
}
