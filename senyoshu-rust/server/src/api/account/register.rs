use axum::Json;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use tracing::instrument;

use senyoshu_common::util::passwd_hasher::{get_passwd_hash, is_legal_username};

use crate::database::account;
use crate::database::database::GLOBAL_DATABASE;

pub async fn register_api(Json((username, passwd_hash)): Json<(String, String)>) -> Json<bool> {
    Json(register(username, passwd_hash.as_str()).await.is_some())
}

#[instrument]
pub(crate) async fn register(username: String, passwd_hash: &str) -> Option<()> {
    let db = GLOBAL_DATABASE.get().unwrap();

    if is_legal_username(&username) == false {
        return None;
    }

    let user = account::ActiveModel {
        username: Set(username.to_string()),
        passwd_hash2: Set(get_passwd_hash(passwd_hash)),
        ..Default::default()
    };
    user.insert(db).await.ok()?;

    Some(())
}
