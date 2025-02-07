use std::iter;

use dioxus::prelude::*;
use itertools::Itertools;

use senyoshu_common::types::word::parts_of_speech::{PartsOfSpeech, VerbClass, VerbConjugation};
use senyoshu_common::types::word::word::WordElement;
use senyoshu_common::types::word::word_entry::WordDefine;
use senyoshu_common::util::alias::alias_to_standard;
use senyoshu_common::util::string_util::StringUtil;

use crate::components::button::Button;
use crate::components::editor::tones::Tones;

#[derive(Props, PartialEq, Clone)]
pub struct WordCreatorProps {
    pub word_define: Signal<WordDefine>,
}

pub fn WordCreator(props: WordCreatorProps) -> Element {
    let mut word_define = props.word_define;
    let word_define_read = word_define.read();
    let word_elements = &word_define_read.word.elements;

    let word_elements_labels = rsx! {
        div { "元型：" }
        div { "発音：" }
        div { "表記：" }
    };
    let word_elements_editor = [word_elements_labels]
        .into_iter().chain(
        word_elements
            .iter()
            .enumerate()
            .map(|(i, ele)| {
                let width = format!(
                    "width:{}rem",
                    ((ele.txt.chars().count().max(ele.ruby.chars().count())) * 4) / 5 + 2
                );
                let txt_eq_ruby = ele.txt == ele.ruby;
                rsx! {
                    div {
                        input {
                            key: "wd-ele-{i+1000}",
                            tabindex: "{i+1000}",
                            style: "{width}",
                            value: ele.proto.to_string(),
                            disabled: {
                                let txt = &word_define.read().word.elements[i].txt;
                                txt.len() == 0 || txt.chars().any(|c| StringUtil::is_kanji(c) == false)
                            },
                            onchange: move |evt| {
                                word_define.write().word.elements[i].proto = evt
                                    .value()
                                    .chars()
                                    .filter(|c| StringUtil::is_kana(*c))
                                    .collect::<String>();
                            }
                        }
                    }
                    div {
                        input {
                            key: "wd-ele-{i+2000}",
                            tabindex: "{i+2000}",
                            style: "{width}",
                            value: ele.ruby.to_string(),
                            onchange: move |evt| {
                                word_define.write().word.elements[i].ruby = evt
                                    .value()
                                    .chars()
                                    .filter(|c| StringUtil::is_kana(*c))
                                    .collect::<String>();
                            }
                        }
                    }
                    div {
                        input {
                            key: "wd-ele-{i+3000}",
                            tabindex: "{i+3000}",
                            style: "{width}",
                            value: ele.txt.to_string(),
                            onchange: move |evt| {
                                word_define.write().word.elements[i].txt = evt.value().to_string();
                            }
                        }
                    }
                }
            })
            .chain(iter::once(rsx! {
                div {
                    input { style: "width:2rem", disabled: true }
                }
                div {
                    input { style: "width:2rem", disabled: true }
                }
                div {
                    input {
                        style: "width:2rem",
                        onchange: move |evt| {
                            let word_elements = &mut word_define.write().word.elements;
                            let txt = evt.value().to_string();
                            word_elements
                                .push(WordElement {
                                    txt,
                                    ruby: String::new(),
                                    proto: String::new(),
                                });
                        }
                    }
                }
            })))
        .map(|it| {
            rsx! {
                div { {it} }
            }
        });

    rsx! {
        div {
            div { style: "display:flex;", {word_elements_editor} }
        }
        div {
            Tones { word_define }
        }
    }
}
