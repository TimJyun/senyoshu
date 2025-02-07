use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

use crate::types::word::mean_entry::{SentenceElement, MeanEntry};
use crate::types::word::wid::WordIdentity;
use crate::types::word::word::Word;

#[cfg_attr(not(target_family = "wasm"), derive(FromJsonQueryResult))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct WordDefine {
    pub word: Word,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub means: Vec<MeanEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loan: Option<Loan>,

    //html element
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub detailed: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub example_sentences: Vec<ExampleSentence>,

}

impl WordDefine {
    pub fn template() -> Self {
        WordDefine {
            word: Word::default(),
            means: Vec::from([MeanEntry::default()]),
            detailed: String::new(),
            loan: None,
            example_sentences: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct ExampleSentence {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ja: Vec<SentenceElement>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub zh: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub en: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WordEntry {
    pub id: WordIdentity,
    pub word_define: WordDefine,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Loan {
    pub language: String,
    pub source_word: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum LoadLanguage {
    荷,
    英,
    拉丁,
    葡,
    意,
    德,
    俄,
    和,
    法,
    西,
}

pub const LOAD_LANGUAGES: [&str; 10] = [
    "荷",
    "英",
    "拉丁",
    "葡",
    "意",
    "德",
    "俄",
    "和",
    "法",
    "西",
];


