use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue},
};

use serde::de::DeserializeOwned;

use trading_api::{
    api::Result,
    errors::TradingApiError,
    exchange::{Exchange, ExchangeName},
};

pub struct KrakenClient {
    client: Client,

    api_key: Option<String>,
    secret: Option<String>,

    pub exchange: Exchange,
}

impl KrakenClient {
    pub fn new() -> KrakenClient {
        KrakenClient {
            exchange: Exchange {
                name: ExchangeName::KrakenFutures,
                url: "http://www.kraken.com".to_string(),
                api_url: "https://futures.kraken.com/derivatives/api".to_string(),
                testnet_url: Some("https://demo-futures.kraken.com".to_string()),
            },
            client: Client::new(),

            api_key: None,
            secret: None,
        }
    }

    pub fn set_credentials(&mut self, api_key: String, secret: String) {
        self.api_key = Some(api_key);
        self.secret = Some(secret);
    }

    fn auth_headers(&self, post_data: String) -> HeaderMap {
        let mut headers = HeaderMap::new();

        let api_key_header = HeaderValue::from_str(&self.api_key.clone().unwrap()).unwrap();
        let secret_header = HeaderValue::from_str(&self.secret.as_ref().unwrap()).unwrap();

        headers.append("API-KEY", api_key_header);
        headers.append("Secret", secret_header);

        return headers;
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}/{}", &self.exchange.api_url, path)
    }

    pub fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.url(path);
        let req = self.client.get(url);
        let data: T = req.send()?.json()?;
        Ok(data)
    }

    pub fn query_priv<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        if self.secret.is_none() || self.api_key.is_none() {
            let msg = format!("Missing api keys");
            Err::<T, TradingApiError>(TradingApiError::Authentication(msg));
        }

        let url = self.url(path);

        let headers = self.auth_headers("lol".to_string());
        // let auth =

        // let params = [("key", self.api_key)];

        let req = self.client.get(url).headers(headers);
        let data = req.send()?.json()?;

        Ok(data)
    }

    // pub fn post<T: DeserializeOwned>(&self, path: &str) -> core::result::Result<T, Error> {
    //     let url = self.url(path);

    //     // let data: T = self.client;
    //     // let params =
    //     // let data: T = res.json()?;

    //     Ok()
    // }
}

#[cfg(test)]
mod tests {
    use super::KrakenClient;

    #[test]
    fn get_works() {
        let client = KrakenClient::new();

        // let res = client.post("");

        println!("here");
    }

    #[test]
    fn set_credentials_works() {
        let mut client = KrakenClient::new();

        assert_eq!(client.api_key, None);
        assert_eq!(client.secret, None);

        let api_key = "some_key".to_string();
        let secret = "secret".to_string();

        client.set_credentials(api_key.to_owned(), secret.to_owned());

        assert_eq!(client.api_key.unwrap(), api_key);
        assert_eq!(client.secret.unwrap(), secret);

        println!("here");
    }

    #[test]
    fn query_priv_works() {
        let mut client = KrakenClient::new();
        //   {
        //     "name": "Kraken Futures Trading",
        //     "api_key": "PtfgcwzilQ5e+j01hiFOVKIQ75+0Sdwuvu4/WZsSL7UV+cPiWJa2MLtx",
        //     "secret": "DyhD1WBJtFon8207eGVeVSfeGBNNBjXyFFZgfiX9+hzGbJUuTer8jXDjHgHId19vmuiuXcmenpUklgP6nJbkeihW"
        //   },

        let api_key = "PtfgcwzilQ5e+j01hiFOVKIQ75+0Sdwuvu4/WZsSL7UV+cPiWJa2MLtx";
        let secret = "DyhD1WBJtFon8207eGVeVSfeGBNNBjXyFFZgfiX9+hzGbJUuTer8jXDjHgHId19vmuiuXcmenpUklgP6nJbkeihW";

        client.set_credentials(api_key.to_owned(), secret.to_owned());

        let post_data = "Soome data";
        let headers = client.auth_headers(post_data.to_string());

        // client.
        // let res = client.post("");

        println!("here");
    }
}
