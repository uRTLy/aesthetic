use serde_urlencoded;
use serde_qs::{Serializer};
use serde::{Serialize, Deserialize};

use sha2::Sha512_256;
use base64::{Engine as _, engine::general_purpose};

use std::time::{SystemTime, UNIX_EPOCH};

fn nonce() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

// HMAC-SHA512 of (URI path + SHA256(nonce + POST data)) and base64 decoded secret API key

pub fn signature<D: Serialize>(uri: &str, data: D, secret: &str) -> (u128, String) {
    let n = nonce();
    let postdata = serde_urlencoded::to_string(data).unwrap();

    let combined = (n.to_string() + &postdata).as_bytes();

    let mut message = uri.as_bytes();



    let encoded = general_purpose::URL_SAFE_NO_PAD.encode(secret);
    let encoded2 = general_purpose::URL_SAFE_NO_PAD.encode(n.to_string());

    return (n, "".to_string());
}


#[cfg(test)]
mod tests {
    use super::{signature, nonce};
    use std::{thread, time::Duration};
    use crate::client::{KRAKEN_EXCHANGE};

    #[test]
    fn nonce_works() {
        assert_eq!(nonce(), nonce());

        let first = nonce();
        thread::sleep(Duration::from_millis(7));
        let second = nonce();

        assert_ne!(first, second);
        assert_eq!(second - first >= 7, true);
    }

    #[test]
    fn signature_works() {
        let result = signature("/endpoint", "", "secret");

        println!("{}/{}", result.0, result.1);
    }
    #[test]
    fn simple_get() {
    //    let body = reqwest::get()
    }

}
