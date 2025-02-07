use chrono::FixedOffset;
use sea_orm::entity::prelude::*;

use senyoshu_common::types::state::State;
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordDefine;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "word_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub pid: i64,
    #[sea_orm(default_value = "now()")]
    pub post_date: chrono::DateTime<FixedOffset>,
    pub author: i64,
    pub wid: WordIdentity,
    #[sea_orm(column_type = "JsonBinary")]
    pub word_define: WordDefine,
    pub state: State,
    #[sea_orm(default_value = "now()")]
    pub update_date: chrono::DateTime<FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
