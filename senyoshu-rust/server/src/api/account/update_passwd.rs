use axum::Json;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter};
use sea_orm::prelude::Expr;
use tracing::{error, instrument};

use senyoshu_common::util::passwd_hasher::get_passwd_hash;

use crate::database::account;
use crate::database::database::GLOBAL_DATABASE;

pub async fn update_passwd_api(
    Json((username, new_passwd_hash, old_passwd_hash)): Json<(String, String, String)>,
) -> Json<bool> {
    Json(
        update_passwd(
            username.as_str(),
            new_passwd_hash.as_str(),
            old_passwd_hash.as_str(),
        )
            .await
            .is_some(),
    )
}

#[instrument]
pub(crate) async fn update_passwd(
    username: &str,
    new_passwd_hash: &str,
    old_passwd_hash: &str,
) -> Option<()> {
    let db = GLOBAL_DATABASE.get().unwrap();

    let result = account::Entity::update_many()
        .filter(
            Condition::all()
                .add(account::Column::Username.eq(username))
                .add(account::Column::PasswdHash2.eq(get_passwd_hash(old_passwd_hash))),
        )
        .col_expr(
            account::Column::PasswdHash2,
            Expr::value(get_passwd_hash(new_passwd_hash)),
        )
        .exec(db)
        .await
        .ok()?;

    match result.rows_affected {
        0 => None,
        1 => Some(()),
        _ => {
            let rows_affected = result.rows_affected;
            error!(rows_affected);
            Some(())
        }
    }
}
