use crate::interfaces::{Order, Symbol};

pub trait PublicApi {
    fn set_test_net() {}

    fn get_server_time();

    fn get_all_markets();
    fn get_market_details(pair: Symbol, interval: u8, since: usize);

    fn get_ohlc_data(pair: Symbol, interval: usize);
    fn get_orderbook(pair: Symbol, count: Option<usize>);
}

pub trait PrivateApi {
    fn authenticate(api_key: &str, secret: &str);
}
