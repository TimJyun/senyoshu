use axum::Json;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoSimpleExpr, QueryFilter, Set,
};
use sea_orm::prelude::Expr;
use sea_orm::TransactionTrait;
use tracing::instrument;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::state::State;

use crate::api::account::get_user_info;
use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::word_history;
use crate::database::dic::words;

pub async fn set_adopted_api(Json((token, pid, state)): Json<(Token, i64, State)>) -> Json<bool> {
    if let State::Pending = state {
        return Json(false);
    }

    Json(set_adopted(token, pid, state).await.is_some())
}

#[instrument]
async fn set_adopted(token: Token, pid: i64, state: State) -> Option<()> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let transaction = db.begin().await.ok()?;

    let user_info = get_user_info(token, &transaction).await?;
    if state == State::Pending {
        return None;
    }

    let word_history_row = word_history::Entity::find_by_id(pid)
        .filter(word_history::Column::State.eq(State::Pending))
        .one(&transaction)
        .await
        .ok()??;

    let update_state = || async {
        word_history::Entity::update_many()
            .filter(word_history::Column::Pid.eq(word_history_row.pid))
            .col_expr(word_history::Column::State, Expr::value(state))
            .col_expr(
                word_history::Column::UpdateDate,
                Expr::current_timestamp().into_simple_expr(),
            )
            .exec(&transaction)
            .await
    };
    if word_history_row.author == user_info.uid && state == State::Withdraw {
        update_state().await.ok()?;
    } else if user_info.content_maintainer && state != State::Withdraw {
        update_state().await.ok()?;

        if let State::Pass = state {
            let result = words::Entity::update_many()
                .col_expr(
                    words::Column::WordDefine,
                    Expr::value(word_history_row.word_define.to_owned()),
                )
                .col_expr(
                    words::Column::UpdateDate,
                    Expr::current_timestamp().into_simple_expr(),
                )
                .filter(words::Column::Wid.eq(word_history_row.wid))
                .exec(&transaction)
                .await
                .ok()?;

            //不会出现更新失败
            match result.rows_affected {
                0 => {
                    words::ActiveModel {
                        wid: Set(word_history_row.wid),
                        update_date: ActiveValue::NotSet,
                        word_define: Set(Some(word_history_row.word_define)),
                    }
                        .insert(&transaction)
                        .await
                        .ok()?;
                }
                1 => {}
                _ => {
                    return None;
                }
            }
        }
    } else {
        return None;
    }

    transaction.commit().await.ok()?;
    Some(())
}
