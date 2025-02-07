use std::cmp::Ordering;

use chrono::FixedOffset;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

use crate::util::time::UtcTimeStamp;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LearnKnowledgeHistory {
    pub history: Vec<OperateRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freeze_time: Option<chrono::DateTime<FixedOffset>>,
}

#[derive(FromJsonQueryResult, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OperateRecord {
    pub operate_type: OperateType,
    pub operate_time: UtcTimeStamp,
}

impl PartialOrd<Self> for OperateRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.operate_time.partial_cmp(&other.operate_time)
    }
}

impl Ord for OperateRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.operate_time.cmp(&other.operate_time)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperateType {
    Remember = 0,
    Vague = 1,
    Forget = 2,
    Seen = 3,
}
