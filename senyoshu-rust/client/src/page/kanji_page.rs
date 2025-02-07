use dioxus::core_macro::rsx;
use dioxus::prelude::IntoDynNode;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::error;

use senyoshu_common::types::kanji_alias::Kanji;
use senyoshu_common::types::kanji_detail::KanjiReference;
use senyoshu_common::util::iter_util::WithNextMutMapItertool;
use senyoshu_common::util::string_util::StringUtil;

use crate::components::sound::Sound;
use crate::components::word_node::WordNode;
use crate::storage::dictionary::DIC;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KanjiPageProps {
    kanji: Kanji,
}

pub fn KanjiPage(props: KanjiPageProps) -> Element {
    let kanji = &props.kanji;
    let dic = DIC.read();

    if let Some(kanji_detail) = dic.query_kanji(*kanji) {
        kanji_page_node(**kanji, kanji_detail, false)
    } else {
        error!("kanji_page load failed");
        rsx! { "the kanji '{kanji}' not found in the dictionary" }
    }
}

pub fn kanji_page_node(
    kanji: char,
    kanji_reference: KanjiReference,
    kanji_keep_left: bool,
) -> Element {
    let recorded_onyomi = kanji_reference.recorded_onyomi.into_iter()
        .map(|(kana, words)| {
            let ws = words.into_iter()
                .with_next_mut_map(move |word, next| {
                    let next_is_some = next.is_some();
                    rsx! {
                        WordNode { word: word.wid }
                        " "
                        if next_is_some {
                            " , "
                        }
                    }
                });

            rsx! {
                div { style: "display:flex",
                    span { style: "width:5rem;margin-top:auto;margin-bottom:auto",
                        Sound { text: StringUtil::ruby_to_katakana(kana.as_str()), { StringUtil::ruby_to_katakana(kana.as_str()) } }
                    }
                    span { style: "flex:1", {ws} }
                }
            }
        });

    let recorded_kunyomi = kanji_reference
        .recorded_kunyomi
        .into_iter()
        .map(|(kana, words)| {
            let ws = words.into_iter().with_next_mut_map(move |word, next| {
                let next_is_some = next.is_some();
                let _ruby = word.word.get_ruby();
                rsx! {
                    WordNode { word: word.wid }
                    if next_is_some {
                        " , "
                    }
                }
            });

            rsx! {
                div { style: "display:flex",
                    span { style: "width:5rem;margin-top:auto;margin-bottom:auto",
                        Sound { text: StringUtil::ruby_to_katakana(kana.as_str()), {kana} }
                    }
                    span { style: "flex:1", {ws} }
                }
            }
        });

    let not_recorded_is_some = kanji_reference.not_recorded.len() > 0;
    let not_recorded = kanji_reference
        .not_recorded
        .into_iter()
        .map(|(kana, words)| {
            let ws = words.into_iter().with_next_mut_map(move |word, next| {
                let next_is_some = next.is_some();
                let _ruby = word.word.get_ruby();
                rsx! {
                    WordNode { word: word.wid }
                    if next_is_some {
                        " , "
                    }
                }
            });

            rsx! {
                div { style: "display:flex",
                    span { style: "width:5rem;margin-top:auto;margin-bottom:auto",
                        Sound { text: StringUtil::ruby_to_katakana(kana.as_str()), {kana} }
                    }
                    span { style: "flex:1", {ws} }
                }
            }
        });

    let special_is_some = kanji_reference.special.len() > 0;
    let special = kanji_reference
        .special
        .into_iter()
        .with_next_mut_map(|word, next| {
            let next_is_some = next.is_some();
            let _ruby = word.word.get_ruby();
            rsx! {
                WordNode { word: word.wid }
                if next_is_some {
                    " , "
                }
            }
        });

    let uncategorized_is_some = kanji_reference.uncategorized.len() > 0;
    let uncategorized =
        kanji_reference
            .uncategorized
            .into_iter()
            .with_next_mut_map(|word, next| {
                let next_is_some = next.is_some();
                let _ruby = word.word.get_ruby();
                rsx! {
                    WordNode { word: word.wid }
                    if next_is_some {
                        " , "
                    }
                }
            });

    rsx! {
        div {
            style: if kanji_keep_left {
                "font-size:2rem;"
            } else {
                "text-align:center;font-size:2rem;"
            },
            "{kanji}"
        }
        div { {recorded_onyomi} }
        div { {recorded_kunyomi} }
        if not_recorded_is_some {
            div {
                div { style: "border-top-width:1px;border-top-style: dotted;width:7rem;font-size:8px",
                    "表外音訓"
                }
                {not_recorded}
            }
        }
        if special_is_some {
            div {
                div { style: "border-top-width:1px;border-top-style: dotted;width:7rem;font-size:8px",
                    "熟字訓"
                }
                {special}
            }
        }

        if uncategorized_is_some {
            div {
                div { style: "border-top-width:1px;border-top-style: dotted;width:7rem;font-size:8px",
                    "未分類"
                }
                {uncategorized}
            }
        }
    }
}
