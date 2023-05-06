use reqwest::header::{HeaderMap, HeaderValue};
use serde::{ser::SerializeStruct, Serialize};
use trading_api::client::{Exchange, ExchangeName};

use crate::signature::signature;

pub static KRAKEN_EXCHANGE: Exchange = Exchange {
    name: ExchangeName::Kraken,
    url: "",

    api_url: "https://api.kraken.com",
    testnet_url: None,

    public_api_url: "/0/public/",
    private_api_url: "/0/private/",
};

pub struct Credentials {
    api_key: String,
    secret: String,
}

pub struct KrakenClient {
    exchange: Exchange,
    client: reqwest::Client,
    credentials: Credentials,
}

fn build_public_url(endpoint: &str) -> String {
    return format!(
        "{}/{}/{}",
        KRAKEN_EXCHANGE.api_url, KRAKEN_EXCHANGE.public_api_url, endpoint
    );
}

fn build_private_url(endpoint: &str) -> String {
    return format!(
        "{}/{}/{}",
        KRAKEN_EXCHANGE.api_url, KRAKEN_EXCHANGE.private_api_url, endpoint
    );
}

impl KrakenClient {
    pub fn new(api_key: &str, secret: &str) -> Self {
        let client = reqwest::Client::new();

        return KrakenClient {
            client,
            exchange: KRAKEN_EXCHANGE.clone(),
            credentials: Credentials {
                api_key: api_key.to_string(),
                secret: secret.to_string(),
            },
        };
    }

    pub async fn query_public(endpoint: &str) {
        let url = build_public_url(endpoint);
        let body = reqwest::get(url).await;
    }

    pub async fn query_private<P: Serialize>(&self, endpoint: &str, p: Option<P>) {
        let url = build_private_url(endpoint);

        let (nonce, signature) = signature(endpoint, p, &self.credentials.secret.as_str());

        let mut headers = HeaderMap::new();

        let api_key = HeaderValue::from_str(&self.credentials.api_key).unwrap();
        let api_sign = HeaderValue::from_str(signature.as_str()).unwrap();

        headers.insert("API-Sign", api_sign);
        headers.insert("API-Key", api_key);

        let req = &self.client.get(url).headers(headers).send().await;
    }

    pub async fn post<P: Serialize, B: Serialize>(&self, endpoint: &str, params: P, body: B) {
        let url = build_private_url(endpoint);

        // &self.client.post(
        //   endpoint
        // ).header("API-Key", value)
    }
}
