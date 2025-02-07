use std::fmt::{Debug, Display};
use std::str::FromStr;

use derive_more::Deref;
use sea_orm::{DbErr, DeriveValueType, TryFromU64};
use serde::{Deserialize, Serialize};

#[cfg_attr(not(target_family = "wasm"), derive(DeriveValueType))]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Deref, Hash)]
pub struct WordIdentity(pub i64);

impl Default for WordIdentity {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i64> for WordIdentity {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Display for WordIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for WordIdentity {
    type Err = <i64 as FromStr>::Err;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(WordIdentity(value.parse::<i64>()?))
    }
}

impl TryFromU64 for WordIdentity {
    fn try_from_u64(n: u64) -> Result<Self, DbErr> {
        if let Ok(n) = i64::try_from(n) {
            Ok(WordIdentity(n))
        } else {
            Err(DbErr::Custom(String::from(
                "WordIdentity: try_from_u64() failed",
            )))
        }
    }
}
