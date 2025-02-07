use dioxus::prelude::*;
use smallvec::SmallVec;

use senyoshu_common::types::kanji_alias::Kanji;
use senyoshu_common::types::learn::knowledge::{Knowledge, KnowledgeType};
use senyoshu_common::types::learn::plan::Plan;
use senyoshu_common::types::word::tones::Tone;
use senyoshu_common::types::word::word_entry::WordEntry;

use crate::page::learn::kana::KanaData;
use crate::page::learn::kanji::KanjiData;
use crate::page::learn::txt::TxtData;
use crate::storage::dictionary::Dic;

pub mod learn_page;

pub mod kana;
pub mod kanji;
mod learn;
pub mod preview;
pub mod preview_knowledge;
pub mod txt;

#[derive(Props, Clone, PartialEq)]
pub struct LearnKnowledgeProps<T: PartialEq + Clone + 'static> {
    pub knowledge: Knowledge,
    pub data: T,
    pub plan: Plan,
    pub on_ended: EventHandler,
}

#[derive(Clone, PartialEq)]
pub enum KnowledgeData {
    Kana(KanaData),
    Txt(TxtData),
    Kanji(KanjiData),
}

impl KnowledgeData {
    pub fn get_learn_data(knowledge: &Knowledge, dic: &Dic) -> Option<Self> {
        Some(match knowledge.knowledge_type {
            KnowledgeType::Kanji => {
                let kanji = Kanji::try_from(knowledge.key.chars().next()?).ok()?;
                KnowledgeData::Kanji(KanjiData {
                    kanji,
                    references: dic.query_kanji(kanji)?,
                })
            }
            KnowledgeType::Txt => {
                let mut words: SmallVec<[WordEntry; 1]> = dic
                    .txt_map
                    .get(&knowledge.key)?
                    .into_iter()
                    .map(|wid| {
                        Some(WordEntry {
                            id: *wid,
                            word_define: dic.get(&wid)?.to_owned(),
                        })
                    })
                    .filter_map(|it| it)
                    .collect();
                words.sort_by_cached_key(|it| it.word_define.word.get_katakana());
                KnowledgeData::Txt(TxtData {
                    txt: knowledge.key.to_string(),
                    words,
                })
            }
            KnowledgeType::Kana => {
                let mut words: SmallVec<[WordEntry; 1]> = dic
                    .kana_map
                    .get(&knowledge.key)?
                    .into_iter()
                    .map(|wid| {
                        Some(WordEntry {
                            id: *wid,
                            word_define: dic.get(&wid)?.to_owned(),
                        })
                    })
                    .filter_map(|it| it)
                    .collect();
                words.sort_by_cached_key(|it| it.word_define.word.get_txt());
                KnowledgeData::Kana(KanaData {
                    kana: knowledge.key.to_string(),
                    words,
                })
            }
        })
    }

    pub fn get_preview_sound(&self) -> Vec<(String, Option<Tone>)> {
        //todo: 支持带声调的tts
        match self {
            KnowledgeData::Kana(kana) => kana
                .words
                .iter()
                .map(|we| (we.word_define.word.get_ruby(), None))
                .collect(),
            KnowledgeData::Txt(txt) => txt
                .words
                .iter()
                .map(|we| (we.word_define.word.get_ruby(), None))
                .collect(),
            KnowledgeData::Kanji(kanji) => kanji
                .references
                .get_rubies()
                .map(|ruby| (ruby, None))
                .collect(),
        }
    }
}
