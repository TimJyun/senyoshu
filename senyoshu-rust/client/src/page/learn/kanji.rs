use std::time::Duration;

use async_std::task::sleep;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use tracing::debug;

use senyoshu_common::types::kanji_alias::Kanji;
use senyoshu_common::types::kanji_detail::KanjiReference;
use senyoshu_common::types::learn::knowledge::{Knowledge, KnowledgeType};
use senyoshu_common::types::learn::plan::Plan;
use senyoshu_common::util::string_util::StringUtil;

use crate::components::memory_buttons::MemoryButtons;
use crate::components::sound::Sound;
use crate::page::kanji_page::kanji_page_node;
use crate::page::learn::LearnKnowledgeProps;
use crate::router::AppRoute;
use crate::singleton::bottom_navigation::BOTTOM_NAVIGATION_HEIGHT;
use crate::storage::workbook::WorkBook;
use crate::voice::play;
use crate::window::WINDOW_HEIGHT;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct KanjiData {
    pub kanji: Kanji,
    pub references: KanjiReference,
}

impl Into<Knowledge> for &KanjiData {
    fn into(self) -> Knowledge {
        Knowledge {
            knowledge_type: KnowledgeType::Kanji,
            key: self.kanji.to_string(),
        }
    }
}

pub(super) fn LearnKanji(props: LearnKnowledgeProps<KanjiData>) -> Element {
    let kanji = use_signal(|| props.data.kanji);
    let kanji_reference = props.data.references.to_owned();

    let mut stage = use_signal(|| 0usize);
    let played = use_signal(|| false);
    let sounds = use_signal(|| kanji_reference.get_rubies().collect_vec());

    let _ = use_coroutine(|_rx: UnboundedReceiver<()>| {
        let mut stage = stage.to_owned();
        let mut played = played.to_owned();
        let sounds = sounds.to_owned();
        async move {
            while *stage.read() != 1 {
                sleep(Duration::from_millis(10)).await;
            }
            let sounds = sounds.read();
            let len = sounds.len();
            for kana in sounds.iter() {
                play(
                    StringUtil::ruby_to_katakana(kana.to_string().as_str()),
                    None,
                )
                .await;
            }
            debug!("stage = 1 and played = {len}");
            *stage.write() = 2;
            played.set(true);
        }
    });

    let on_select = {
        let knowledge: Knowledge = (&props.data).into();
        move |evt| {
            WorkBook::add_record(knowledge.to_owned(), SmallVec::from([evt]));

            props.on_ended.call(());
        }
    };

    let content = match *stage.read() {
        0 => {
            rsx! {
                div { style: "font-size:2rem", "{kanji}" }
            }
        }
        1 => kanji_page_node(**kanji.read(), props.data.references.to_owned(), true),
        2 => {
            rsx! {
                {kanji_page_node( **kanji.read(),props.data.references.to_owned(),true)},
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
pub fn PreviewKanji(data: KanjiData) -> Element {
    let kanji = data.kanji;
    let kanji_reference = (&data.references).to_owned();

    let rubies = kanji_reference.get_rubies().map(|ruby| {
        rsx! {
            span { style: "min-width:8rem;display:inline-block",
                Sound { text: ruby.to_string(), {ruby} }
            }
        }
    });

    rsx! {
        div { style: "display:flex",
            span { style: "width:5rem;margin-top:auto;margin-bottom:auto",
                Link {
                    onclick: |evt: Event<MouseData>| {
                        evt.stop_propagation();
                    },
                    to: AppRoute::KanjiPage { kanji },
                    "{kanji}"
                }
            }
            span { style: "flex:1", {rubies} }
        }
    }
}
