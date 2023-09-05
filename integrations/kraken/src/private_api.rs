use crate::http_client::KrakenClient;

use trading_api::{
    api::PrivateApi,
    interfaces::{Instrument, Symbol},
};

impl PrivateApi for KrakenClient {
    fn authenticate(&mut self, api_key: String, secret: String) {
        self.set_credentials(api_key, secret)
    }

    fn check_access() {}

    fn get_wallets(&self) {
        // &self.get()
    }
}

#[cfg(test)]
mod tests {
    use trading_api::exchange::ExchangeName;

    use super::*;

    #[test]
    fn authenticate_works() {
        let client = KrakenClient::new();

        let w = client.get_wallets();

        assert_eq!(client.exchange.name, ExchangeName::KrakenFutures);
    }
}
