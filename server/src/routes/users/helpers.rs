use std::ops::Deref;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

pub enum LoginIdentifier {
    Username(String),
    Email(String),
}

impl Deref for LoginIdentifier {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Username(s) => s,
            Self::Email(s) => s,
        }
    }
}
