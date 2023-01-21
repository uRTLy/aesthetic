enum Provider {
  KrakenFutures,
  KrakenSpot
}

enum Access {
  Trade,
  ReadOnly
}

enum Capabilities {
  BulkOrders,
}

pub struct Credentials {
  name: String,
  api_key: String, 
  secret: String, 

  access: Access, 
  provider: Provider,
}

static DEFAULT_JSON_PATH: &str = "~/aesthetic.json";

pub struct CredentialsStorage {
  credentials: Vec<Credentials>,
  json_path: String
}

impl CredentialsStorage {
  fn new() -> Self {
    return CredentialsStorage { credentials: vec![], json_path: DEFAULT_JSON_PATH.to_string() };
  } 

  fn new_from_json(json_path: &str) -> Self {
    return CredentialsStorage { credentials: vec![], json_path: json_path.to_string() };
  } 

  fn add(&mut self, name: &str, api_key: &str, secret: &str, provider: Provider, access: Access) {
    self.credentials.push(
      Credentials{
        access,
        provider,
        name: name.to_string(),
        api_key: api_key.to_string(),
        secret: secret.to_string(),
      }
    );
  }
  fn add_credentials(&mut self, creds: Credentials) {
    self.credentials.push(creds);
  }

  // fn edit() -> Self {

  // }
  
  // fn remove() -> Self {

  // }

  // fn load_from_JSON(&self) {

  // }

  // fn save_to_JSON(path: &str) -> Self {

  // }
}

#[cfg(test)]
mod tests {
  // use crate::
    use super::{Access, Credentials, CredentialsStorage, Provider, DEFAULT_JSON_PATH};

    #[test]
    fn add_works() {
      let mut storage = CredentialsStorage::new();

      storage.add("Kraken Futures Trading", "api_key", "api_secret", Provider::KrakenFutures, Access::Trade);

      assert_eq!(storage.credentials.len(), 1);
      assert_eq!(storage.json_path, DEFAULT_JSON_PATH);

      assert_eq!(storage.credentials[0].api_key, "api_key");
      assert_eq!(storage.credentials[0].secret, "api_secret");
      assert_eq!(storage.credentials[0].name, "Kraken Futures Trading");
    }

    fn add_credentials_works() {
      let mut storage = CredentialsStorage::new();

      let creds = Credentials {
        access: Access::Trade,
        name: "Kraken Spot Trading".to_string(),
        secret: "api_secret".to_string(),
        api_key: "api_key".to_string(),
        provider: Provider::KrakenSpot
      };

      storage.add_credentials(creds);

      assert_eq!(storage.credentials.len(), 1);
      assert_eq!(storage.json_path, DEFAULT_JSON_PATH);

      assert_eq!(storage.credentials[0].api_key, "api_key");
      assert_eq!(storage.credentials[0].secret, "api_secret");
      assert_eq!(storage.credentials[0].name, "Kraken Spot Trading");
    }
}