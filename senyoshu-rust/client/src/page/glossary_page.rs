use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use dioxus::core_macro::Props;
use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;
use itertools::{Either, Itertools};
use serde::{Deserialize, Serialize};

use senyoshu_common::embed::Asset;
use senyoshu_common::glossary::words::WORD_LIST_BY_NAME;
use senyoshu_common::types::learn::knowledge::{Knowledge, KnowledgeType};
use senyoshu_common::types::learn::learn_knowledge_history::LearnKnowledgeHistory;
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordEntry;
use senyoshu_common::util::string_util::StringUtil;

use crate::components::lazy_list::LazyList;
use crate::components::search::SearchComponent;
use crate::components::word_preview::WordPreview;
use crate::refresh_app;
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::dictionary::Dic;
use crate::storage::dictionary::DIC;
use crate::storage::use_storage::GlobalSignalStorage;
use crate::storage::workbook::WorkBook;
use crate::text::TEXT;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GlossaryPageProps {
    pub filter: GlossaryFilter,
    pub order: Order,
}

impl Display for GlossaryFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_qs::to_string(self).unwrap())
    }
}

impl FromStr for GlossaryFilter {
    type Err = serde_qs::Error;
    fn from_str(query: &str) -> Result<Self, Self::Err> {
        serde_qs::from_str::<GlossaryFilter>(query)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GlossaryFilter {
    pub kw: Option<String>,
    pub set: Option<String>,
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_qs::to_string(self).unwrap())
    }
}

