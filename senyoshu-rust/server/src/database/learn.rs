use chrono::FixedOffset;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;

use senyoshu_common::types::learn::knowledge::KnowledgeType;
use senyoshu_common::types::learn::learn_knowledge_history::OperateRecord;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "learn")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: i64,
    #[sea_orm(primary_key)]
    pub knowledge_type: KnowledgeType,
    #[sea_orm(primary_key)]
    pub knowledge_key: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub history: Vec<OperateRecord>,
    #[sea_orm(default_value = "now()")]
    pub update_time: chrono::DateTime<FixedOffset>,
    pub freeze_time: Option<chrono::DateTime<FixedOffset>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
