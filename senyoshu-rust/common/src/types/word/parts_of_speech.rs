use std::collections::HashSet;
use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

//品词分类
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct PartsOfSpeech {
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub noun: Option<Noun>,
    // "some" means it is a type of verb , "none" mean s it's not verb
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub verb: Option<VerbClass>,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub compound: Compound,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub adjective: bool,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub na_adjective: bool,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub adverb: bool,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub interjection: bool,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub pronouns: bool,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub phrase: bool,

    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub onomatopoeia:bool,

    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub others: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub struct Noun {
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub people_name: bool,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub place_name: bool,
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum Compound {
    #[default]
    IsNot,
    Prefix,
    Suffix,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub struct VerbClass {
    //動詞活用
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub conjugation: Option<VerbConjugation>,

    //結合価 valency
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub intransitive: bool,
    #[serde(default, skip_serializing_if = "DefaultExt::eq_default")]
    pub transitive: bool,
}

//動詞活用
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum VerbConjugation {
    FiveRowVerb,
    OneRowVerb,
    IrregularVerb,
}

impl PartsOfSpeech {
    pub fn is_undefined(&self) -> bool {
        self == &PartsOfSpeech::default()
    }
}

pub trait DefaultExt: Default {
    fn eq_default(&self) -> bool;
}

impl<T: Default + PartialEq> DefaultExt for T {
    fn eq_default(&self) -> bool {
        self == &Self::default()
    }
}


impl Display for PartsOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut part_of_speech_str: Vec<char> = Vec::with_capacity(32);
        part_of_speech_str.push('[');
        let part_of_speech = self;
        if part_of_speech.compound != Compound::IsNot {
            part_of_speech_str.push('接');
            match part_of_speech.compound {
                Compound::Prefix => {
                    part_of_speech_str.push('頭');
                }
                Compound::Suffix => {
                    part_of_speech_str.push('尾');
                }
                _ => {
                    debug_assert!(false);
                }
            }
            part_of_speech_str.push('辞');
            part_of_speech_str.push('|');
        }
        if part_of_speech.pronouns {
            part_of_speech_str.push('代');
            part_of_speech_str.push('|');
        }

        if let Some(noun) = part_of_speech.noun {
            if noun.people_name {
                part_of_speech_str.push('人');
            }
            if noun.place_name {
                if noun.people_name {
                    part_of_speech_str.push('&');
                }
                part_of_speech_str.push('地');
            }
            part_of_speech_str.push('名');
            part_of_speech_str.push('|');
        }
        if part_of_speech.adjective {
            part_of_speech_str.push('形');
            part_of_speech_str.push('|');
        }
        if part_of_speech.na_adjective {
            part_of_speech_str.push('形');
            part_of_speech_str.push('動');
            part_of_speech_str.push('|');
        }
        if part_of_speech.adverb {
            part_of_speech_str.push('副');
            part_of_speech_str.push('|');
        }
        if part_of_speech.interjection {
            part_of_speech_str.push('嘆');
            part_of_speech_str.push('|');
        }
        if part_of_speech.onomatopoeia{
            part_of_speech_str.push('擬');
            part_of_speech_str.push('聲');
            part_of_speech_str.push('|');

        }

        if let Some(verb) = &part_of_speech.verb {
            if let Some(conjugation) = &verb.conjugation {
                match conjugation {
                    VerbConjugation::FiveRowVerb => {
                        part_of_speech_str.push('五');
                        part_of_speech_str.push('段');
                    }
                    VerbConjugation::OneRowVerb => {
                        part_of_speech_str.push('一');
                        part_of_speech_str.push('段');
                    }
                    VerbConjugation::IrregularVerb => {
                        part_of_speech_str.push('不');
                        part_of_speech_str.push('規');
                        part_of_speech_str.push('則');
                    }
                }
            }
            if verb.intransitive || verb.transitive {
                if verb.conjugation.is_some() {
                    part_of_speech_str.push('(');
                }
                if verb.intransitive {
                    part_of_speech_str.push('自');
                    if verb.transitive {
                        part_of_speech_str.push('&');
                        part_of_speech_str.push('他');
                    }
                } else {
                    debug_assert!(verb.transitive);
                    part_of_speech_str.push('他');
                }
                if verb.conjugation.is_some() {
                    part_of_speech_str.push(')');
                }
            }
            part_of_speech_str.push('動');
            part_of_speech_str.push('詞');
            part_of_speech_str.push('|');
        }

        if part_of_speech.phrase {
            part_of_speech_str.push('句');
            part_of_speech_str.push('|');
        }

        let mut others = self.others.iter().cloned().collect_vec();
        others.sort();
        for o in others {
            for c in o.chars() {
                part_of_speech_str.push(c);
            }
            part_of_speech_str.push('|');
        }


        if part_of_speech_str.len() > 1 {
            debug_assert!(*part_of_speech_str.last().unwrap() == '|');
            *part_of_speech_str.last_mut().unwrap() = ']';
        } else {
            part_of_speech_str.clear();
            "[未知品詞]"
                .chars()
                .for_each(|it| part_of_speech_str.push(it));
        }

        write!(f, "{}", part_of_speech_str.iter().collect::<String>())
    }
}
