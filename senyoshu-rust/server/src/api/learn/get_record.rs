use std::collections::HashMap;

use axum::Json;
use chrono::{DateTime, FixedOffset};
use itertools::Itertools;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::ActiveModelTrait;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::learn::knowledge::Knowledge;
use senyoshu_common::types::learn::learn_knowledge_history::LearnKnowledgeHistory;
use senyoshu_common::types::learn::LearnHistoryMap;

use crate::api::account::get_user_info;
use crate::database::database::GLOBAL_DATABASE;
use crate::database::learn;

pub async fn get_record_api(
    Json((token, from)): Json<(Token, Option<DateTime<FixedOffset>>)>,
) -> Json<Option<Vec<(Knowledge, LearnKnowledgeHistory)>>> {
    //todo:response error
    let rv = get_record(token, from)
        .await
        .map(|ops| ops.into_iter().collect_vec());
    Json(rv)
}

pub(crate) async fn get_record(
    token: Token,
    from: Option<DateTime<FixedOffset>>,
) -> Option<LearnHistoryMap> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let user_info = get_user_info(token, db).await?;

    const DATE_MILLIS: f64 = (24 * 60 * 60) as f64;
    let mut select = learn::Entity::find().filter(learn::Column::Uid.eq(user_info.uid));
    if let Some(from) = from {
        select = select.filter(learn::Column::UpdateTime.gte(from));
    }

    let result = select.all(db).await.ok()?.into_iter().map(|it| {
        (
            Knowledge {
                knowledge_type: it.knowledge_type,
                key: it.knowledge_key,
            },
            LearnKnowledgeHistory {
                history: it.history,
                freeze_time: it.freeze_time,
            },
        )
    });

    let mut rv = HashMap::with_capacity(result.len());
    for (k, v) in result {
        rv.insert(k, v);
    }

    Some(LearnHistoryMap::new(rv))
}
