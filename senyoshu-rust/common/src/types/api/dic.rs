use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};

use crate::types::api::account::Token;
use crate::types::api::API;
use crate::types::api::api::WordHistoryEntry;
use crate::types::state::State;
use crate::types::word::wid::WordIdentity;
use crate::types::word::word_entry::{WordDefine, WordEntry};

pub const CREATE_WORD_API: API<(Token, WordDefine), Option<WordIdentity>> = API::new("create_word");
pub const DELETE_WORD_API: API<(Token, /* wid */ i64), bool> = API::new("delete_word");
pub const SYNC_DIC_API: API<
    Option<DateTime<FixedOffset>>,
    HashMap<WordIdentity, Option<WordDefine>>,
> = API::new("sync_dic");

pub const GET_CHANGE_REQUEST_API: API<Token, Vec<WordHistoryEntry>> =
    API::new("get_change_request");

pub const GET_WORD_BY_PID_API: API</* pid */ i64, Option<WordEntry>> = API::new("get_word_by_pid");

pub const GET_WORD_HISTORY_API: API</* wid */ i64, Vec<WordHistoryEntry>> =
    API::new("get_word_history");

pub const POST_WORD_API: API<(Token, WordEntry), bool> = API::new("post_word");

pub const SET_ADOPTED_API: API<(Token, /* pid */ i64, State), bool> = API::new("set_adopted");


pub const UPDATE_MANY_API: API<(Token, HashMap<WordIdentity, WordDefine>), bool> = API::new("update_many");