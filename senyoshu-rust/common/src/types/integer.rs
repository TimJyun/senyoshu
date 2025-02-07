use std::fmt::Display;
use std::str::FromStr;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[cfg_attr(not(target_family = "wasm"), derive(FromJsonQueryResult))]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Integer(pub i64);

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for Integer {
    type Err = <i64 as FromStr>::Err;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Integer(value.parse::<i64>()?))
    }
}
