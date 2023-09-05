pub struct Credentials {
    name: String,
    api_key: String,
    secret: String,
}

pub struct CredentialsStorage {
    credentials: Vec<Credentials>,
    json_path: String,
}

impl CredentialsStorage {
    fn new() -> Self {
        return CredentialsStorage {
            credentials: vec![],
            json_path: "".to_string(),
        };
    }

    fn new_from_json(json_path: &str) -> Self {
        return CredentialsStorage {
            credentials: vec![],
            json_path: json_path.to_string(),
        };
    }

    fn add(&mut self, name: &str, api_key: &str, secret: &str) {
        self.credentials.push(Credentials {
            name: name.to_string(),
            api_key: api_key.to_string(),
            secret: secret.to_string(),
        });
    }

    fn add_credentials(&mut self, creds: Credentials) {
        self.credentials.push(creds);
    }
}

#[cfg(test)]
mod tests {
    // use crate::
    use super::{Credentials, CredentialsStorage};

    #[test]
    fn add_works() {
        let mut storage = CredentialsStorage::new();

        storage.add("Kraken Futures Trading", "api_key", "api_secret");

        assert_eq!(storage.credentials.len(), 1);

        assert_eq!(storage.credentials[0].api_key, "api_key");
        assert_eq!(storage.credentials[0].secret, "api_secret");
        assert_eq!(storage.credentials[0].name, "Kraken Futures Trading");
    }

    #[test]
    fn add_credentials_works() {
        let mut storage = CredentialsStorage::new();

        let creds = Credentials {
            name: "Kraken Spot Trading".to_string(),
            secret: "api_secret".to_string(),
            api_key: "api_key".to_string(),
        };

        storage.add_credentials(creds);

        assert_eq!(storage.credentials.len(), 1);

        assert_eq!(storage.credentials[0].api_key, "api_key");
        assert_eq!(storage.credentials[0].secret, "api_secret");
        assert_eq!(storage.credentials[0].name, "Kraken Spot Trading");
    }
}
