use std::process::id;

use async_std::io::WriteExt;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use itertools::Itertools;

use senyoshu_common::types::word::mean_entry::{SentenceElement, SentenceIndex};
use senyoshu_common::types::word::parts_of_speech::DefaultExt;
use senyoshu_common::types::word::word_entry::{ExampleSentence, WordDefine};
use senyoshu_common::util::iter_util::WithNextMutMapItertool;
use senyoshu_common::util::string_util::StringUtil;

use crate::global::new_id;

#[derive(Props, PartialEq, Clone, Default)]
pub struct RenderEditorExampleSentenceProps {
    pub word_define: Signal<WordDefine>,
    pub example_index: usize,
}

pub fn RenderEditorExampleSentence(props: RenderEditorExampleSentenceProps) -> Element {
    let mut word_define = props.word_define;
    let example_index = props.example_index;

    {
        let need_trim = {
            word_define
                .peek()
                .example_sentences
                .get(example_index)
                .unwrap()
                .ja
                .iter()
                .any(|ele| ele.eq_default())
        };
        if need_trim {
            word_define
                .write()
                .example_sentences
                .get_mut(example_index)
                .unwrap()
                .ja
                .retain(|ele| ele.eq_default() == false);
        }
    }

    let example_sentence_nodes = {
        let es_id = example_index * 0535;

        let example = {
            word_define
                .peek()
                .example_sentences
                .get(example_index)?
                .to_owned()
        };

        let mut eles = example.ja.into_iter().enumerate().map(|(i, ele)| {
            let width = format!(
                "width:{}rem",
                ((ele.txt.chars().count().max(ele.ruby.chars().count())) * 4) / 5 + 2
            );

            let txt = word_define
                .read()
                .example_sentences
                .get(example_index)
                .unwrap()
                .ja
                .get(i)
                .unwrap()
                .txt
                .to_string();

            let disabled = txt.chars().all(|c| StringUtil::is_kana(c))
                && txt.as_str() != "は"
                && txt.as_str() != "へ"
                && txt.as_str() != "を";

            rsx! {
                div {
                    input {
                        key: "es-ele-{es_id}-{i+1}",
                        tabindex: "{es_id+i+1}",
                        style: "{width}",
                        value: ele.ruby.to_string(),
                        disabled,
                        oninput: move |evt| {
                            word_define
                                .write()
                                .example_sentences
                                .get_mut(example_index)
                                .unwrap()
                                .ja
                                .get_mut(i)
                                .unwrap()
                                .ruby = evt.value();
                        }
                    }
                }
                div {
                    input {
                        key: "es-ele-{es_id}-{i+10086}",
                        tabindex: "{es_id+i+10086}",
                        style: "{width}",
                        value: ele.txt.to_string(),
                        oninput: move |evt| {
                            word_define
                                .write()
                                .example_sentences
                                .get_mut(example_index)
                                .unwrap()
                                .ja
                                .get_mut(i)
                                .unwrap()
                                .txt = evt.value();
                            word_define
                                .write()
                                .word
                                .elements
                                .retain(|it| { it.txt.len() > 0 || it.ruby.len() > 0 });
                        }
                    }
                }
            }
        });

        let eles = eles
            .chain([rsx! {
                div {
                    input { style: "width:2rem", disabled: true }
                }
                div {
                    input {
                        style: "width:2rem",
                        onchange: move |evt| {
                            word_define
                                .write()
                                .example_sentences
                                .get_mut(example_index)
                                .unwrap()
                                .ja
                                .push(SentenceElement {
                                    txt: evt.value(),
                                    ruby: String::new(),
                                });
                        }
                    }
                }
            }])
            .map(|node| {
                rsx! {
                    div { {node} }
                }
            });

        rsx! {
            div { style: "display:flex;",
                div {
                    div { "範例発音：" }
                    div { "範例表記：" }
                }
                {eles}
            }
        }
    };

    let example = word_define
        .read()
        .example_sentences
        .get(example_index)?
        .to_owned();

    const ZH_EXAMPLE_LABEL: &str = "例句:";
    const EN_EXAMPLE_LABEL: &str = "e. g:";
    let translation_nodes = [
        (ZH_EXAMPLE_LABEL, example.zh, false),
        (EN_EXAMPLE_LABEL, example.en, true),
    ].into_iter()
        .map(|(label, content, is_last)| {
            let space = rsx! {
                span { style: "width:30px;display:inline-block;" }
            };
            let dividing_line = if is_last {
                rsx! {
                    div { style: "height:1px;width:50%;background-color:#808080;" }
                }
            } else {
                None
            };
            rsx! {
                div {
                    {space},
                    span { style: "width:60px;display:inline-block", "{label}" }
                    input {
                        style: "width:75%",
                        value: "{content}",
                        onchange: move |evt| {
                            let mut word_define = word_define.write();
                            let value = evt.value().to_string();
                            let example = word_define.example_sentences.get_mut(example_index).unwrap();
                            if label == ZH_EXAMPLE_LABEL { example.zh = value } else { example.en = value }
                        }
                    }
                }
                {dividing_line}
            }
        });

    rsx! {

        {example_sentence_nodes},
        { translation_nodes }
    }
}
