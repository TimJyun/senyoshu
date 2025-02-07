use itertools::Itertools;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::types::word::parts_of_speech::DefaultExt;
use crate::types::word::tones::Tones;
use crate::util::alias::alias_to_standard;
use crate::util::number::number_to_japanese;
use crate::util::string_util::StringUtil;

#[derive(FromJsonQueryResult, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, Hash)]
pub struct Word {
    pub elements: Vec<WordElement>,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub tones: Tones,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, Hash)]
pub struct WordElement {
    pub txt: String,
    pub ruby: String,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub proto: String,
}

impl Word {
    pub fn get_kanji_dedup_iter(&self) -> impl Iterator<Item=char> {
        self.elements
            .iter()
            .map(|it| it.txt.chars())
            .flatten()
            .filter(|c| StringUtil::is_kanji(*c))
            .sorted_unstable()
            .dedup()
    }

    pub fn get_txt(&self) -> String {
        let mut rv: Vec<char> = Vec::new();
        self.elements.iter().for_each(|it| {
            it.txt.chars().for_each(|c| {
                rv.push(alias_to_standard(c));
            });
        });
        String::from_iter(rv)
    }

    pub fn get_ruby(&self) -> String {
        let mut rv: Vec<char> = Vec::new();
        self.elements.iter().for_each(|it| {
            it.ruby.chars().for_each(|c| {
                rv.push(c);
            });
        });
        String::from_iter(rv)
    }

    pub fn get_katakana(&self) -> String {
        let mut groups: SmallVec<[String; 8]> = SmallVec::new();
        self.elements.iter().for_each(|ele| {
            if ele.txt.chars().all(|c| StringUtil::is_kana(c)) {
                if let Some(last) = groups.last_mut() {
                    last.push_str(ele.ruby.as_str());
                } else {
                    groups.push(ele.ruby.to_string())
                }
            } else {
                groups.push(ele.ruby.to_string())
            }
        });

        groups.into_iter()
            .map(|it| { StringUtil::ruby_to_katakana(&it) })
            .collect::<String>()
    }

    pub fn from_u64(num: u64) -> Self {
        number_to_japanese(num)
    }
}