impl FromStr for Order {
    type Err = serde_qs::Error;
    fn from_str(query: &str) -> Result<Self, Self::Err> {
        serde_qs::from_str::<Order>(query)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct Order {
    pub desc: Option<bool>,
    pub order_by: Option<OrderBy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum OrderBy {
    Txt,
    Kana,
    Time,
}

static SELECTED: GlobalSignalStorage<HashSet<WordIdentity>> =
    GlobalSignalStorage::session("GlossaryPageSelectedSessionStorage:", || {
        HashSet::<WordIdentity>::new()
    });

pub fn GlossaryPage(props: GlossaryPageProps) -> Element {
    let (filter_newest, order_newest) = (props.filter, props.order);

    let mut filter = use_signal(|| filter_newest.to_owned());
    let change = { *filter.read() != filter_newest };
    if change {
        *filter.write() = filter_newest;
    }
    let mut order = use_signal(|| order_newest);
    let change = { *order.read() != order_newest };
    if change {
        *order.write() = order_newest;
    }

    let mut refresh = use_signal(|| false);
    if *refresh.read() {
        refresh.set(false);
        return None;
    }

    let work_book = use_signal(|| WorkBook::get());

    let word_list = use_memo(move || {
        let dic = DIC.read();
        let filter = filter.read().to_owned();
        let set_file = filter
            .set
            .map(|set| Asset::get(format!("word/{set}.txt").as_str()).unwrap());

        let set_file_data = set_file.as_ref().map(|file| file.data.as_ref());

        let tmp_iter =
            if let Some(list) = set_file_data.map(|data| std::str::from_utf8(data).unwrap()) {
                Either::Left(
                    list.split("\n")
                        .map(|txt| dic.txt_map.get(txt))
                        .filter_map(|it| it)
                        .map(|words| words.into_iter())
                        .flatten()
                        .map(|wid| Some((*wid, dic.get(wid)?)))
                        .filter_map(|it| it),
                )
            } else {
                Either::Right(dic.iter().map(|(wid, wd)| (*wid, wd)))
            };

        let mut word_set = if let Some(kw) = filter.kw {
            Either::Left(if kw.chars().any(|c| StringUtil::is_kanji(c)) {
                Either::Left(tmp_iter.filter(move |(_, wd)| wd.word.get_txt().contains(&kw)))
            } else {
                Either::Right(tmp_iter.filter(move |(_, wd)| {
                    wd.word
                        .get_katakana()
                        .contains(&StringUtil::ruby_to_katakana(&kw))
                }))
            })
        } else {
            Either::Right(tmp_iter)
        }
        .map(|(wid, wd)| WordEntry {
            id: wid,
            word_define: wd.to_owned(),
        })
        .collect_vec();
        word_set.sort_by_cached_key(|we| we.word_define.word.get_txt());
        word_set
    });

    let is_in_plan = move |word_entry: &WordEntry| {
        let work_book = work_book.read();
        work_book
            .history
            .get(&Knowledge {
                knowledge_type: KnowledgeType::Txt,
                key: word_entry.word_define.word.get_txt(),
            })
            .map(|knowledge| knowledge.freeze_time.is_none())
            .unwrap_or(false)
            && work_book
                .history
                .get(&Knowledge {
                    knowledge_type: KnowledgeType::Kana,
                    key: word_entry.word_define.word.get_katakana(),
                })
                .map(|knowledge| knowledge.freeze_time.is_none())
                .unwrap_or(false)
    };

    let estimate_item_count = use_memo(move || word_list.read().len());

    let get_word_list_node = move |idx: usize| {
        let selected_ro = SELECTED.read();
        let word_list_ro = word_list.read();
        word_list_ro.get(idx)
            .map(|word_entry| {
                let in_plan = is_in_plan(word_entry);
                let wid = word_entry.id;
                let checked = in_plan || selected_ro.get(&wid).is_some();
                rsx! {
                    div { key: "wid-{wid}",
                        label { style: "display:flex;flex-direction:row;margin-top:auto;margin-bottom:auto",
                            input {
                                style: "margin-top:auto;margin-bottom:auto",
                                r#type: "checkbox",
                                checked,
                                disabled: in_plan,
                                onclick: move |_| {
                                    let mut selected = SELECTED.write();
                                    if selected.contains(&wid) {
                                        selected.remove(&wid);
                                    } else {
                                        selected.insert(wid.to_owned());
                                    }
                                }
                            }
                            div { style: "flex:1",
                                WordPreview { word_entry: word_entry.to_owned() }
                            }
                        }
                    }
                }
            })
    };

    let items = Vec::from([
        Vec::from([
            MenuItem {
                label: TEXT.read().glossary_page_selector_select_all,
                onclick: EventHandler::new(move |_| {
                    let mut selected = SELECTED.write();
                    for we in word_list.peek().iter() {
                        let in_plan = is_in_plan(we);
                        if !in_plan {
                            selected.insert(we.id);
                        }
                    }
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: TEXT.read().glossary_page_selector_clear,
                onclick: EventHandler::new(move |_| {
                    let mut selected = SELECTED.write();
                    for we in word_list.peek().iter() {
                        let in_plan = is_in_plan(we);
                        if !in_plan {
                            selected.remove(&we.id);
                        }
                    }
                    refresh.set(true);
                }),
                ..Default::default()
            },
        ]),
        Vec::from([MenuItem {
            label: TEXT.read().glossary_page_action_add_to_plan,
            onclick: EventHandler::new(move |_| {
                learn(&DIC.peek(), &SELECTED.read());
                SELECTED.reset();
                refresh.set(true);
            }),
            ..Default::default()
        }]),
    ]);

    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_menu_items(items);
    TOP_NAVIGATION.set_content(Some(rsx! {
        SearchComponent { kw: (filter.read().kw).as_ref().cloned().unwrap_or_default() }
    }));

    let mounted = use_mounted();
    let height = use_size(mounted).height();

    rsx! {
        div {
            style: "height:100%;",
            onmounted: move |event| mounted.onmounted(event),
            LazyList {
                container_height: height,
                make_item: get_word_list_node,
                item_height: 28.0,
                estimate_item_count: estimate_item_count()
            }
        }
    }
}

fn learn(dic: &Dic, selected: &HashSet<WordIdentity>) {
    WorkBook::with_mut(|work_book| {
        for wid in selected {
            if let Some(word_entry) = dic.get(wid) {
                work_book.append_record(
                    Knowledge {
                        knowledge_type: KnowledgeType::Txt,
                        key: word_entry.word.get_txt(),
                    },
                    [],
                );
                work_book.append_record(
                    Knowledge {
                        knowledge_type: KnowledgeType::Kana,
                        key: word_entry.word.get_katakana(),
                    },
                    [],
                );
            }
        }
    });
}
