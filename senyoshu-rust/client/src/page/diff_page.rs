use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use gloo::console::debug;
use serde::{Deserialize, Serialize};

use senyoshu_common::types::api::dic::{DELETE_WORD_API, UPDATE_MANY_API};
use senyoshu_common::types::integer::Integer;
use senyoshu_common::types::word::parts_of_speech::DefaultExt;
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::WordDefine;

use crate::components::button::Button;
use crate::components::editor::editor::RenderEditor;
use crate::singleton::confirm_box::confirm;
use crate::singleton::top_navigation::{MenuItem, TOP_NAVIGATION};
use crate::storage::account::ACCOUNT;
use crate::storage::dictionary::{Dic, DIC};
use crate::window::is_widescreen;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct DiffPageProps {
    pub wid: WordIdentity,
    pub wid2: WordIdentity,
}

pub fn DiffPage(props: DiffPageProps) -> Element {
    let dic = DIC.read();
    let nav = use_navigator();

    let token = ACCOUNT.snap().unwrap().token;

    let wid = props.wid;
    let wid2 = props.wid2;

    debug!("wid:{wid}");
    debug!("wid2:{wid2}");

    let mut word_define = use_signal(|| dic.get(&wid).unwrap().to_owned());
    let mut word_define2 = use_signal(|| dic.get(&wid2).unwrap().to_owned());

    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_menu_items(Vec::from([Vec::from([MenuItem {
        img: None,
        label: "提交左侧并删除右侧",
        onclick: EventHandler::new(move |_| {
            let token = token.to_owned();
            spawn(async move {
                if confirm(Vec::from([String::from("您确定要合并词汇？")])).await {
                    for i in 0..3 {
                        let result = UPDATE_MANY_API
                            .call(&(
                                token.to_owned(),
                                HashMap::from([(wid.0.into(), word_define.read().to_owned())]),
                            ))
                            .await;
                        if result.unwrap_or(false) {
                            break;
                        }
                        if i == 2 {
                            return debug!("提交失败");
                        }
                    }
                    for i in 0..3 {
                        let result = DELETE_WORD_API
                            .call(&(token.to_owned(), (wid2.0.into())))
                            .await;
                        if result.unwrap_or(false) {
                            break;
                        }
                        if i == 2 {
                            return debug!("删除失败");
                        }
                    }
                    Dic::update().await;
                    nav.go_back();
                    debug!("提交并删除成功");
                }
            });
        }),
        disabled: {
            word_define.read().word.get_txt() != word_define2.read().word.get_txt()
                || word_define.read().word.get_katakana() != word_define2.read().word.get_katakana()
                || word_define.read().word.tones.eq_default()
                || word_define
                    .read()
                    .means
                    .iter()
                    .any(|mean| mean.parts_of_speech.others.contains("外"))
                || (!word_define2.read().word.tones.eq_default())
        },
    }])]));

    rsx! {
        div {
            Button {
                onclick: move |_| {
                    let word_define_tmp = { word_define.peek().to_owned() };
                    {
                        *word_define.write() = { word_define2.peek().to_owned() };
                    }
                    *word_define2.write() = word_define_tmp;
                },
                "交换两侧"
            }

            Button {
                onclick: move |_| {
                    let word_define_tmp = { word_define.peek().to_owned() };
                    let word_define_tmp2 = { word_define2.peek().to_owned() };
                    if word_define_tmp.word.elements.len() == word_define_tmp2.word.elements.len() {
                        return;
                    }
                    if word_define_tmp.word.elements.len() < word_define_tmp2.word.elements.len() {
                        word_define.write().word.elements = word_define_tmp2
                            .word
                            .elements
                            .to_owned();
                    } else if word_define_tmp.word.elements.len()
                        > word_define_tmp2.word.elements.len()
                    {
                        word_define2.write().word.elements = word_define_tmp
                            .word
                            .elements
                            .to_owned();
                    }
                },
                "自动分词"
            }
            DiffEditor { word_define, word_define2 }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct DiffEditorProps {
    pub word_define: Signal<WordDefine>,
    pub word_define2: Signal<WordDefine>,
}
pub fn DiffEditor(props: DiffEditorProps) -> Element {
    let word_define = props.word_define;
    let word_define2 = props.word_define2;
    rsx! {
        div { style: "display:flex;flex-direction:row",
            span { style: "flex:1",
                RenderEditor { word_define }
            }
            span { style: "flex:1",
                RenderEditor { word_define: word_define2 }
            }
        }
    }
}
