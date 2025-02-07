use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::api::account::Token;
use crate::types::api::API;
use crate::types::learn::knowledge::Knowledge;
use crate::types::learn::learn_knowledge_history::LearnKnowledgeHistory;
use crate::types::learn::LearnHistoryMap;

pub const POST_LEARN_RECORD_API: API<(Token, LearnHistoryMap), bool> =
    API::new("post_learn_record");
pub const GET_RECORD_API: API<
    (Token, Option<DateTime<FixedOffset>>),
    Option<Vec<(Knowledge, LearnKnowledgeHistory)>>,
> = API::new("get_record");

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostLearnRecordApi {
    token: Token,
    learn_record_increment_vec: LearnHistoryMap,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetRecordApi {
    token: Token,
    from: Option<DateTime<FixedOffset>>,
}
