use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use derive_more::Deref;
use serde::{Deserialize, Serialize};

use crate::util::alias::alias_to_standard;
use crate::util::string_util::StringUtil;

pub type KanjiAliasList = Vec<KanjiAlias>;

#[derive(Serialize, Deserialize)]
pub struct KanjiAlias {
    kanji: char,
    alias: Vec<char>,
}

#[derive(Serialize, Deserialize, Deref, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Kanji {
    c: char,
}

impl Default for Kanji {
    fn default() -> Self {
        Self { c: '一' }
    }
}

impl TryFrom<char> for Kanji {
    type Error = &'static str;

    fn try_from(kanji: char) -> Result<Self, Self::Error> {
        if StringUtil::is_kanji(kanji) {
            Ok(Self {
                c: alias_to_standard(kanji),
            })
        } else {
            Err("Not A Kanji Char")
        }
    }
}

impl TryFrom<&str> for Kanji {
    type Error = &'static str;

    fn try_from(kanji_str: &str) -> Result<Self, Self::Error> {
        if let Some(kanji) = kanji_str.chars().next() {
            if StringUtil::is_kanji(kanji) {
                Ok(Self {
                    c: alias_to_standard(kanji),
                })
            } else {
                Err("Not A Kanji Char")
            }
        } else {
            Err("Not A Kanji Char")
        }
    }
}

impl Display for Kanji {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.c, f)
    }
}

impl FromStr for Kanji {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(c) = s.chars().next() {
            Kanji::try_from(c)
        } else {
            Err("Not A Kanji Char")
        }
    }
}
