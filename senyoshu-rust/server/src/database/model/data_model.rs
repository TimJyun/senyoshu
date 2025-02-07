use chrono::FixedOffset;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;

use senyoshu_common::types::learn::learn_knowledge_history::OperateRecord;

#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "data_model")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub dmid: i64,
    pub author: i64,

    #[sea_orm(column_type = "JsonBinary")]
    pub history: Vec<OperateRecord>,
    #[sea_orm(default_value = "now()")]
    pub update_time: chrono::DateTime<FixedOffset>,

    pub extend: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
