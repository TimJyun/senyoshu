use axum::Json;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use senyoshu_common::types::api::account::OtherUserInfo;

use crate::database::account;
use crate::database::database::GLOBAL_DATABASE;

pub async fn get_other_user_info_api(
    Json((uid, username)): Json<(Option<i64>, Option<String>)>,
) -> Json<Option<OtherUserInfo>> {
    if uid.is_some() || username.is_some() {
        Json(get_other_user_info(uid, username).await)
    } else {
        Json(None)
    }
}

pub async fn get_other_user_info(
    uid: Option<i64>,
    username: Option<String>,
) -> Option<OtherUserInfo> {
    let db = GLOBAL_DATABASE.get().unwrap();
    let mut select = account::Entity::find();
    if let Some(uid) = uid {
        select = select.filter(account::Column::Uid.eq(uid));
    }
    if let Some(username) = username {
        select = select.filter(account::Column::Username.eq(username));
    }
    let user_info = select.one(db).await.ok()??;

    Some(OtherUserInfo {
        uid: user_info.uid,
        register_date: user_info.register_date,
        username: user_info.username,
        post_permission: user_info.post_permission,
        restrict_user: user_info.restrict_user,
        content_maintainer: user_info.content_maintainer,
        vip: user_info.vip,
    })
}
