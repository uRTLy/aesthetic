

pub struct ExchangeClient {
  exchange: exchange,
  public_api: PublicApi
  private_api: PrivateApi,
}

trait PublicApi {
  pub fn get_all_markets();
  pub fn get_market_details();
}

trait PrivateApi {
  pub fn get_capabilities();

  pub fn authenticate();
  pub fn test_connection();

  pub fn limit_order(o: Order);
  pub fn market_order(o: Order);

  pub fn stop_limit();
  pub fn stop_market();


  pub fn bulk_orders();
}