
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
