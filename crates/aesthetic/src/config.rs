  static DEFAULT_JSON_PATHS: [&str; 3] = [
    "./aesthetic.json",
    "~/aesthetic.json",
    "~/aesthetic.credentials.json",
  ];

  pub struct Configuration {
    DEFAULT_JSON_PATH: String
  }

  impl Configuration {

    fn new() -> Self {
      return Configuration {
        DEFAULT_JSON_PATH:  DEFAULT_JSON_PATHS[0].to_string()
      }
    }
    
    fn get_default_json_path(&self) {
      // return self.DEFAULT_JSON_PATH
    }

    fn find_path() {
      for path in DEFAULT_JSON_PATHS.iter() {

      } 
    }
  }
