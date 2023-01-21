
pub enum ExchangeName {
  Kraken,
  KrakenFutures
}

pub struct Exchange {
  name: ExchangeName,

  url: String,
  testnet_url: String,

  api_url: string,
  public_api_url: string,
  private_api_url: string,

}

pub enum Side {
  BUY,
  SELL
}

pub enum Ticker {}

pub struct Trade {
  side: Side,
  quantity: usize, 
}

pub struct Order {
  side: Side,
  quantity: usize, 
}

