use chrono::{DateTime, FixedOffset};
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter};
use sea_orm::DerivePrimaryKey;
use sea_orm::EntityTrait;
use sea_orm::PrimaryKeyTrait;
use serde::Serialize;

use senyoshu_common::types::api::session::SessionVec;

#[derive(Serialize, Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: i64,
    #[sea_orm(default_value = "now()")]
    pub register_date: DateTime<FixedOffset>,
    #[sea_orm(unique)]
    pub username: String,
    pub e_mail: Option<String>,
    #[serde(skip)]
    pub passwd_hash2: String,

    //permission
    #[sea_orm(default_value = true)]
    pub post_permission: bool,
    #[sea_orm(default_value = false)]
    pub restrict_user: bool,
    #[sea_orm(default_value = false)]
    pub content_maintainer: bool,

    pub vip: Option<DateTime<FixedOffset>>,

    #[sea_orm(column_type = "JsonBinary")]
    pub sessions: Option<SessionVec>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

