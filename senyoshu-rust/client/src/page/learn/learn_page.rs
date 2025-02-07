use chrono::{DateTime, Days, Utc};
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use senyoshu_common::types::learn::knowledge::Knowledge;
use senyoshu_common::types::learn::learn_knowledge_history::{OperateRecord, OperateType};
use senyoshu_common::types::learn::plan::Plan;
use senyoshu_common::util::time::UtcTimeStamp;

use crate::components::button::Button;
use crate::imgs::{
    AUDIO_MARK_MENU_ITEM, CLEAN_MENU_ITEM, MUTE_MENU_ITEM, QUESTION_MARK_MENU_ITEM,
    REFRESH_MENU_ITEM,
};
use crate::page::learn::learn::Learn;
use crate::page::learn::preview::Preview;
use crate::page::learn::preview_knowledge::PreviewKnowledge;
use crate::page::learn::KnowledgeData;
use crate::router::AppRoute;
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::account::ACCOUNT;
use crate::storage::dictionary::DIC;
use crate::storage::setting::SETTING;
use crate::storage::use_storage::GlobalSignalStorage;
use crate::storage::workbook::WorkBook;

static LEARN_HISTORY: GlobalSignalStorage<Vec<(Knowledge, Plan)>> =
    GlobalSignalStorage::session("learn_history", || Vec::<(Knowledge, Plan)>::new());

static LAST_REMIND_LOGIN: GlobalSignalStorage<DateTime<Utc>> =
    GlobalSignalStorage::local("last_remind_login", || DateTime::<Utc>::MIN_UTC);

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct LearnPageProps {}

pub fn LearnPage() -> Element {
    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_no_padding(true);
    TOP_NAVIGATION.set_no_back_button(true);
    TOP_NAVIGATION.set_no_background(true);

    let account_info = ACCOUNT.snap();
    let nav = use_navigator();
    if account_info.is_none() {
        let last_remind = LAST_REMIND_LOGIN.read();
        let now = Utc::now();
        if *last_remind < now - Days::new(1) {
            return rsx! {
                div { style: "text-align:center;margin-top:240px",
                    div {
                        Button {
                            onclick: move |_| {
                                nav.push(AppRoute::LoginPage {});
                            },
                            "登录 "
                        }
                        span { style: "margin-left:4px", "以获取同步学习记录" }
                    }
                    div { style: "height:32px" }
                    div {
                        Button {
                            onclick: |_| {
                                *LAST_REMIND_LOGIN.write() = Utc::now();
                            },
                            "继续学习"
                        }
                    }
                }
            };
        }
    }

    let dic = DIC.read();

    let mut learn_plan = use_signal(|| WorkBook::plan());

    let least = learn_plan
        .read()
        .iter()
        .filter(|(_, p)| p.next_review_time < UtcTimeStamp::now())
        .count();

    TOP_NAVIGATION.set_content(Some(rsx! {
        span { style: "display:flex",
            span { style: "flex:1" }
            span { style: "background: white;user-select: none;", "{least}" }
        }
    }));

    let on_ended = move |_| {
        if let Some((k, p)) = learn_plan.write().pop() {
            let mut history = LEARN_HISTORY.write();
            history.push((k, p));
        }
    };

    let history_snap = LEARN_HISTORY.read();
    let history_nodes = history_snap.iter().map(|(k, _p)| {
        let data = KnowledgeData::get_learn_data(k, &dic).unwrap();
        rsx! {
            div { style: "border-bottom-width:1px;border-bottom-style: dotted",
                PreviewKnowledge { data }
            }
        }
    });

    //迷惑，为什么 if let Some((k, p)) = { learn_plan.read().last().cloned() } 会拿着引用不放手
    //但是造个临时变量不会
    //猜测是常量函数 last() 造成了右值提升: https://rust-lang.github.io/rfcs/1414-rvalue_static_promotion.html

    let next_opt = { learn_plan.read().last().cloned() };
    let current_learn = if let Some((k, p)) = next_opt {
        if (p.last_seen_time + UtcTimeStamp::hour()) < UtcTimeStamp::now() {
            let data = KnowledgeData::get_learn_data(&k, &dic);
            if let Some(data) = data {
                if p.next_review_time < UtcTimeStamp::now() {
                    js_sys::eval(
                        "setTimeout(()=>{document.querySelector('#learn-knowledge').scrollIntoView({behavior:'smooth'});},1);"
                    ).expect("js eval fail");
                    rsx! {
                        div { id: "learn-knowledge",
                            Learn { data, knowledge: k.to_owned(), on_ended, plan: p.to_owned() }
                        }
                    }
                } else {
                    js_sys::eval(
                        "setTimeout(()=>{document.querySelector('#learn-knowledge').scrollIntoView(false);},1);"
                    ).expect("js eval fail");
                    rsx! {
                        div { id: "learn-knowledge",
                            Preview { data, knowledge: k.to_owned(), on_ended }
                        }
                    }
                }
            } else {
                learn_plan.write().pop();
                return None;
            }
        } else {
            learn_plan.write().pop();
            return None;
        }
    } else {
        rsx! { "all is learn" }
    };

    let mut items = Vec::with_capacity(4);

    let setting = SETTING.read();
    let silent_mode_button = if setting.silent_mode {
        MenuItem {
            img: Some(AUDIO_MARK_MENU_ITEM),
            label: "unmute",
            onclick: EventHandler::new(|_| {
                let mut setting = SETTING.write();
                (*setting).silent_mode = !(*setting).silent_mode;
            }),
            ..Default::default()
        }
    } else {
        MenuItem {
            img: Some(MUTE_MENU_ITEM),
            label: "mute",
            onclick: EventHandler::new(|_| {
                let mut setting = SETTING.write();
                (*setting).silent_mode = !(*setting).silent_mode;
            }),
            ..Default::default()
        }
    };
    items.push(Vec::from([silent_mode_button]));

    if learn_plan.read().last().is_some() {
        let set_forget_button = MenuItem {
            img: Some(QUESTION_MARK_MENU_ITEM),
            label: "set forget",
            onclick: EventHandler::new(move |_| {
                if let Some((k, p)) = learn_plan.write().pop() {
                    WorkBook::add_record(
                        k.to_owned(),
                        [OperateRecord {
                            operate_type: OperateType::Forget,
                            operate_time: UtcTimeStamp::now(),
                        }],
                    );
                    let mut history = LEARN_HISTORY.write();
                    history.push((k, p));
                }
            }),
            ..Default::default()
        };
        items.push(Vec::from([set_forget_button]))
    };
    let clear_history_button = MenuItem {
        img: Some(CLEAN_MENU_ITEM),
        label: "clear history",
        onclick: EventHandler::new(|_| {
            LEARN_HISTORY.reset();
        }),
        ..Default::default()
    };
    items.push(Vec::from([clear_history_button]));

    let regenerate_plan_button = MenuItem {
        img: Some(REFRESH_MENU_ITEM),
        label: "regenerate plan",
        onclick: EventHandler::new(move |_| {
            learn_plan.set(WorkBook::plan());
        }),
        ..Default::default()
    };

    items.push(Vec::from([regenerate_plan_button]));

    TOP_NAVIGATION.set_menu_items(items);

    rsx! {
        div { style: "user-select:none",
            {history_nodes},
            {current_learn}
        }
    }
}
