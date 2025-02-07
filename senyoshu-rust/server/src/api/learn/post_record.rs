use std::default::Default;

use axum::Json;
use sea_orm::{
    ColumnTrait, EntityTrait, IntoSimpleExpr, ModelTrait, QueryFilter, TransactionTrait,
};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::Expr;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::learn::LearnHistoryMap;
use senyoshu_common::util::time::UtcTimeStamp;

use crate::api::account::get_user_info;
use crate::database::database::GLOBAL_DATABASE;
use crate::database::learn;

pub async fn post_learn_record_api(
    Json((token, learn_record_increment_vec)): Json<(Token, LearnHistoryMap)>,
) -> Json<bool> {
    Json(
        post_learn_record(token, learn_record_increment_vec)
            .await
            .is_some(),
    )
}

pub(crate) async fn post_learn_record(
    token: Token,
    learn_operate_vec: LearnHistoryMap,
) -> Option<()> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let transaction = db.begin().await.ok()?;
    let user_info = get_user_info(token, &transaction).await?;

    for (knowledge, mut operates) in learn_operate_vec.into_iter() {
        let result = learn::Entity::find_by_id((
            user_info.uid,
            knowledge.knowledge_type,
            knowledge.key.to_owned(),
        ))
            .one(&transaction)
            .await
            .ok()?;

        let is_some = result.is_some();
        let (mut history, freeze_time) = result
            .map(|it| (it.history, it.freeze_time))
            .unwrap_or_default();
        //让后来的记录覆盖先前的
        operates.history.append(&mut history);
        let mut history = operates.history;
        history.sort();
        history.dedup();

        if is_some {
            learn::Entity::update_many()
                .set(learn::ActiveModel {
                    freeze_time: if let Some(freeze_time) = freeze_time {
                        let last_op = history.last().map(|op| op.operate_time).unwrap_or_default();
                        if UtcTimeStamp::from(freeze_time.to_utc()) < last_op {
                            Set(None)
                        } else {
                            Set(operates.freeze_time)
                        }
                    } else {
                        Set(operates.freeze_time)
                    },
                    history: Set(history),
                    ..Default::default()
                })
                .col_expr(
                    learn::Column::UpdateTime,
                    Expr::current_timestamp().into_simple_expr(),
                )
                .filter(learn::Column::Uid.eq(user_info.uid))
                .filter(learn::Column::KnowledgeType.eq(knowledge.knowledge_type))
                .filter(learn::Column::KnowledgeKey.eq(knowledge.key))
                .exec(&transaction)
                .await
                .ok()?;
        } else {
            learn::ActiveModel {
                uid: Set(user_info.uid),
                knowledge_type: Set(knowledge.knowledge_type),
                knowledge_key: Set(knowledge.key),
                history: Set(history),
                ..Default::default()
            }
                .insert(&transaction)
                .await
                .ok()?;
        };
    }

    transaction.commit().await.ok()?;
    Some(())
}
