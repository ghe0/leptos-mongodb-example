#![cfg(feature = "ssr")]
use crate::model::*;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use once_cell::sync::OnceCell;

static MONGODB_CLIENT: OnceCell<Client> = OnceCell::new();

pub fn get_mongodb_client() -> &'static Client {
    unsafe { MONGODB_CLIENT.get_unchecked() }
}

pub async fn init() {
    let mongodb_uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://127.0.0.1:27017".into());
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("failed to connect");
    MONGODB_CLIENT.set(client).unwrap();
}

fn members_coll() -> Collection<Member> {
    let client = get_mongodb_client();
    client.database("test").collection::<Member>("members")
}

pub async fn get_members() -> Result<Vec<Member>, mongodb::error::Error> {
    let mut cursor = members_coll().find(doc! {}, None).await?;
    let mut members: Vec<Member> = Vec::new();
    while let Some(member) = cursor.try_next().await? {
        members.push(member);
    }
    Ok(members)
}
