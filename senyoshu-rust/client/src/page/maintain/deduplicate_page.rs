use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use senyoshu_common::types::word::word_entry::WordEntry;

use crate::components::lazy_list::LazyList;
use crate::components::word_preview::WordPreview;
use crate::page::learn::preview_knowledge::PreviewKnowledge;
use crate::page::learn::txt::TxtData;
use crate::page::learn::KnowledgeData;
use crate::router::AppRoute;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::dictionary::DIC;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct DeduplicatePageProps {}

pub fn DeduplicatePage(props: DeduplicatePageProps) -> Element {
    let groups = use_memo(move || {
        let dic = DIC.read();
        dic.kana_map
            .inner()
            .iter()
            .filter(|item| item.1.len() > 1)
            .map(|item| {
                let groups = item
                    .1
                    .iter()
                    .filter_map(|wid| {
                        Some(WordEntry {
                            id: *wid,
                            word_define: dic.get(wid)?.to_owned(),
                        })
                    })
                    .into_group_map_by(|we| we.word_define.word.get_txt());
                groups
                    .into_iter()
                    .filter(|(_, wes)| wes.len() > 1)
                    .map(|(key, word)| word)
            })
            .flatten()
            .collect_vec()
    });

    let nav = use_navigator();

    let make_item = move |idx| {
        let group = groups.get(idx)?;
        let words = group.iter().cloned().map(|word_entry| {
            rsx! {
                div {
                    WordPreview { word_entry }
                }
            }
        });

        let wid = group[0].id;
        let wid2 = group[1].id;

        Some(rsx! {
            div {
                style: "border-bottom-width:1px;border-bottom-style: dotted",
                onclick: move |_| {
                    nav.push(AppRoute::DiffPage { wid, wid2 });
                },
                {words}
            }
        })
    };

    let estimate_item_count = groups.len();
    let mounted = use_mounted();
    let height = use_size(mounted).height();

    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_content(rsx! { "{estimate_item_count}" }.into());
    rsx! {
        div {
            style: "height:100%;",
            onmounted: move |event| mounted.onmounted(event),
            LazyList { container_height: height, make_item, item_height: 48.0, estimate_item_count }
        }
    }
}
