use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(EnumIter, DeriveActiveEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[sea_orm(rs_type = "i16", db_type = "SmallInteger")]
pub enum State {
    Pending = 0,
    Pass = 1,
    Cancel = 2,
    Withdraw = 3,
}
