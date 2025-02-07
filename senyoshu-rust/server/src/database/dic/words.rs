use chrono::FixedOffset;
use sea_orm::entity::prelude::*;

use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordDefine;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "words")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub wid: WordIdentity,
    #[sea_orm(column_type = "JsonBinary")]
    pub word_define: Option<WordDefine>,
    #[sea_orm(default_value = "now()")]
    pub update_date: chrono::DateTime<FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

