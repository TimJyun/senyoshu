use std::collections::HashSet;
use std::rc::Rc;
use std::string::ToString;

use chrono::{FixedOffset, Local};
use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tracing::debug;

use senyoshu_common::types::learn::knowledge::{Knowledge, KnowledgeType};

use crate::components::lazy_list::LazyList;
use crate::imgs::SNOWFLAKE_IMG;
use crate::page::learn::preview_knowledge::PreviewKnowledge;
use crate::page::learn::KnowledgeData;
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::dictionary::DIC;
use crate::storage::use_storage::GlobalSignalStorage;
use crate::storage::workbook::WorkBook;
use crate::text::TEXT;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KnowledgePageProps {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct KnowledgePageFilter {
    show_freezed: bool,
    show_unfreezed: bool,
    show_kanji: bool,
    show_txt: bool,
    show_kana: bool,
}

static FILTER: GlobalSignalStorage<KnowledgePageFilter> =
    GlobalSignalStorage::session("KnowledgePageFilterSessionStorage", || {
        KnowledgePageFilter {
            show_freezed: false,
            show_unfreezed: true,
            show_kanji: true,
            show_txt: true,
            show_kana: true,
        }
    });

static SELECTED: GlobalSignalStorage<HashSet<Knowledge>> =
    GlobalSignalStorage::session("KnowledgePageSelectedSessionStorage", || {
        HashSet::<Knowledge>::new()
    });

pub fn KnowledgePage() -> Element {
    let mut refresh = use_signal(|| false);
    if *refresh.read() {
        refresh.set(false);
        return None;
    }

    let dic = DIC.read();

    let work_book = WorkBook::get();

    let filter_ro = FILTER.read();
    let mut knowledge_vec = work_book
        .history
        .iter()
        .filter(|(k, h)| {
            (filter_ro.show_freezed || h.freeze_time.is_none())
                && (filter_ro.show_unfreezed || h.freeze_time.is_some())
                && (filter_ro.show_kanji || k.knowledge_type != KnowledgeType::Kanji)
                && (filter_ro.show_txt || k.knowledge_type != KnowledgeType::Txt)
                && (filter_ro.show_kana || k.knowledge_type != KnowledgeType::Kana)
        })
        .filter_map(|(k, _)| {
            let data = KnowledgeData::get_learn_data(k, &dic)?;
            Some((k.to_owned(), data))
        })
        .collect_vec();

    knowledge_vec.sort_by(|(k1, _), (k2, _)| k1.key.cmp(&k2.key));

    let knowledge_vec_rc = Rc::new(knowledge_vec);

    let knowledge_vec_rc_list = knowledge_vec_rc.to_owned();
    let selected_ro = SELECTED.read();
    let knowledge_list = knowledge_vec_rc_list.iter()
        .map(|(k, data)| {
            let kt = match &k.knowledge_type {
                KnowledgeType::Kanji => { "漢字" }
                KnowledgeType::Txt => { "表記" }
                KnowledgeType::Kana => { "発音" }
            };
            let checked = selected_ro.contains(k);

            let is_freezed = work_book.history.get(k).map(|it| { it.freeze_time.is_some() }).unwrap_or(true);
            let freezed_state_style = if is_freezed {
                "height:16px;width:16px;"
            } else {
                "height:16px;width:16px;visibility:hidden;"
            };

            rsx! {
                div { style: "display:flex;margin: auto;border-bottom-width:1px;border-bottom-style: dotted",
                    input {
                        style: "margin-top:auto;margin-bottom:auto",
                        name: "{k}",
                        r#type: "checkbox",
                        checked,
                        onclick: {
                            let k = k.to_owned();
                            move |_| {
                                let mut selected = SELECTED.write();
                                if let Some(_) = selected.get(&k) {
                                    selected.remove(&k);
                                } else {
                                    selected.insert(k.to_owned());
                                };
                            }
                        }
                    }

                    img {
                        style: "{freezed_state_style};margin-top:auto;margin-bottom:auto",
                        src: SNOWFLAKE_IMG
                    }

                    span { style: "margin-left:8px;margin-right:8px;margin-top:auto;margin-bottom:auto",
                        "{kt}"
                    }
                    div { style: "flex:1;margin-top:auto;margin-bottom:auto",
                        PreviewKnowledge { data: data.to_owned() }
                    }
                }
            }
        });

    let items = Vec::from([
        Vec::from([
            MenuItem {
                label: if filter_ro.show_freezed {
                    TEXT.read().knowledge_page_filter_hide_freezed
                } else {
                    TEXT.read().knowledge_page_filter_show_freezed
                },
                onclick: EventHandler::new(move |_| {
                    let mut filter = FILTER.write();
                    filter.show_freezed = !filter.show_freezed;
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: if filter_ro.show_unfreezed {
                    TEXT.read().knowledge_page_filter_hide_unfreezed
                } else {
                    TEXT.read().knowledge_page_filter_show_unfreezed
                },
                onclick: EventHandler::new(move |_| {
                    let mut filter = FILTER.write();
                    filter.show_unfreezed = !filter.show_unfreezed;
                    refresh.set(true);
                }),
                ..Default::default()
            },
        ]),
        Vec::from([
            MenuItem {
                label: if filter_ro.show_kanji {
                    TEXT.read().knowledge_page_filter_hide_kanji
                } else {
                    TEXT.read().knowledge_page_filter_show_kanji
                },
                onclick: EventHandler::new(move |_| {
                    let mut filter = FILTER.write();
                    filter.show_kanji = !filter.show_kanji;
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: if filter_ro.show_txt {
                    TEXT.read().knowledge_page_filter_hide_txt
                } else {
                    TEXT.read().knowledge_page_filter_show_txt
                },
                onclick: EventHandler::new(move |_| {
                    let mut filter = FILTER.write();
                    filter.show_txt = !filter.show_txt;
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: if filter_ro.show_kana {
                    TEXT.read().knowledge_page_filter_hide_kana
                } else {
                    TEXT.read().knowledge_page_filter_show_kana
                },
                onclick: EventHandler::new(move |_| {
                    let mut filter = FILTER.write();
                    filter.show_kana = !filter.show_kana;
                    refresh.set(true);
                }),
                ..Default::default()
            },
        ]),
        Vec::from([
            MenuItem {
                label: TEXT.read().knowledge_page_selector_reload,
                onclick: EventHandler::new(move |_| {
                    debug!("reload");
                    SELECTED.reset();
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: TEXT.read().knowledge_page_selector_select_all,
                onclick: EventHandler::new({
                    let knowledge_vec_rc = knowledge_vec_rc.to_owned();
                    move |_| {
                        debug!("select all");
                        let mut selected = SELECTED.write();
                        for (k, _) in knowledge_vec_rc.iter() {
                            selected.insert((*k).to_owned());
                        }
                        refresh.set(true);
                    }
                }),
                ..Default::default()
            },
            MenuItem {
                label: TEXT.read().knowledge_page_selector_clear,
                onclick: EventHandler::new({
                    let knowledge_vec_rc = knowledge_vec_rc.to_owned();
                    move |_| {
                        debug!("clear");
                        let mut selected = SELECTED.write();
                        for (k, _) in knowledge_vec_rc.iter() {
                            selected.remove(k);
                        }
                        refresh.set(true);
                    }
                }),
                ..Default::default()
            },
        ]),
        Vec::from([
            MenuItem {
                label: TEXT.read().knowledge_page_action_freeze,
                onclick: EventHandler::new(move |_| {
                    debug!("freeze");
                    let now: chrono::DateTime<FixedOffset> = Local::now().into();
                    WorkBook::with_mut(|work_book| {
                        for k in SELECTED.read().iter() {
                            if let Some(h) = work_book.history.get_mut(&k) {
                                h.freeze_time = Some(now.to_owned());
                            }
                        }
                    });
                    SELECTED.reset();
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: TEXT.read().knowledge_page_action_unfreeze,
                onclick: EventHandler::new(move |_| {
                    debug!("unfreeze");
                    WorkBook::with_mut(|work_book| {
                        for k in SELECTED.read().iter() {
                            if let Some(h) = work_book.history.get_mut(&k) {
                                h.freeze_time = None;
                            }
                        }
                    });
                    SELECTED.reset();
                    refresh.set(true);
                }),
                ..Default::default()
            },
        ]),
    ]);

    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_menu_items(items);

    let knowledge_list = knowledge_list.collect_vec();
    let estimate_item_count = knowledge_list.len();
    let mounted = use_mounted();
    let height = use_size(mounted).height();
    rsx! {
        div {
            style: "height:100%;",
            onmounted: move |event| mounted.onmounted(event),
            LazyList {
                container_height: height,
                make_item: move |idx: usize| { knowledge_list.get(idx).cloned() },
                item_height: 24.0,
                estimate_item_count
            }
        }
    }
}
