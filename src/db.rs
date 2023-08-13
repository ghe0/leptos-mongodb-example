#![cfg(feature = "ssr")]
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Client, Collection};
use once_cell::sync::OnceCell;
use thiserror::Error;

use crate::model::*;

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn get_mongodb_client() -> &'static Client {
    unsafe { MONGODB_CLIENT.get_unchecked() }
}

pub async fn init() {
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".into());
    let client = Client::with_uri_str(mongodb_uri).await.expect("failed to connect");
    MONGODB_CLIENT.set(client).unwrap();
}

fn members_coll() -> Collection<Member> {
    let client = get_mongodb_client();
    client.database("test").collection::<Member>("members")
}

fn posts_coll() -> Collection<Post> {
    let client = get_mongodb_client();
    client.database("test").collection::<Post>("posts")
}

pub async fn get_posts() -> Result<Vec<Post>, mongodb::error::Error> {
    let mut cursor = posts_coll().find(doc! {}, None).await?;
    let mut posts = Vec::new();
    while let Some(post) = cursor.try_next().await? {
        posts.push(post);
    }
    Ok(posts)
}

#[derive(Clone, Debug, Error)]
pub enum AuthError {
    #[error("Bad member credentials.")]
    BadCredentials,
    #[error("Database error")]
    Mongo(#[from] mongodb::error::Error),
}

pub async fn auth_member(username: &str, password: &str) -> Result<(), AuthError> {
    match members_coll()
        .find_one(doc! { "username": username, "password": password }, None)
        .await? {
            Some(_) => Ok(()),
            None => Err(AuthError::BadCredentials),
        }
}

pub async fn add_member(member: Member) -> Result<(), mongodb::error::Error> {
    members_coll().insert_one(member, None).await?;
    Ok(())
}

pub async fn add_post(post: Post) -> Result<(), mongodb::error::Error> {
    posts_coll().insert_one(post, None).await?;
    Ok(())
}

