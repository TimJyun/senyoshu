use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use gloo::console::log;
use tracing::log::debug;

use senyoshu_common::types::word::parts_of_speech::DefaultExt;
use senyoshu_common::types::word::word_entry::{LoadLanguage, Loan, WordDefine, LOAD_LANGUAGES};

use crate::components::button::Button;
use crate::components::editor::example_container::RenderEditorExampleSentenceContainer;
use crate::components::editor::mean_container::RenderMeanContainer;
use crate::components::editor::word_creator::WordCreator;
use crate::components::viewer::ViewerNode;
use crate::window::is_widescreen;

#[derive(Props, PartialEq, Clone)]
pub struct RenderEditorProps {
    pub word_define: Signal<WordDefine>,
}

pub fn RenderEditor(props: RenderEditorProps) -> Element {
    let mut word_define = props.word_define;

    let detailed_nodes = {
        let detailed = word_define.read().detailed.to_string();
        if detailed.is_empty() {
            None
        } else {
            rsx! {
                fieldset { style: "max-width: 75%;",
                    legend { "詳細" }
                    textarea {
                        style: "width:100%;resize:none;font-size: 1rem;height: 160px;",
                        value: detailed,
                        oninput: move |evt| {
                            word_define.write().detailed = evt.value();
                        }
                    }
                }
            }
        }
    };
    let mut loan = use_memo(move || word_define.read().loan.to_owned());

    let loan_nodes = {
        let options = [rsx! {
            option { selected: word_define.read().loan.is_some() }
        }]
        .into_iter()
        .chain(LOAD_LANGUAGES.map(|language| {
            let selected = language
                == word_define
                    .read()
                    .loan
                    .as_ref()
                    .map(|loan| loan.language.to_string())
                    .unwrap_or_else(String::new);
            rsx! {
                option { value: language, selected, "{language}" }
            }
        }));

        rsx! {
            "外来語："
            select {
                value: loan.read().as_ref().map(|l| l.language.to_string()).unwrap_or_else(String::new),
                onchange: move |evt| {
                    log!("select(外来語):change to  {}", evt.value());
                    let mut word_define_write = word_define.write();
                    if evt.value().is_empty() {
                        word_define_write.loan = None;
                    } else if let Some(loan) = word_define_write.loan.as_mut() {
                        loan.language = evt.value();
                    } else {
                        word_define_write.loan = Some(Loan {
                            language: evt.value(),
                            source_word: String::new(),
                        });
                    }
                },
                {options}
            }
            "語彙："
            input {
                value: loan.read().as_ref().map(|l| l.source_word.to_string()).unwrap_or_else(String::new),
                oninput: move |evt| {
                    log!("oninput(語彙):change to  {}", evt.value());
                    let mut word_define_write = word_define.write();
                    if let Some(loan) = word_define_write.loan.as_mut() {
                        loan.source_word = evt.value();
                    } else {
                        word_define_write.loan = Some(Loan {
                            language: String::new(),
                            source_word: evt.value(),
                        });
                    }
                }
            }
        }
    };

    rsx! {
        div { style: "margin:16px",
            div { style: "font-size: 1rem",
                WordCreator { word_define }
            }
            div { {loan_nodes} }
            div { style: "height:2px;width:50%;background-color: #808080;" }
            RenderMeanContainer { word_define }
            div {
                RenderEditorExampleSentenceContainer { word_define }
            }
            {detailed_nodes},
            div {
                Button {
                    custom_style: "margin:4px;",
                    onclick: move |_| {
                        word_define.write().means.push(Default::default());
                    },
                    "追加意味"
                }
                Button {
                    custom_style: "margin:4px;",
                    onclick: move |_| {
                        word_define.write().example_sentences.push(Default::default());
                    },
                    "追加範例"
                }
                Button {
                    custom_style: "margin:4px;",
                    onclick: move |_| {
                        let mut word_define_write = word_define.write();
                        word_define_write.word.elements.retain(|ele| { !ele.eq_default() });
                        word_define_write.means.retain(|mean| { !mean.eq_default() });
                        word_define_write.example_sentences.retain(|es| { !es.eq_default() });
                    },
                    "整理"
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct EditorProps {
    pub word_define: Signal<WordDefine>,
}

pub fn Editor(props: EditorProps) -> Element {
    let word_define = props.word_define;
    let mut edite_mode = use_signal(|| true);

    if is_widescreen() {
        rsx! {
            div { style: "display:flex;flex-direction:row",
                span { style: "flex:1",
                    RenderEditor { word_define }
                }
                span { style: "flex:1",
                    ViewerNode { word_define: word_define.read().to_owned() }
                }
            }
        }
    } else {
        let ui = if *edite_mode.read() {
            rsx! {
                RenderEditor { word_define }
            }
        } else {
            rsx! {
                ViewerNode { word_define: word_define.read().to_owned() }
            }
        };
        rsx! {
            div { style: "text-align:center",
                Button {
                    disabled: *edite_mode.read(),
                    onclick: move |_| {
                        edite_mode.set(true);
                    },
                    "編集(edite)"
                }
                Button {
                    disabled: *edite_mode.read() == false,
                    onclick: move |_| {
                        edite_mode.set(false);
                    },
                    "プレビュー(preview)"
                }
            }
            {ui}
        }
    }
}
