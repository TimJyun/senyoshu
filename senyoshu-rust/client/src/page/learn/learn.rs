use std::ops::Deref;

use dioxus::core_macro::rsx;
use dioxus::prelude::*;

use senyoshu_common::types::learn::knowledge::Knowledge;
use senyoshu_common::types::learn::plan::Plan;

use crate::page::learn::kana::LearnKana;
use crate::page::learn::kanji::LearnKanji;
use crate::page::learn::txt::LearnTxt;
use crate::page::learn::KnowledgeData;

#[derive(Props, PartialEq, Clone)]
pub struct LearnProps {
    pub knowledge: Knowledge,
    pub data: KnowledgeData,
    pub plan: Plan,
    pub on_ended: EventHandler,
}

pub(super) fn Learn(props: LearnProps) -> Element {
    let LearnProps {
        data,
        knowledge,
        plan,
        on_ended,
    } = props;

    //提前返回不会影响 use_state 的正确性
    //清除作为列表元素中残留的钩子
    let mut last_knowledge = use_signal(|| knowledge.to_owned());
    if last_knowledge.read().deref() != &knowledge {
        last_knowledge.set(knowledge.to_owned());
        return None;
    }

    let on_ended = move |evt| {
        on_ended.call(evt);
    };

    let nodes = match (data).to_owned() {
        KnowledgeData::Kanji(data) => {
            rsx! {
                LearnKanji { knowledge: knowledge.to_owned(), data, on_ended, plan }
            }
        }
        KnowledgeData::Txt(data) => {
            rsx! {
                LearnTxt { knowledge: knowledge.to_owned(), data, on_ended, plan }
            }
        }
        KnowledgeData::Kana(data) => {
            rsx! {
                LearnKana { knowledge: knowledge.to_owned(), data, on_ended, plan }
            }
        }
    };

    nodes
}
