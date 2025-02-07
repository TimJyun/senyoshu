use axum::Json;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use sea_orm::TransactionTrait;
use tracing::instrument;

use senyoshu_common::types::api::account::Token;
use senyoshu_common::types::state::State;
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordDefine;

use crate::api::account::get_user_info;
use crate::database::database::GLOBAL_DATABASE;
use crate::database::dic::word_history;
use crate::database::dic::words;

pub async fn create_word_api(
    Json((token, new_word_define)): Json<(Token, WordDefine)>,
) -> Json<Option<WordIdentity>> {
    Json(create_word(token, new_word_define).await)
}

#[instrument]
async fn create_word(token: Token, new_word_define: WordDefine) -> Option<WordIdentity> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let transaction = db.begin().await.ok()?;

    let user_info = get_user_info(token, &transaction).await?;
    if (!user_info.post_permission) && (!user_info.content_maintainer) {
        return None;
    }

    let word = words::ActiveModel {
        word_define: Set(Option::from(new_word_define.to_owned())),
        ..Default::default()
    }
        .insert(&transaction)
        .await
        .ok()?;

    word_history::ActiveModel {
        author: Set(user_info.uid),
        wid: Set(word.wid),
        word_define: Set(new_word_define),
        state: Set(State::Pass),
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
    Some(word.wid)
}
