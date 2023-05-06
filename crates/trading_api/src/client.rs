type StaticString = &'static str;

#[derive(Clone, Debug)]
pub enum ExchangeName {
    Kraken,
    KrakenFutures,
}
#[derive(Clone, Debug)]
pub struct Exchange {
    pub name: ExchangeName,

    pub url: StaticString,
    pub api_url: StaticString,

    pub public_api_url: StaticString,
    pub private_api_url: StaticString,

    pub testnet_url: Option<StaticString>,
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
