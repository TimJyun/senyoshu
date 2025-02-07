use chrono::FixedOffset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SoundMeta {
    pub sid: i64,
    pub update_date: chrono::DateTime<FixedOffset>,
    pub author: i64,
    pub katakana: String,
    pub tone: u8,
    pub checked: Option<bool>,
}
