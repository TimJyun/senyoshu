use std::collections::VecDeque;

use chrono::FixedOffset;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub login_time: chrono::DateTime<FixedOffset>,
    pub active: bool,
}

#[derive(FromJsonQueryResult, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SessionVec(pub VecDeque<Session>);
