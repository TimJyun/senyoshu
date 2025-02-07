use std::iter;

use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use senyoshu_common::glossary::kanjis::KANJI_LIST_BY_NAME;
use senyoshu_common::glossary::words::WORD_LIST_BY_NAME;

use crate::components::search::SearchComponent;
use crate::page::glossary_page::{GlossaryFilter, Order};
use crate::router::AppRoute;
use crate::singleton::top_navigation::TOP_NAVIGATION;

pub fn CollectionPage() -> Element {
    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_no_back_button(true);
    TOP_NAVIGATION.set_content(Some(rsx! {
        SearchComponent {}
    }));

    rsx! {
        Kanjis {}
        Words {}
    }
}

pub fn Kanjis() -> Element {
    let lists = iter::once("全部漢字")
        .chain(KANJI_LIST_BY_NAME.iter().map(|(label, _)| *label))
        .map(|label| {
            rsx! {
                div {
                    Link {
                        to: AppRoute::KanjiListPage {
                            name: label.to_string(),
                        },
                        "{label}"
                    }
                }
            }
        });

    rsx! {
        fieldset {
            display: "block",
            margin: "0 auto",
            width: "360px",
            max_width: "84%",
            legend { margin: "0 auto", "汉字" }
            {lists}
        }
    }
}

pub fn Words() -> Element {
    let lists = iter::once(rsx! {
        div {
            Link {
                to: AppRoute::GlossaryPage {
                    filter: Default::default(),
                    order: Default::default(),
                },
                "全部詞彙"
            }
        }
    })
    .chain(
        WORD_LIST_BY_NAME
            .into_iter()
            .enumerate()
            .map(|(idx, word_list)| {
                rsx! {
                    div {
                        Link {
                            to: AppRoute::GlossaryPage {
                                filter: GlossaryFilter {
                                    set: Some(word_list.to_string()),
                                    ..Default::default()
                                },
                                order: Default::default(),
                            },
                            "{word_list}"
                        }
                    }
                }
            }),
    );

    rsx! {
        fieldset {
            display: "block",
            margin: "0 auto",
            width: "360px",
            max_width: "84%",
            legend { margin: "0 auto", "詞彙" }
            {lists}
        }
    }
}
