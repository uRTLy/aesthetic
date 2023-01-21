mod config {
  static DEFAULT_JSON_PATHS: vec![&str] = [
    "~/aesthetic.json",
    "~/aesthetic.credentials.json",
    "./aesthetic.json"
  ];

  pub struct Configuration {
    DEFAULT_JSON_PATH: String
  }

  impl Configuration {
    fn new() -> Self {
      return Configuration {
        DEFAULT_JSON_PATH
      }
    }
    
    fn get_default_json_path(&self) {
      return self.DEFAULT_JSON_PATH
    }

  }

  pub fn get_default_json_path(p: &str) {
    return DEFAULT_JSON_PATH;
  }
}

