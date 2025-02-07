use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};

use crate::glossary::jo_yo_kan_ji::YO_MI_MAP;
use crate::types::word::wid::WordIdentity;
use crate::types::word::word::Word;
use crate::util::iter_util::WithNextMutMapItertool;
use crate::util::string_util::{
    HIRAGANA_B, HIRAGANA_D, HIRAGANA_G, HIRAGANA_K, HIRAGANA_P, HIRAGANA_S, HIRAGANA_T, HIRAGANA_Z,
    StringUtil,
};

#[cfg_attr(not(target_family = "wasm"), derive(FromJsonQueryResult))]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WordRef {
    pub wid: WordIdentity,
    pub word: Word,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct KanjiReference {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recorded_onyomi: Vec<(String, Vec<WordRef>)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recorded_kunyomi: Vec<(String, Vec<WordRef>)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_recorded: Vec<(String, Vec<WordRef>)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special: Vec<WordRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uncategorized: Vec<WordRef>,
}

impl KanjiReference {
    pub fn is_empty(&self) -> bool {
        self.recorded_onyomi.is_empty()
            && self.recorded_kunyomi.is_empty()
            && self.not_recorded.is_empty()
            && self.special.is_empty()
            && self.uncategorized.is_empty()
    }

    pub fn new(kanji: char, words: Vec<WordRef>) -> Self {
        let mut recorded_onyomi: HashMap<String, HashSet<WordIdentity>> = HashMap::new();

        let mut recorded_kunyomi: HashMap<String, HashSet<WordIdentity>> = HashMap::new();
        let mut not_recorded: HashMap<String, HashSet<WordIdentity>> = HashMap::new();
        let mut special: HashSet<WordIdentity> = HashSet::new();
        let mut uncategorized = HashSet::new();

        struct TempWordElement {
            ruby: SmallVec<[SmallVec<[char; 8]>; 8]>,
            txt: SmallVec<[char; 8]>,
            proto: SmallVec<[char; 8]>,
        }

        let words_new = words.iter()
            .map(|w| {
                (
                    w.wid,
                    w.word.elements.iter()
                        .map(|we| {
                            TempWordElement {
                                ruby: smallvec![we.ruby.chars().collect()],
                                txt: we.txt.chars().collect(),
                                proto: we.proto.chars().collect(),
                            }
                        })
                        .with_next_mut_map(|mut this_ele, next_ele_opt| {
                            //todo:剪枝
                            if let Some(next_ele) = next_ele_opt {
                                if this_ele.txt.len() == 1 &&
                                    StringUtil::is_kanji(this_ele.txt.first().cloned().unwrap_or(' ')) &&
                                    this_ele.ruby.iter().all(|iter| iter.iter().all(|c| StringUtil::is_hiragana(*c))) &&

                                    next_ele.txt.len() == 1 &&
                                    StringUtil::is_kanji(next_ele.txt.first().cloned().unwrap_or(' ')) &&
                                    next_ele.ruby.iter().all(|iter| iter.iter().all(|c| StringUtil::is_hiragana(*c))) {
                                    let mut this_ele_ruby_vec_tmp = this_ele.ruby.clone();
                                    while let Some(this_ele_ruby_tmp) = this_ele_ruby_vec_tmp.pop() {
                                        let mut next_ele_ruby_vec_tmp = next_ele.ruby.clone();
                                        while let Some(next_ele_ruby_tmp) = next_ele_ruby_vec_tmp.pop() {
                                            if this_ele_ruby_tmp.len() > 1 {
                                                if let Some('っ') = this_ele_ruby_tmp.last() {
                                                    if let Some(next_ele_ruby_tmp_first) = next_ele_ruby_tmp.first() {
                                                        if HIRAGANA_K.chars()
                                                            .chain(HIRAGANA_S.chars())
                                                            .chain(HIRAGANA_T.chars())
                                                            .chain(HIRAGANA_P.chars())
                                                            .contains(&next_ele_ruby_tmp_first) {
                                                            {
                                                                let mut this_ele_ruby_tmp_new = this_ele_ruby_tmp.clone();
                                                                if let Some(this_ele_ruby_last) = this_ele_ruby_tmp_new.last_mut() {
                                                                    *this_ele_ruby_last = 'ち';
                                                                    this_ele.ruby.push(this_ele_ruby_tmp_new);
                                                                }
                                                                let mut this_ele_ruby_tmp_new = this_ele_ruby_tmp.clone();
                                                                if let Some(this_ele_ruby_last) = this_ele_ruby_tmp_new.last_mut() {
                                                                    *this_ele_ruby_last = 'つ';
                                                                    this_ele.ruby.push(this_ele_ruby_tmp_new);
                                                                }
                                                            }

                                                            if HIRAGANA_K.chars().contains(&next_ele_ruby_tmp_first) {
                                                                let mut this_ele_ruby_tmp_new = this_ele_ruby_tmp.clone();
                                                                if let Some(this_ele_ruby_last) = this_ele_ruby_tmp_new.last_mut() {
                                                                    *this_ele_ruby_last = 'き';
                                                                    this_ele.ruby.push(this_ele_ruby_tmp_new);
                                                                }
                                                                let mut this_ele_ruby_tmp_new = this_ele_ruby_tmp.clone();
                                                                if let Some(this_ele_ruby_last) = this_ele_ruby_tmp_new.last_mut() {
                                                                    *this_ele_ruby_last = 'く';
                                                                    this_ele.ruby.push(this_ele_ruby_tmp_new);
                                                                }
                                                            }


                                                            if HIRAGANA_P.chars().contains(&next_ele_ruby_tmp_first) {
                                                                let mut next_ele_ruby_tmp_new = next_ele_ruby_tmp.clone();
                                                                if let Some(next_ele_ruby_first) = next_ele_ruby_tmp_new.first_mut() {
                                                                    *next_ele_ruby_first = StringUtil::hiragana_to_no_sign(*next_ele_ruby_first);
                                                                    next_ele.ruby.push(next_ele_ruby_tmp_new);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if let Some('ん') = this_ele_ruby_tmp.last() {
                                                    if let Some(next_ele_ruby_first) = next_ele_ruby_tmp.first() {
                                                        if HIRAGANA_P.chars()
                                                            .chain(HIRAGANA_B.chars())
                                                            .contains(next_ele_ruby_first) {
                                                            let mut next_ele_ruby = next_ele_ruby_tmp.clone();
                                                            if let Some(next_ele_ruby_first) = next_ele_ruby.first_mut() {
                                                                *next_ele_ruby_first = StringUtil::hiragana_to_no_sign(*next_ele_ruby_first);
                                                                next_ele.ruby.push(next_ele_ruby);
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            if let Some(next_ele_ruby_first) = next_ele_ruby_tmp.first() {
                                                if HIRAGANA_G.chars()
                                                    .chain(HIRAGANA_Z.chars())
                                                    .chain(HIRAGANA_D.chars())
                                                    .chain(HIRAGANA_B.chars())
                                                    .contains(next_ele_ruby_first) {
                                                    let mut next_ele_ruby = next_ele_ruby_tmp.clone();
                                                    if let Some(next_ele_ruby_first) = next_ele_ruby.first_mut() {
                                                        *next_ele_ruby_first = StringUtil::hiragana_to_no_sign(*next_ele_ruby_first);
                                                        next_ele.ruby.push(next_ele_ruby);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            if this_ele.proto.len() > 0 {
                                this_ele.ruby = smallvec![this_ele.proto.to_owned()];
                            }
                            this_ele
                        }).collect_vec()
                )
            }).collect_vec();

        let mut time = 0;
        words_new.iter().for_each(|(wid, word)| {
            time = time + 1;
            for (i, ele) in word.iter().enumerate() {
                if ele.txt.contains(&kanji) {
                    if ele.txt.len() == 1 {
                        let mut finded = false;

                        for ruby in ele.ruby.iter() {
                            let ruby_str = ruby.into_iter().collect::<String>();
                            if let Some(yomi) = YO_MI_MAP.get(&kanji) {
                                if yomi.on.contains(&ruby_str) {
                                    recorded_onyomi.entry(ruby_str).or_default().insert(*wid);
                                    finded = true;
                                    continue;
                                }

                                let mut ruby_str_extend = ruby_str.clone();
                                let mut i2 = i + 1;
                                while let Some(tmp_ele) = word.get(i2) {
                                    if tmp_ele.txt.iter().all(|c| StringUtil::is_hiragana(*c)) {
                                        tmp_ele
                                            .txt
                                            .iter()
                                            .for_each(|hirakana| ruby_str_extend.push(*hirakana));
                                        i2 = i2 + 1;
                                        continue;
                                    }
                                    break;
                                }
                                yomi.kun.iter().for_each(|kunyomi| {
                                    if ruby_str_extend.starts_with(kunyomi) {
                                        recorded_kunyomi
                                            .entry(ruby_str.clone())
                                            .or_default()
                                            .insert(*wid);
                                        finded = true;
                                    }
                                });
                            }
                        }
                        let ruby_raw = ele.ruby.get(0).unwrap().into_iter().collect::<String>();
                        if finded == false {
                            not_recorded.entry(ruby_raw).or_default().insert(*wid);
                        }
                    } else if ele.txt.len() > 1 {
                        if ele.proto.len() > 0 {
                            special.insert(*wid);
                        } else {
                            uncategorized.insert(*wid);
                        }
                    }
                }
            }
        });


        let mut not_recorded = not_recorded
            .into_iter()
            .filter_map(|(kana, wids)| {
                if let Some(map) = recorded_kunyomi.get_mut(&kana)
                    .or(recorded_onyomi.get_mut(&kana)) {
                    wids.into_iter()
                        .for_each(|wid| {
                            map.insert(wid);
                        });
                    None
                } else {
                    Some((kana, wids))
                }
            })
            .map(|(kana, wids)| {
                let mut wref = wids
                    .into_iter()
                    .map(|wid| words.iter().find(|word_ref| word_ref.wid == wid).cloned())
                    .filter_map(|it| it)
                    .collect_vec();
                wref.sort_by_cached_key(|r| r.word.get_txt());
                (kana, wref)
            })
            .collect_vec();
        not_recorded.sort_by_cached_key(|it| it.0.to_string());


        let mut recorded_onyomi = recorded_onyomi
            .into_iter()
            .map(|(kana, wids)| {
                let mut wref = wids
                    .into_iter()
                    .map(|wid| words.iter().find(|word_ref| word_ref.wid == wid).cloned())
                    .filter_map(|it| it)
                    .collect_vec();
                wref.sort_by_cached_key(|r| r.word.get_txt());
                (kana, wref)
            })
            .collect_vec();
        recorded_onyomi.sort_by_cached_key(|it| it.0.to_string());

        let mut recorded_kunyomi = recorded_kunyomi
            .into_iter()
            .map(|(kana, wids)| {
                let mut wref = wids
                    .into_iter()
                    .map(|wid| words.iter().find(|word_ref| word_ref.wid == wid).cloned())
                    .filter_map(|it| it)
                    .collect_vec();
                wref.sort_by_cached_key(|r| r.word.get_txt());
                (kana, wref)
            })
            .collect_vec();
        recorded_kunyomi.sort_by_cached_key(|it| it.0.to_string());


        let mut special = special
            .into_iter()
            .map(|wid| words.iter().find(|word_ref| word_ref.wid == wid).cloned())
            .filter_map(|it| it)
            .collect_vec();
        special.sort_by_cached_key(|it| it.word.get_txt());

        let mut uncategorized = uncategorized
            .into_iter()
            .map(|wid| words.iter().find(|word_ref| word_ref.wid == wid).cloned())
            .filter_map(|it| it)
            .collect_vec();
        uncategorized.sort_by_cached_key(|it| it.word.get_txt());

        Self {
            recorded_onyomi,
            recorded_kunyomi,
            not_recorded,
            special,
            uncategorized,
        }
    }

    pub fn get_rubies<'a>(&'a self) -> impl Iterator<Item=String> + 'a {
        self.recorded_onyomi
            .iter()
            .map(|it| StringUtil::ruby_to_katakana(it.0.as_str()))
            .chain(
                self.recorded_kunyomi
                    .iter()
                    .chain(self.not_recorded.iter())
                    .map(|it| it.0.to_string()),
            )
    }
}
