use crate::{
    errors::TradingApiError,
    interfaces::{Instrument, Symbol},
};

pub trait TestnetApi {
    fn set_testnet();
}

pub type Result<T> = core::result::Result<T, TradingApiError>;

pub trait PublicApi {
    fn get_server_time(&self);

    fn get_markets(&self) -> Result<Vec<Instrument>>;
    fn get_market_details(&self, market: String) -> Result<Instrument>;

    fn get_ohlc_data(pair: Symbol, interval: usize);
    fn get_orderbook(pair: Symbol, count: Option<usize>);
}

pub trait PrivateApi {
    fn authenticate(&mut self, api_key: String, secret: String);

    fn get_wallets(&self);
    fn check_access();
}
