#![cfg(feature = "ssr")]

use chrono::prelude::*;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use leptos::log;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const KEY_FROM_ENV: &'static [u8] = b"this_is_my_secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    exp: DateTime<Utc>,
}

pub fn encode(username: String) -> String {
    let exp = Utc::now().checked_add_signed(chrono::Duration::minutes(15)).unwrap();
    let claims = Claims { username, exp };
    // TODO: figure if this can fail and handle the error properly
    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(KEY_FROM_ENV))
        .unwrap()
}

#[derive(Clone, Debug, Error)]
pub enum AuthError {
    #[error("The token is not valid.")]
    InvalidToken,
    #[error("The token is expired.")]
    ExpiredToken,
}

/// returns the username or error if the token is expired
pub fn extract_username(token: String) -> Result<String, AuthError> {
    match jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(KEY_FROM_ENV),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(c) if c.claims.exp.gt(&Utc::now()) => Err(AuthError::ExpiredToken),
        Ok(c) => Ok(c.claims.username),
        Err(err) => {
            log!("Failed to authenticate via token: {err:?}");
            Err(AuthError::InvalidToken)
        }
    }
}
