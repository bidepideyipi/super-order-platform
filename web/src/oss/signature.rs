use base64::Engine;
use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

pub fn generate_signature(access_key_secret: &str, string_to_sign: &str) -> String {
    let mut mac = HmacSha1::new_from_slice(access_key_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize();
    base64::engine::general_purpose::STANDARD.encode(result.into_bytes())
}

pub fn url_encode(text: &str) -> String {
    urlencoding::encode(text).to_string()
}
