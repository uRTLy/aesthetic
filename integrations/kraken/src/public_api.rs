use crate::{
    http_client::KrakenClient,
    schema::{KrakenInstrument, KrakenInstrumentsResponse},
};
use trading_api::{
    api::{PublicApi, Result},
    errors::TradingApiError,
    interfaces::{Instrument, Symbol},
};

impl PublicApi for KrakenClient {
    fn get_server_time(&self) {
        let url = &self.url("/");
    }

    fn get_markets(&self) -> Result<Vec<Instrument>> {
        let req = &self.get::<KrakenInstrumentsResponse>("/v3/instruments");

        let instruments: Vec<Instrument> = req
            .as_ref()
            .unwrap()
            .instruments
            .iter()
            .map(|i: &KrakenInstrument| Instrument::from(i))
            .collect();

        Ok(instruments)
    }

    fn get_market_details(&self, market: String) -> Result<Instrument> {
        let markets = self.get_markets().unwrap();

        let found = markets.iter().find(|m| m.id == market);

        if let Some(instrument) = found.cloned() {
            Ok(instrument)
        } else {
            let msg = format!("Market {market} not found");
            Err(TradingApiError::NotFound(msg))
        }
    }

    fn get_ohlc_data(market: Symbol, interval: usize) {}

    fn get_orderbook(market: Symbol, count: Option<usize>) {}
}

#[cfg(test)]
mod tests {
    use trading_api::exchange::ExchangeName;

    use super::*;

    #[test]
    fn works() {
        let client = KrakenClient::new();

        assert_eq!(client.exchange.name, ExchangeName::KrakenFutures);
    }

    #[test]
    fn get_markets_works() {
        let client = KrakenClient::new();

        let markets = client.get_markets().unwrap();

        markets.iter().for_each(|m| println!("{:?}", m));

        assert!(markets.len() > 0);
        assert_eq!(client.exchange.name, ExchangeName::KrakenFutures);
    }

    #[test]
    fn get_market_details_works() {
        let client = KrakenClient::new();

        let markets = client.get_markets().unwrap();

        let market = markets.first().unwrap();
        let market_details = client.get_market_details(market.id.clone()).unwrap();

        assert!(markets.len() > 0);
        assert!(market_details.id == market.id);
    }
}
