use std::collections::HashMap;
use std::rc::Rc;

use async_std::io::ReadExt;
use chrono::{FixedOffset, Local};
use dioxus::core_macro::Props;
use dioxus::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use senyoshu_common::glossary::kanjis::KANJI_LIST_BY_NAME;
use senyoshu_common::types::kanji_alias::Kanji;
use senyoshu_common::types::learn::knowledge::{Knowledge, KnowledgeType};
use senyoshu_common::types::learn::learn_knowledge_history::LearnKnowledgeHistory;
use senyoshu_common::util::string_util::StringUtil;

use crate::page::learn::kanji::{KanjiData, PreviewKanji};
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::dictionary::DIC;
use crate::storage::use_storage::GlobalSignalStorage;
use crate::storage::workbook::WorkBook;
use crate::text::TEXT;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct KanjiListPageProps {
    name: String,
}

static SELECTED: GlobalSignalStorage<HashMap<char, bool>> =
    GlobalSignalStorage::session("KanjiListPageSelectedSessionStorage:", move || {
        let mut s = HashMap::<char, bool>::new();

        for (kanji, checked) in WorkBook::get().history.iter().filter_map(
            |(k, LearnKnowledgeHistory { freeze_time, .. })| {
                if let KnowledgeType::Kanji = k.knowledge_type {
                    Some((k.key.chars().next()?, freeze_time.is_none()))
                } else {
                    None
                }
            },
        ) {
            s.insert(kanji, checked);
        }
        s
    });

pub fn KanjiListPage(props: KanjiListPageProps) -> Element {
    let mut refresh = use_signal(|| false);
    if *refresh.read() {
        refresh.set(false);
        return None;
    }

    let dic = DIC.read();

    let list = KANJI_LIST_BY_NAME
        .iter()
        .find(|(n, _)| *n == props.name.as_str())
        .map(|(_n, list)| list.chars().collect_vec())
        .unwrap_or({
            let kanjis_in_dic = dic
                .char_map
                .keys()
                .filter(|c| StringUtil::is_kanji(**c))
                .cloned();
            let joyo_kanjis = KANJI_LIST_BY_NAME[0].0.chars();
            let mut kanjis = kanjis_in_dic.chain(joyo_kanjis).collect_vec();
            kanjis.sort();
            kanjis.dedup();
            kanjis
        });
    let list = Rc::new(list);

    let selected_ro = SELECTED.read();
    let kanji_list = list
        .iter()
        .map(|c| Kanji::try_from(*c))
        .map(|c| {
            let c = c.ok()?;
            Some((c, dic.query_kanji(c)?))
        })
        .filter_map(|it| it)
        .map(|(kanji, kanji_reference)| {
            let checked = selected_ro.get(&kanji).cloned().unwrap_or(false);

            let onclick = move |_| {
                let mut selected = SELECTED.write();
                let e = selected.entry(*kanji).or_insert(false);
                *e = !*e;
            };

            // 这里/3只是个大概，在宽屏设备上会有更好的值
            let intrinsic_size = (kanji_reference.recorded_onyomi.len()
                + kanji_reference.recorded_kunyomi.len()
                + kanji_reference.not_recorded.len()
                + kanji_reference.special.len())
                / 3;
            rsx! {
                div {
                    key: "kanji-{*kanji}",
                    style: "display:flex;flex-direction:row;margin-top:auto;margin-bottom:auto;",
                    onclick: onclick,
                    input { r#type: "checkbox", checked }
                    div { style: "flex:1",
                        PreviewKanji {
                            data: KanjiData {
                                kanji,
                                references: kanji_reference,
                            }
                        }
                    }
                }
            }
        });

    let list_all = Rc::clone(&list);
    let list_none = Rc::clone(&list_all);

    let items = Vec::from([
        Vec::from([
            MenuItem {
                label: TEXT.read().kanji_list_page_selector_select_all,
                onclick: EventHandler::new(move |_| {
                    let mut selected = SELECTED.write();
                    list_all
                        .iter()
                        .filter_map(|c| Kanji::try_from(*c).ok())
                        .for_each(|kanji| {
                            selected.insert(*kanji, true);
                        });
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: TEXT.read().kanji_list_page_selector_clear,
                onclick: EventHandler::new(move |_| {
                    let mut selected = SELECTED.write();
                    list_none
                        .iter()
                        .filter_map(|c| Kanji::try_from(*c).ok())
                        .for_each(|kanji| {
                            selected.insert(*kanji, false);
                        });
                    refresh.set(true);
                }),
                ..Default::default()
            },
        ]),
        Vec::from([
            MenuItem {
                label: TEXT.read().kanji_list_page_action_reload,
                onclick: EventHandler::new(move |_| {
                    SELECTED.reset();
                    refresh.set(true);
                }),
                ..Default::default()
            },
            MenuItem {
                label: TEXT.read().kanji_list_page_action_save_plan,
                onclick: EventHandler::new(move |_| {
                    save(&SELECTED.read());
                    SELECTED.reset();
                    refresh.set(true);
                }),
                ..Default::default()
            },
        ]),
    ]);

    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_menu_items(items);

    rsx! {
        {kanji_list}
    }
}

fn save(selected: &HashMap<char, bool>) {
    let now: chrono::DateTime<FixedOffset> = Local::now().into();
    WorkBook::with_mut(|work_book| {
        for (kanji, in_plan) in selected {
            let know = Knowledge {
                knowledge_type: KnowledgeType::Kanji,
                key: kanji.to_string(),
            };

            if let Some(t) = work_book.history.get_mut(&know) {
                if *in_plan {
                    t.freeze_time = None;
                } else {
                    t.freeze_time = Some(now.to_owned());
                }
                work_book.to_be_push.insert(know);
            } else {
                if *in_plan {
                    work_book.history.insert(
                        know.to_owned(),
                        LearnKnowledgeHistory {
                            history: Default::default(),
                            freeze_time: None,
                        },
                    );
                    work_book.to_be_push.insert(know);
                }
            }
        }
    });
}
