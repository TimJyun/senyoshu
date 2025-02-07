use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;

use senyoshu_common::types::api::account::{Token, UserInfo};

use crate::database::account;

pub mod get_other_user_info;
pub mod update_user_state;
pub mod login;
pub mod register;
pub mod update_passwd;

pub(crate) async fn get_user_info<C: ConnectionTrait>(token: Token, db: &C) -> Option<UserInfo> {
    let user_info = account::Entity::find_by_id(token.uid)
        .one(db)
        .await
        .ok()??;

    let sessions = user_info.sessions?;

    if sessions.0.iter().any(|s| s.token == token.token) {
        Some(UserInfo {
            uid: user_info.uid,
            register_date: user_info.register_date,
            username: user_info.username,
            e_mail: user_info.e_mail,
            post_permission: user_info.post_permission,
            restrict_user: user_info.restrict_user,
            content_maintainer: user_info.content_maintainer,
            vip: user_info.vip,
            sessions: sessions.0.into(),
        })
    } else {
        None
    }
}
