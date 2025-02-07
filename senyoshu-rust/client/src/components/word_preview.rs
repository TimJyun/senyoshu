use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use senyoshu_common::types::word::word_entry::WordEntry;

use crate::components::word_node::WordNode;
use crate::storage::setting::{Language, SETTING};

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WordPreviewProps {
    pub word_entry: WordEntry,
}

pub fn WordPreview(props: WordPreviewProps) -> Element {
    let language = SETTING.read().language.unwrap_or(Language::Ja);
    let word_entry = props.word_entry;
    let means = word_entry.word_define.means.into_iter().map(|mean| {
        let pos = mean.parts_of_speech.to_string();
        let mut explanation = match language {
            Language::Zh => mean.explanation.zh.to_string(),
            Language::En => mean.explanation.en.to_string(),
            _ => mean.explanation.en.to_string(),
        };
        if explanation.is_empty() {
            if !mean.explanation.en.is_empty() {
                explanation = mean.explanation.en.to_string();
            } else if !mean.explanation.zh.is_empty() {
                explanation = mean.explanation.zh.to_string();
            } else {
                explanation = String::new()
            }
        }

        rsx! {
            div { style: "display:flex;flex-direction:row;margin-top:auto;margin-bottom:auto",
                span { style: "flex:1;margin-top:auto;margin-bottom:auto",
                    {pos},
                    {explanation}
                }
            }
        }
    });

    rsx! {
        span { style: "display:flex;flex-direction:row;margin-top:auto;margin-bottom:auto",
            span { style: "flex:3;margin-top:auto;margin-bottom:auto",
                WordNode { word: word_entry.id }
            }
            span { style: "flex:7;margin-top:auto;margin-bottom:auto", {means} }
        }
    }
}
