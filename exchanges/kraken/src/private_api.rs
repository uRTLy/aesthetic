use trading_api::api::PrivateApi;

use crate::client::KrakenClient;

impl PrivateApi for KrakenClient {

  fn authenticate(api_key: &str, secret: &str) {
      
  }
}