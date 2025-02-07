use axum::Json;
use sea_orm::EntityTrait;

use senyoshu_common::types::api::account::{Token, UserState};
use senyoshu_common::types::api::account::UserInfo;

use crate::database::account;
use crate::database::database::GLOBAL_DATABASE;

pub async fn update_user_state_api(Json(token): Json<Token>) -> Json<UserState> {
    let db = GLOBAL_DATABASE.get().unwrap();

    if let Ok(user_info) = account::Entity::find_by_id(token.uid)
        .one(db)
        .await {
        if let Some(user_info) = user_info {
            if let Some(sessions) = user_info.sessions {
                let authed = sessions.0.iter().any(|s| s.token == token.token);
                if authed {
                    return Json(UserState::Valid(UserInfo {
                        uid: user_info.uid,
                        register_date: user_info.register_date,
                        username: user_info.username,
                        e_mail: user_info.e_mail,
                        post_permission: user_info.post_permission,
                        restrict_user: user_info.restrict_user,
                        content_maintainer: user_info.content_maintainer,
                        vip: user_info.vip,
                        sessions: sessions.0.into(),
                    }));
                }
            }
        }
        Json(UserState::TokenRevoked)
    } else {
        Json(UserState::Err)
    }
}

