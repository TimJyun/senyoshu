use axum::Json;
use itertools::Itertools;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use sea_orm::ActiveModelTrait;
use tracing::instrument;

use senyoshu_common::types::api::api::WordHistoryEntry;

use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::word_history;

pub async fn get_word_history_api(Json(wid): Json<i64>) -> Json<Vec<WordHistoryEntry>> {
    Json(get_word_history(wid).await.unwrap_or_default())
}

#[instrument]
async fn get_word_history(wid: i64) -> Option<Vec<WordHistoryEntry>> {
    let db = GLOBAL_DATABASE.get().unwrap();

    let rv = word_history::Entity::find()
        .filter(word_history::Column::Wid.eq(wid))
        .order_by_desc(word_history::Column::Pid)
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
