use dioxus::core_macro::rsx;
use dioxus::prelude::*;

use crate::page::learn::kana::PreviewKana;
use crate::page::learn::kanji::PreviewKanji;
use crate::page::learn::txt::PreviewTxt;
use crate::page::learn::KnowledgeData;

#[component]
pub fn PreviewKnowledge(data: KnowledgeData) -> Element {
    match data {
        KnowledgeData::Kanji(data) => {
            rsx! {
                PreviewKanji { data }
            }
        }
        KnowledgeData::Txt(data) => {
            rsx! {
                PreviewTxt { data }
            }
        }
        KnowledgeData::Kana(data) => {
            rsx! {
                PreviewKana { data }
            }
        }
    }
}
