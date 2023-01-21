use std::any::Any;

// HMAC-SHA512 of (URI path + SHA256(nonce + POST data)) and base64 decoded secret API key


pub fn authenticate(url: String, data: String, secret: String) {

}

pub fn auth_headers() {

}

#[cfg(test)]
mod tests {
    use super::*;

    // const ENDPOINT = '';

    #[test]
    fn valid_authenticate_method() {
        let result = authenticate();
        println!("{}", result);
    }
}
