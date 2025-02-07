use std::collections::HashMap;

use axum::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoSimpleExpr};
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::prelude::Expr;
use sea_orm::QueryFilter;
use sea_orm::TransactionTrait;
use tracing::instrument;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::state::State;
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordDefine;

use crate::api::account::get_user_info;
use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::{word_history, words};

pub async fn update_many_api(
    Json((token, update)): Json<(Token, HashMap<WordIdentity, WordDefine>)>,
) -> Json<bool> {
    Json(update_many(token, update).await.is_some())
}


#[instrument]
pub async fn update_many(
    token: Token,
    update: HashMap<WordIdentity, WordDefine>,
) -> Option<()> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let transaction = db.begin().await.ok()?;
    let user_info = get_user_info(token, &transaction).await?;
    if user_info.content_maintainer == false {
        return None;
    }
    for (wid, word_define) in update.into_iter() {
        word_history::ActiveModel {
            author: Set(user_info.uid),
            wid: Set(wid),
            word_define: Set(word_define.to_owned()),
            state: Set(State::Pass),
            ..Default::default()
        }.insert(&transaction)
            .await.ok()?;
        let result = words::Entity::update_many()
            .col_expr(
                words::Column::WordDefine,
                Expr::value(word_define),
            )
            .col_expr(
                words::Column::UpdateDate,
                Expr::current_timestamp().into_simple_expr(),
            )
            .filter(words::Column::Wid.eq(wid))
            .exec(&transaction)
            .await
            .ok()?;
        if result.rows_affected == 0 {
            return None;
        }
    }


    transaction.commit().await.ok()?;
    Some(())
}


