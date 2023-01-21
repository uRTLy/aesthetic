use crate::interfaces::{Order, Symbol};

pub trait PublicApi {
   fn get_server_time();

   fn get_all_markets();
   fn get_market_details(pair: Symbol, interval: u8, since: usize);

   fn get_ohlc_data(pair: Symbol, interval: usize);
   fn get_orderbook(pair: Symbol, count: Option<usize>);
}

pub trait PrivateApi {
   fn get_capabilities();

   fn authenticate();
   fn test_connection();

   fn limit_order(o: Order);
   fn market_order(o: Order);

   fn stop_limit();
   fn stop_market();

   fn cancel_all_orders();
   fn cancel_order(order_id: String);

   fn bulk_orders();
   fn cancel_order_batch(order_id: String);
}

pub enum ExchangeName {
  Kraken,
  KrakenFutures
}

pub struct Exchange {
  name: ExchangeName,

  url: String,
  testnet_url: String,

  api_url: String,
  public_api_url: String,
  private_api_url: String,
}

pub struct ExchangeClient<A: PublicApi, B: PrivateApi> {
  exchange: Exchange,

  public_api: A,
  private_api: B,
}