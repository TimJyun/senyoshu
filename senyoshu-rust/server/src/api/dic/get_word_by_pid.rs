use axum::Json;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use tracing::instrument;

use senyoshu_common::types::word::word_entry::WordEntry;

use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::word_history;

pub async fn get_word_by_pid_api(Json(pid): Json<i64>) -> Json<Option<WordEntry>> {
    Json(get_word_by_pid(pid).await)
}

#[instrument]
async fn get_word_by_pid(pid: i64) -> Option<WordEntry> {
    let db = GLOBAL_DATABASE.get().unwrap();

    let rv = word_history::Entity::find_by_id(pid)
        .one(db)
        .await
        .ok()?
        .map(|it| WordEntry {
            id: it.wid,
            word_define: it.word_define,
        });

    rv
}
