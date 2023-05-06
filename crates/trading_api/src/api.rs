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
    // fn get_capabilities();

    // fn test_connection();

    // fn limit_order(o: Order);
    // fn market_order(o: Order);

    // fn stop_limit();
    // fn stop_market();

    // fn cancel_all_orders();
    // fn cancel_order(order_id: String);

    // fn bulk_orders();
    // fn cancel_order_batch(order_id: String);
}
