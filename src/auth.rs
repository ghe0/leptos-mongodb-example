#![cfg(feature = "ssr")]
use actix_session::Session;
use chrono::prelude::*;
use leptos_actix::extract;
use leptos::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Token {
    username: String,
    exp: DateTime<Utc>,
}

pub async fn login(cx: Scope, username: String) -> Result<(), ServerFnError> {
    let exp = Utc::now().checked_add_signed(chrono::Duration::minutes(1)).unwrap();
    let token = Token { username, exp };
    let session = extract(cx, |session: Session| async move { session }).await.unwrap();
    session.insert("token", token)?;
    Ok(())
}

#[derive(Clone, Debug, Error)]
pub enum AuthError {
    #[error("The token is not valid.")]
    InvalidToken,
    #[error("The token is expired.")]
    ExpiredToken,
}

pub async fn verify(cx: Scope) -> Result<(), AuthError> {
    let session = extract(cx, |session: Session| async move { session }).await.unwrap();
    if let Ok(Some(token)) = session.get::<Token>("token") {
        if token.exp.lt(&Utc::now()) {
            Err(AuthError::ExpiredToken)
        } else {
            Ok(())
        }
    } else {
        Err(AuthError::InvalidToken)
    }
}
