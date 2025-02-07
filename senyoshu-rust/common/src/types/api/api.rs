use std::fmt::{Display, Formatter};
use std::str::FromStr;

use chrono::FixedOffset;
use serde::{Deserialize, Serialize};
use crate::types::api::account::{Token, UserState};
use crate::types::api::API;

use crate::types::word::word::Word;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WordHistoryEntry {
    pub pid: i64,
    pub post_date: chrono::DateTime<FixedOffset>,
    pub author: i64,
    pub word: Word,
    // pub adopted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PostList {
    pub pid: i64,
    pub post_date: chrono::DateTime<FixedOffset>,
    pub author: i64,
    pub word: Word,
    pub reviewer: Option<i64>,
    pub review_date: Option<chrono::DateTime<FixedOffset>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WordQuery {
    pub txt: String,
    pub ruby: String,
}

impl Display for WordQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_urlencoded::to_string(self).unwrap().as_str())
    }
}

impl FromStr for WordQuery {
    type Err = serde::de::value::Error;

    fn from_str(query: &str) -> Result<Self, Self::Err> {
        serde_urlencoded::from_str(query)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SurfServer {
    pub name:String,
    pub server: String,
    pub server_port: u16,
    pub password: String,
    pub method: String,
}

pub const GET_SURF_SERVERS_API: API<(), Vec<SurfServer>> = API::new("get_surf_servers");