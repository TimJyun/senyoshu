use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::api::API;
use crate::types::api::session::Session;

pub const GET_OTHER_USER_INFO_API: API<
    (
        /* uid */ Option<i64>,
        /* username */ Option<String>,
    ),
    Option<OtherUserInfo>,
> = API::new("get_other_user_info");
pub const UPDATE_USER_STATE_API: API<Token, UserState> = API::new("get_user_info");
pub const LOGIN_API: API<
    (
        /* username */ String,
        /* new_passwd_hash */ String,
    ),
    Option<Token>,
> = API::new("login");
pub const REGISTER_API: API<
    (
        /* username */ String,
        /* new_passwd_hash */ String,
    ),
    bool,
> = API::new("register");
pub const UPDATE_PASSWD_API: API<
    (
        /* username */ String,
        /* new_passwd_hash */ String,
        /* old_passwd_hash */ String,
    ),
    bool,
> = API::new("update_passwd");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum UserState {
    TokenRevoked,
    Valid(UserInfo),
    Err,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct UserInfo {
    pub uid: i64,
    pub register_date: DateTime<FixedOffset>,
    pub username: String,
    pub e_mail: Option<String>,
    pub post_permission: bool,
    pub restrict_user: bool,
    pub content_maintainer: bool,
    pub vip: Option<DateTime<FixedOffset>>,
    pub sessions: Vec<Session>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub uid: i64,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct OtherUserInfo {
    pub uid: i64,
    pub register_date: DateTime<FixedOffset>,
    pub username: String,
    pub post_permission: bool,
    pub restrict_user: bool,
    pub content_maintainer: bool,
    pub vip: Option<DateTime<FixedOffset>>,
}
