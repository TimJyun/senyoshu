use std::time::Duration;

use async_std::task::sleep;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use smallvec::SmallVec;

use senyoshu_common::types::learn::knowledge::{Knowledge, KnowledgeType};
use senyoshu_common::types::learn::plan::Plan;
use senyoshu_common::types::word::tones::Tone;
use senyoshu_common::types::word::word_entry::WordEntry;

use crate::components::memory_buttons::MemoryButtons;
use crate::components::sound::Sound;
use crate::components::viewer::ViewerNode;
use crate::components::word_node::WordNode;
use crate::imgs::AUDIO_IMG;
use crate::page::learn::LearnKnowledgeProps;
use crate::router::AppRoute;
use crate::singleton::bottom_navigation::BOTTOM_NAVIGATION_HEIGHT;
use crate::storage::workbook::WorkBook;
use crate::voice::play;
use crate::window::WINDOW_HEIGHT;

#[derive(Props, PartialEq, Clone)]
pub struct TxtData {
    pub txt: String,
    pub words: SmallVec<[WordEntry; 1]>,
}

impl Into<Knowledge> for &TxtData {
    fn into(self) -> Knowledge {
        Knowledge {
            knowledge_type: KnowledgeType::Txt,
            key: self.txt.to_owned(),
        }
    }
}

pub fn LearnTxt(props: LearnKnowledgeProps<TxtData>) -> Element {
    let words = props.data.words.to_owned();
    let txt = use_signal(|| props.data.txt.to_string());
    let mut stage = use_signal(|| 0usize);
    let played = use_signal(|| false);
    let sounds = use_signal(|| {
        words
            .iter()
            .map(|it| {
                (
                    it.word_define.word.get_katakana(),
                    it.word_define.word.tones.iter().next(),
                )
            })
            .collect::<SmallVec<[(String, Option<Tone>); 1]>>()
    });

    let _ = use_coroutine(|_rx: UnboundedReceiver<()>| {
        let stage = stage.to_owned();
        let mut played = played.to_owned();
        let sounds = sounds.to_owned();
        async move {
            while *stage.read() != 1 {
                sleep(Duration::from_millis(10)).await;
            }

            for (kana, tone) in sounds.read().iter() {
                play(kana.to_string(), tone.to_owned()).await;
            }
            played.set(true);
        }
    });

    let word_nodes = words.iter().map(|it| {
        rsx! {
            div {
                WordNode { word: it.word_define.word.to_owned(), font_size: 2. }
            }
        }
    });

    let word_entry_nodes = words.iter().map(|it| {
        rsx! {
            ViewerNode { word_define: it.word_define.to_owned(), align_left: true }
        }
    });

    let on_select = move |evt| {
        let knowledge: Knowledge = (&props.data).into();
        WorkBook::add_record(knowledge.to_owned(), [evt]);
        props.on_ended.call(());
    };

    let content = match *stage.read() {
        0 => {
            rsx! {
                div { style: "font-size:2rem", "{txt}" }
            }
        }
        1 => {
            rsx! {
                { word_nodes}
            }
        }
        2 => {
            rsx! {
                {word_entry_nodes},
                div {
                    MemoryButtons { on_select, plan: props.plan }
                }
            }
        }
        _ => None,
    };

    let min_height =
        (*WINDOW_HEIGHT.read()).unwrap_or(800f64).floor() as usize - BOTTOM_NAVIGATION_HEIGHT;

    rsx! {
        div {
            style: "display:flex;flex-direction:column;min-height:{min_height}px",
            onclick: move |_| {
                if *stage.read() == 0 || (*stage.read() == 1 && *played.read()) {
                    *stage.write() += 1;
                }
            },
            {content}
        }
    }
}

#[component]
pub fn PreviewTxt(data: TxtData) -> Element {
    let word = &data.words;
    let txt = &data.txt;

    let words = word.iter().map(|word_entry| {
        let ruby = word_entry.word_define.word.get_ruby();
        let loan = word_entry.word_define.loan.as_ref().map(|loan| {
            rsx! {
                div {

                    "("
                    {loan.language.to_string()},
                    ")"
                    {loan.source_word.to_string()}
                }
            }
        });
        let means = word_entry.word_define.means.iter().map(|mean| {
            let pos = mean.parts_of_speech.to_string();
            let mean = mean.explanation.zh.to_string();
            rsx! {
                div {
                    {pos},
                    { mean}
                }
            }
        });
        rsx! {
            div { style: "display:flex;flex-direction:row",
                span { style: "width:6rem;margin-top:auto;margin-bottom:auto",
                    Link {
                        to: AppRoute::WordPage {
                            wid: word_entry.id,
                        },
                        {ruby}
                    }
                    Sound { text: word_entry.word_define.word.get_ruby(),
                        img {
                            src: AUDIO_IMG,
                            style: "width:1.25rem;cursor:pointer;"
                        }
                    }
                }
                span { style: "flex:1",
                    {loan},
                    {means}
                }
            }
        }
    });

    rsx! {
        div { style: "display:flex;flex-direction:row",
            span { style: "width:5rem;margin-top:auto;margin-bottom:auto", "{txt}" }
            span { style: "flex:1", {words} }
        }
    }
}
