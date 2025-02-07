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
pub struct KanaData {
    pub kana: String,
    pub words: SmallVec<[WordEntry; 1]>,
}

impl Into<Knowledge> for &KanaData {
    fn into(self) -> Knowledge {
        Knowledge {
            knowledge_type: KnowledgeType::Kana,
            key: self.kana.to_owned(),
        }
    }
}

pub(super) fn LearnKana(props: LearnKnowledgeProps<KanaData>) -> Element {
    let words = props.data.words.to_owned();
    let kana = use_signal(|| (&props.data.kana).to_string());

    let mut stage = use_signal(|| 0usize);
    let played = use_signal(|| false);

    let _ = use_coroutine(|_rx: UnboundedReceiver<()>| {
        let t = words
            .iter()
            .map(|it| it.word_define.word.tones.iter().next())
            .collect::<SmallVec<[Option<Tone>; 6]>>();

        let kana = kana.to_owned();
        let mut played = played.to_owned();
        async move {
            for tone in t.iter() {
                play(kana.read().to_string(), tone.to_owned()).await;
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
            ViewerNode { word_define: it.word_define.to_owned() }
        }
    });

    let on_select = move |evt| {
        let knowledge: Knowledge = (&props.data).into();
        WorkBook::add_record(knowledge.to_owned(), SmallVec::from([evt]));

        props.on_ended.call(());
    };

    let min_height =
        (*WINDOW_HEIGHT.read()).unwrap_or(800f64).floor() as usize - BOTTOM_NAVIGATION_HEIGHT;

    let content = match *stage.read() {
        0 => {
            rsx! {
                div { style: "font-size:2rem", "{kana}" }
            }
        }
        1 => {
            rsx! {
                {word_nodes}
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

    rsx! {
        div {
            style: "display:flex;flex-direction:column;min-height:{min_height}px",
            onclick: move |_| {
                if *played.peek() && *stage.peek() < 2 {
                    *stage.write() += 1;
                }
            },
            {content}
        }
    }
}

#[component]
pub fn PreviewKana(data: KanaData) -> Element {
    let word = &data.words;
    let kana = &data.kana;

    let words = word.iter().map(|word_entry| {
        let txt = word_entry.word_define.word.get_txt();
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
                    {mean}
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
                        {txt}
                    }

                    Sound { text: ruby,
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
            span { style: "width:5rem;margin-top:auto;margin-bottom:auto", "{kana}" }
            span { style: "flex:1", {words} }
        }
    }
}
