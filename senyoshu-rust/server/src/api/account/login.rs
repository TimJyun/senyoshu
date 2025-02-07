use axum::Json;
use blake2::{Blake2b512, Digest};
use chrono::Local;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use sea_orm::prelude::Expr;
use tracing::{debug, instrument};

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::api::session::{Session, SessionVec};
use senyoshu_common::util::passwd_hasher::get_passwd_hash;

use crate::database::account;
use crate::database::database::GLOBAL_DATABASE;

pub async fn login_api(
    Json((username, password_hash)): Json<(String, String)>,
) -> Json<Option<Token>> {
    Json(login(username, password_hash).await)
}

#[instrument]
pub async fn login(username: String, password_hash: String) -> Option<Token> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let transaction = db.begin().await.ok()?;

    let passwd_hash2 = get_passwd_hash(password_hash.as_str());
    debug!("username:{}", username.as_str());
    debug!("passwd_hash2:{}", passwd_hash2);

    let user_info = account::Entity::find()
        .filter(account::Column::Username.eq(username))
        .one(&transaction)
        .await
        .unwrap()?;

    debug!("passwd_hash2_in_db:{}", user_info.passwd_hash2);
    if user_info.passwd_hash2 != passwd_hash2 {
        return None;
    }

    let mut sessions = user_info.sessions.unwrap_or_default().0;
    //目前限制为4个

    if sessions.len() >= 4 {
        sessions.pop_front();
    }

    let time = Local::now();
    let mut hasher = Blake2b512::new();
    hasher.update(b"senyoshu-user-token-");
    hasher.update(user_info.uid.to_string().as_bytes());
    hasher.update(b"-");
    hasher.update(time.to_string().as_str());
    hasher.update(b"-4cvx58dsf1cxv5df");
    let result = hasher.finalize();

    let token = hex::encode(result);

    sessions.push_back(Session {
        token: token.to_string(),
        login_time: time.into(),
        active: true,
    });

    account::Entity::update_many()
        .filter(account::Column::Uid.eq(user_info.uid))
        .col_expr(
            account::Column::Sessions,
            Expr::value(Some(SessionVec(sessions.to_owned()))),
        )
        .exec(&transaction)
        .await
        .ok()?;

    transaction.commit().await.ok()?;

    Some(Token {
        uid: user_info.uid,
        token,
    })
}
