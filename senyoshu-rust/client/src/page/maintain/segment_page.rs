use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use senyoshu_common::types::json_value_template::JsonValueTemplate::String;
use senyoshu_common::types::word::word_entry::WordEntry;
use senyoshu_common::util::string_util::StringUtil;

use crate::components::lazy_list::LazyList;
use crate::components::word_preview::WordPreview;
use crate::page::learn::preview_knowledge::PreviewKnowledge;
use crate::page::learn::txt::TxtData;
use crate::page::learn::KnowledgeData;
use crate::router::AppRoute;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::dictionary::DIC;

pub fn SegmentPage() -> Element {
    let groups = use_memo(move || {
        let dic = DIC.read();
        dic.iter()
            .filter(|(wid, wd)| {
                wd.word.elements.iter().any(|ele| {
                    ele.proto.trim().is_empty()
                        && ele.txt.chars().filter(|c| StringUtil::is_kanji(*c)).count() > 1
                })
            })
            .map(|(wid, wd)| WordEntry {
                id: wid.to_owned(),
                word_define: wd.to_owned(),
            })
            .collect_vec()
    });

    let nav = use_navigator();

    let make_item = move |idx| {
        let word_entry_ref = groups.get(idx)?;
        let word_entry = WordEntry::clone(&word_entry_ref);
        let words = rsx! {
            div {
                WordPreview { word_entry }
            }
        };
        let wid = word_entry_ref.id;
        Some(rsx! {
            div {
                style: "border-bottom-width:1px;border-bottom-style: dotted",
                onclick: move |_| {
                    nav.push(AppRoute::EditorPage { wid });
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
            LazyList { container_height: height, make_item, item_height: 32.0, estimate_item_count }
        }
    }
}
