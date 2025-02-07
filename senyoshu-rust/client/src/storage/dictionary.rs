use std::collections::{HashMap, HashSet};

use chrono::Local;
use derive_more::Deref;
use dioxus::prelude::{GlobalSignal, Readable, ReadableRef, Signal};
use itertools::Itertools;
use serde::Serialize;
use smallvec::{smallvec, SmallVec};
use tracing::{debug, error};

use senyoshu_common::types::api::api::WordQuery;
use senyoshu_common::types::api::dic::SYNC_DIC_API;
use senyoshu_common::types::kanji_alias::Kanji;
use senyoshu_common::types::kanji_detail::{KanjiReference, WordRef};
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordDefine;
use senyoshu_common::util::alias::alias_to_standard;
use senyoshu_common::util::seq_map::SeqMap;
use senyoshu_common::util::string_util::StringUtil;

use crate::storage::permanent_storage::PermanentStorage;
use crate::storage::LAST_UPDATED;

pub static DIC: Dictionary = Dictionary::new();

pub struct Dictionary(GlobalSignal<Dic>);

impl Dictionary {
    const fn new() -> Self {
        Dictionary(Signal::global(|| Dic::from_local_storage()))
    }

    pub fn read(&self) -> ReadableRef<GlobalSignal<Dic>> {
        self.0.read()
    }

    pub fn peek(&self) -> ReadableRef<GlobalSignal<Dic>> {
        self.0.peek()
    }
}

#[derive(Default, Deref, Clone)]
pub struct Dic {
    pub txt_map: SeqMap<String, SmallVec<[WordIdentity; 1]>>,
    pub kana_map: SeqMap<String, SmallVec<[WordIdentity; 1]>>,
    pub char_map: SeqMap<char, Vec<CharIndex>>,
    #[deref]
    dic: HashMap<WordIdentity, WordDefine>,
}

pub type DicModel = HashMap<WordIdentity, WordDefine>;

#[derive(Serialize, Clone)]
pub struct CharIndex {
    frequency: usize,
    word_id: WordIdentity,
}

pub const DIC_LOCAL_STORAGE: &str = "dic";

impl Dic {
    pub fn get() -> DicModel {
        PermanentStorage::get::<DicModel>(DIC_LOCAL_STORAGE).unwrap_or_default()
    }

    fn set(new: &DicModel) {
        PermanentStorage::set::<&DicModel>(DIC_LOCAL_STORAGE, new).unwrap();
    }

    fn from(dic: HashMap<WordIdentity, WordDefine>) -> Self {
        let mut txt_map: HashMap<String, SmallVec<[WordIdentity; 1]>> =
            HashMap::with_capacity(dic.len());
        let mut kana_map: HashMap<String, SmallVec<[WordIdentity; 1]>> =
            HashMap::with_capacity(dic.len());
        let mut char_map: HashMap<char, Vec<CharIndex>> = HashMap::new();
        for (wid, wd) in dic.iter() {
            let txt = wd.word.get_txt();
            if let Some(small_vec) = txt_map.get_mut(&txt) {
                small_vec.push(*wid)
            } else {
                txt_map.insert(txt, smallvec![*wid]);
            }
            let kana = wd.word.get_katakana();
            if let Some(small_vec) = kana_map.get_mut(&kana) {
                small_vec.push(*wid)
            } else {
                kana_map.insert(kana, smallvec![*wid]);
            }

            for (c, count) in wd
                .word
                .elements
                .iter()
                .map(|e| e.txt.chars())
                .flatten()
                .map(|c| (alias_to_standard(c), ()))
                .into_group_map()
            {
                let ci = CharIndex {
                    frequency: count.len(),
                    word_id: *wid,
                };
                if let Some(ci_vec) = char_map.get_mut(&c) {
                    ci_vec.push(ci)
                } else {
                    char_map.insert(c, Vec::from([ci]));
                }
            }
        }

        Dic {
            txt_map: txt_map.into(),
            kana_map: kana_map.into(),
            char_map: char_map.into(),
            dic,
        }
    }

    fn from_local_storage() -> Self {
        Self::from(Self::get())
    }

    //todo: 使用cache api
    pub async fn update() -> bool {
        let last_update = { LAST_UPDATED.peek().dic.to_owned() };
        let updated_words = SYNC_DIC_API.call(&last_update).await;
        if let Ok(words) = updated_words {
            if words.len() > 0 {
                let mut dic = Self::get();
                for (k, v) in words.into_iter() {
                    if let Some(word_define) = v {
                        dic.insert(k, word_define);
                    } else {
                        dic.remove(&k);
                    }
                }
                Self::set(&dic);
                let dic_new = Dic::from(dic);
                let mut dic_ref = DIC.0.write();
                *dic_ref = dic_new;
            }
            LAST_UPDATED.write().dic = Some(Local::now().into());
            debug!("dic update finish");
            true
        } else {
            error!("dic update fail");
            false
        }
    }

    pub fn query_word(&self, word: &WordQuery) -> Option<WordIdentity> {
        let words_txt = self
            .txt_map
            .get(&word.txt)?
            .into_iter()
            .cloned()
            .collect::<HashSet<WordIdentity>>();
        let words_kana = self
            .kana_map
            .get(&StringUtil::ruby_to_katakana(word.ruby.as_str()))?
            .into_iter()
            .cloned()
            .collect::<HashSet<WordIdentity>>();
        let index = words_txt.intersection(&words_kana).next()?;
        Some(*index)
    }

    pub fn query_kanji(&self, kanji: Kanji) -> Option<KanjiReference> {
        let result = self
            .char_map
            .get(&*kanji)?
            .into_iter()
            .map(|it| {
                Some(WordRef {
                    wid: it.word_id,
                    word: self.dic.get(&it.word_id)?.word.to_owned(),
                })
            })
            .filter_map(|it| it)
            .collect_vec();

        Some(KanjiReference::new(*kanji, result))
    }
}
