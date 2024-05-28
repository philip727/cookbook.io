use std::{collections::BTreeMap, time::{SystemTime, UNIX_EPOCH}};

use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, Token};
use sha2::Sha512;

pub fn get_signed_jwt_token(
    mut claims: BTreeMap<String, String>,
) -> Token<jwt::Header, BTreeMap<String, String>, jwt::token::Signed> {
    let key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha512> = Hmac::new_from_slice(key.as_bytes()).unwrap();

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time has gone backwards").as_secs();
    // Adds 30 days
    let exp = current_time + (60 * 60 * 24 * 14);
    // Adds the expiry date and time created
    claims.insert("iat".to_string(), current_time.to_string());
    claims.insert("exp".to_string(), exp.to_string());
    let header = jwt::Header {
        algorithm: jwt::AlgorithmType::Hs512,
        ..Default::default()
    };

    let token = Token::new(header, claims).sign_with_key(&key).unwrap();
    token
}
