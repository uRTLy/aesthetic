#[derive(Clone, Debug)]
pub enum ExchangeName {
  Kraken,
  KrakenFutures
}
#[derive(Clone, Debug)]
pub struct Exchange {
  pub name: ExchangeName,

  pub url: &'static str,
  pub api_url: &'static str,

  pub public_api_url: &'static str,
  pub private_api_url: &'static str,

  pub testnet_url: Option<&'static str>,
}

// impl Clone for Exchange {
//   fn clone(&self) -> Exchange {
//     Exchange {
//       name: self.name,
//       url: self.url,
//       api_url: self.api_url,
//       testnet_url: self.testnet_url,
//       public_api_url: self.public_api_url,
//       private_api_url: self.private_api_url,
//     }
//   }
// }