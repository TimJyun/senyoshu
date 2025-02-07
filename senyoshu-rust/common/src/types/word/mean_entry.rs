use serde::{Deserialize, Serialize};

use crate::types::word::parts_of_speech::DefaultExt;
use crate::types::word::parts_of_speech::PartsOfSpeech;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct MeanEntry {
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub parts_of_speech: PartsOfSpeech,

    //multi-language-explanation
    #[serde(default, skip_serializing_if = "Sentence::is_empty")]
    pub explanation: Sentence,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Sentence {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub zh: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub en: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, Hash)]
pub struct SentenceElement {
    pub txt: String,
    pub ruby: String,
}




#[derive(Copy, Clone)]
pub enum SentenceIndex {
    ZH,
    EN,
}

impl Sentence {
    pub fn is_empty(&self) -> bool {
       self.zh.is_empty() && self.en.is_empty()
    }

    //todo:改名
    pub fn get_mut_by_index(&mut self, index: SentenceIndex) -> &mut String {
        match index {
            SentenceIndex::ZH => &mut self.zh,
            SentenceIndex::EN => &mut self.en,
        }
    }
    pub fn get_by_index(&self, index: SentenceIndex) -> &String {
        match index {
            SentenceIndex::ZH => &self.zh,
            SentenceIndex::EN => &self.en,
        }
    }
}
