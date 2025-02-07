use axum::Json;
use sea_orm::{ActiveModelTrait, IntoSimpleExpr, QueryFilter};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::prelude::Expr;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::word::word_entry::WordDefine;

use crate::api::account::get_user_info;
use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::words;

pub async fn delete_word_api(Json((token, wid)): Json<(Token, i64)>) -> Json<bool> {
    Json(delete_word(token, wid).await.is_some())
}

async fn delete_word(token: Token, wid: i64) -> Option<()> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let user_info = get_user_info(token, db).await?;
    if !user_info.content_maintainer {
        return None;
    }

    if words::Entity::update_many()
        .filter(words::Column::Wid.eq(wid))
        .col_expr(words::Column::WordDefine, Expr::value(None::<WordDefine>))
        .col_expr(
            words::Column::UpdateDate,
            Expr::current_timestamp().into_simple_expr(),
        )
        .exec(db)
        .await
        .ok()?
        .rows_affected
        == 0
    {
        return None;
    }

    Some(())
}
