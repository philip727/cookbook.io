use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, Token, VerifyWithKey};
use sha2::Sha512;

pub type UIDString = String;
pub type UsernameString = String;

pub fn verify_jwt_token(token_str: &str) -> Result<(UIDString, UsernameString), anyhow::Error> {
    let key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha512> = Hmac::new_from_slice(key.as_bytes()).unwrap();
    let claims: BTreeMap<String, String> = token_str
        .verify_with_key(&key)
        .context("Failed to verify key")?;

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time has gone backwards")
        .as_secs();

    let uid = claims
        .get("uid")
        .ok_or(anyhow::Error::msg("No uid claim in jwt"))?;

    let username = claims
        .get("username")
        .ok_or(anyhow::Error::msg("No username claim in jwt"))?;

    // verify exp and iat
    let exp = claims
        .get("exp")
        .ok_or(anyhow::Error::msg("No exp claim in jwt"))?
        .parse::<u64>()?;

    let _ = claims
        .get("iat")
        .ok_or(anyhow::Error::msg("No iat claim in jwt"))?;

    if current_time > exp {
        return Err(anyhow::Error::msg("JWT is expired"));
    }

    Ok((uid.to_string(), username.to_string()))
}

pub fn get_signed_jwt_token(
    mut claims: BTreeMap<String, String>,
) -> Token<jwt::Header, BTreeMap<String, String>, jwt::token::Signed> {
    let key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key: Hmac<Sha512> = Hmac::new_from_slice(key.as_bytes()).unwrap();

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time has gone backwards")
        .as_secs();
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
