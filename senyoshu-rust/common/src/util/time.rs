use std::cmp::Ordering;
use std::ops::Add;

use chrono::{DateTime, Utc};
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};

#[derive(
Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, Deref, DerefMut, Hash,
)]
pub struct UtcTimeStamp(pub i64);

impl PartialOrd<Self> for UtcTimeStamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for UtcTimeStamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Add for UtcTimeStamp {
    type Output = UtcTimeStamp;

    fn add(self, rhs: Self) -> Self::Output {
        UtcTimeStamp(self.0 + rhs.0)
    }
}

impl From<DateTime<Utc>> for UtcTimeStamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value.timestamp_millis())
    }
}

impl UtcTimeStamp {
    pub fn now() -> Self {
        Self(Utc::now().timestamp_millis())
    }

    pub const fn day() -> Self {
        Self(24 * 60 * 60 * 1000)
    }

    pub const fn hour() -> Self {
        Self(60 * 60 * 1000)
    }

    pub const fn mul(mut self, mul: i64) -> Self {
        self.0 *= mul;
        self
    }

    pub fn to_string(&self) -> Option<String> {
        DateTime::from_timestamp_millis(self.0).map(|date_time| date_time.to_string())
    }
}
