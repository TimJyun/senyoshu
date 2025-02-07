use std::collections::HashMap;

use axum::Json;
use chrono::{DateTime, FixedOffset};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::ActiveModelTrait;
use tracing::instrument;

use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordDefine;

use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::words;

#[instrument]
pub async fn sync_dic_api(
    Json(from): Json<Option<DateTime<FixedOffset>>>,
) -> Json<HashMap<WordIdentity, Option<WordDefine>>> {
    //todo:response error
    Json(sync_dic(from).await.unwrap())
}

async fn sync_dic(
    from: Option<DateTime<FixedOffset>>,
) -> Option<HashMap<WordIdentity, Option<WordDefine>>> {
    let db = GLOBAL_DATABASE.get().unwrap();

    let mut selected = words::Entity::find();
    if let Some(from) = from {
        selected = selected.filter(words::Column::UpdateDate.gte(from));
    } else {
        selected = selected.filter(words::Column::WordDefine.is_not_null());
    }

    let rows = selected.all(db).await.unwrap();

    let mut rv = HashMap::with_capacity(rows.len());
    for row in rows.into_iter() {
        rv.insert(row.wid, row.word_define);
    }

    Some(rv)
}
