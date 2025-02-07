use std::fmt::Display;
use std::str::FromStr;

use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Knowledge {
    pub knowledge_type: KnowledgeType,
    pub key: String,
}

impl Serialize for Knowledge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        String::serialize(&self.to_string(), serializer)
    }
}

impl<'de> Deserialize<'de> for Knowledge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        match String::deserialize(deserializer) {
            Ok(str) => {
                if let Ok(know) = Knowledge::from_str(str.as_str()) {
                    Ok(know)
                } else {
                    Err(D::Error::custom("error"))
                }
            }
            Err(err) => Err(err),
        }
    }
}

impl Display for Knowledge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let knowledge_type = self.knowledge_type;
        let key = &self.key;
        write!(f, "{knowledge_type}|{key}")
    }
}

impl FromStr for Knowledge {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split('|');
        let k_type = s
            .next()
            .map(|it| KnowledgeType::from_str(it).ok())
            .flatten();
        let key = s.next();
        if let (Some(k_type), Some(key)) = (k_type, key) {
            Ok(Knowledge {
                knowledge_type: k_type,
                key: key.to_string(),
            })
        } else {
            Err(())
        }
    }
}

#[derive(
Display,
EnumString,
Serialize,
Deserialize,
Copy,
Clone,
Debug,
PartialEq,
Eq,
Hash,
EnumIter,
DeriveActiveEnum,
)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum KnowledgeType {
    Kanji = 0,
    Txt = 1,
    Kana = 2,
}
