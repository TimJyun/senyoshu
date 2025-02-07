use std::ops::Deref;

use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use smallvec::SmallVec;

use senyoshu_common::types::learn::knowledge::Knowledge;
use senyoshu_common::types::learn::learn_knowledge_history::{OperateRecord, OperateType};
use senyoshu_common::util::time::UtcTimeStamp;

use crate::page::learn::preview_knowledge::PreviewKnowledge;
use crate::page::learn::KnowledgeData;
use crate::storage::workbook::WorkBook;

#[derive(Props, PartialEq, Clone)]
pub struct PreviewKnowledgeProps<T: PartialEq + Clone + 'static> {
    pub knowledge: Knowledge,
    pub data: T,
    pub on_ended: EventHandler,
}

pub fn Preview(props: PreviewKnowledgeProps<KnowledgeData>) -> Element {
    let PreviewKnowledgeProps {
        data,
        knowledge,
        on_ended,
    } = props;

    let mut playing = use_signal(|| true);
    let mut play_sound = use_coroutine({
        let sounds = data.get_preview_sound();
        |_rx: UnboundedReceiver<()>| async move {
            for (ruby, tone) in sounds.into_iter() {
                crate::voice::play(ruby, tone).await;
            }
            playing.set(false);
        }
    });

    //提前返回不会影响 use_state 的正确性
    //清除作为列表元素中残留的钩子
    let mut last_knowledge = use_signal(|| knowledge.to_owned());
    if last_knowledge.read().deref() != &knowledge {
        last_knowledge.set(knowledge.to_owned());
        playing.set(true);
        play_sound.restart();
        return None;
    }

    let preview_nodes = {
        let data = (data).to_owned();
        rsx! {
            PreviewKnowledge { data }
        }
    };

    let next_preview = {
        let knowledge: Knowledge = (&knowledge).to_owned();
        move |_| {
            WorkBook::add_record(
                knowledge.to_owned(),
                SmallVec::from([OperateRecord {
                    operate_type: OperateType::Seen,
                    operate_time: UtcTimeStamp::now(),
                }]),
            );
            let _ = &on_ended.call(());
        }
    };
    let next_button = rsx! {
        div {
            // id: "preview-knowledge",
            style: "
                background-color: #fff;
                display:flex;
                flex-direction: row;
                text-align:center;
                width:100%;
                user-select:none;
                height:48px",
            visibility: if *playing.read() { "hidden" },
            span {
                style: "
                    width:100%;
                    height:32px;
                    line-height:32px;
                    margin-top:8px;
                    margin-left: 10%;
                    margin-right: 10%;
                    border: 1px;
                    border-style: solid;
                    border-radius: 12px",
                onclick: next_preview,
                "next"
            }
        }
    };
    // js_sys::eval(
    //     "setTimeout(()=>{document.querySelector('#preview-knowledge').scrollIntoView({behavior:'smooth'});},1);"
    // ).expect("js eval fail");

    rsx! {
        {preview_nodes},
        {next_button}
    }
}
