use axum::Json;
use sea_orm::{ActiveModelTrait, TransactionTrait};
use sea_orm::ActiveValue::Set;
use tracing::instrument;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::state::State;
use senyoshu_common::types::word::word_entry::WordEntry;

use crate::api::account::get_user_info;
use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::word_history;

pub async fn post_word_api(
    Json((token, update_word_entry)): Json<(Token, WordEntry)>,
) -> Json<bool> {
    Json(post_word(token, update_word_entry).await.is_some())
}

#[instrument]
async fn post_word(token: Token, update_word_entry: WordEntry) -> Option<()> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let transaction = db.begin().await.ok()?;

    let user_info = get_user_info(token, &transaction).await?;
    if (!user_info.post_permission) && (!user_info.content_maintainer) {
        return None;
    }

    word_history::ActiveModel {
        author: Set(user_info.uid),
        wid: Set(update_word_entry.id),
        word_define: Set(update_word_entry.word_define),
        state: Set(State::Pending),
        ..Default::default()
    }
        .insert(&transaction)
        .await
        .map_err(|it| {
            println!("{}", it.to_string());
            it
        })
        .ok()?;

    transaction.commit().await.ok()?;
    Some(())
}
