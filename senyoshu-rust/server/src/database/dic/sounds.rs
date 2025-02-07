use chrono::FixedOffset;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sounds")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub katakana: String,
    #[sea_orm(primary_key)]
    pub tone: i16,
    #[sea_orm(primary_key)]
    pub role: String,
    pub sound: Vec<u8>,
    #[sea_orm(default_value = "now()")]
    pub update_date: chrono::DateTime<FixedOffset>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
