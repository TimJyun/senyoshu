use axum::Json;
use itertools::Itertools;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::ActiveModelTrait;
use tracing::instrument;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::api::api::WordHistoryEntry;
use senyoshu_common::types::state::State;

use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::word_history;

pub async fn get_change_request_api(Json(token): Json<Token>) -> Json<Vec<WordHistoryEntry>> {
    Json(get_change_request(token).await.unwrap_or_default())
}

//todo: 鉴权
#[instrument]
async fn get_change_request(token: Token) -> Option<Vec<WordHistoryEntry>> {
    let db = GLOBAL_DATABASE.get().unwrap();

    let rv = word_history::Entity::find()
        .filter(word_history::Column::State.eq(State::Pending))
        .all(db)
        .await
        .ok()?
        .into_iter()
        .map(|it| WordHistoryEntry {
            pid: it.pid,
            post_date: it.post_date,
            author: it.author,
            word: it.word_define.word,
        })
        .collect_vec();

    Some(rv)
}
