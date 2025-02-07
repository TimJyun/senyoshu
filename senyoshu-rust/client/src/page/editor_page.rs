use std::collections::HashMap;

use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use serde::{Deserialize, Serialize};
use tracing::error;

use senyoshu_common::types::api::dic::{POST_WORD_API, UPDATE_MANY_API};
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word_entry::{WordDefine, WordEntry};
use senyoshu_common::util::string_util::StringUtil;

use crate::components::editor::editor::Editor;
use crate::global::BUSYING;
use crate::router::AppRoute;
use crate::singleton::confirm_box::confirm;
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::account::ACCOUNT;
use crate::storage::dictionary::DIC;
use crate::storage::use_storage::GlobalSignalStorage;
use crate::text::TEXT;

const WORDS_DRAFT: &str = "words_draft";

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EditorPageProps {
    wid: WordIdentity,
}

pub static WORD_DEFINE_DRAFT: GlobalSignalStorage<Draft> =
    GlobalSignalStorage::session(WORDS_DRAFT, Draft::default);

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Draft {
    new: Option<WordDefine>,
    edite: HashMap<WordIdentity, WordDefine>,
}

pub fn EditorPage(props: EditorPageProps) -> Element {
    let nav = use_navigator();

    let account_info = ACCOUNT.snap().unwrap();
    let token = account_info.token;

    let dic = DIC.read();
    let wid = props.wid;
    let word_define = use_signal(|| dic.get(&wid).unwrap().to_owned());

    let post_disabled = *BUSYING.read()
        || word_define.read().word.elements.is_empty()
        || word_define
            .read()
            .word
            .elements
            .iter()
            .any(|it| it.ruby.is_empty() || it.ruby.chars().any(|c| !StringUtil::is_kana(c)));

    let items = Vec::from([{
        let mut items = Vec::with_capacity(2);
        items.push(MenuItem {
            img: None,
            label: { TEXT.read().editor_page_action_submit },
            onclick: {
                let token = token.to_owned();
                EventHandler::new(move |_| {
                    *BUSYING.write() = true;
                    let token = token.to_owned();
                    spawn(async move {
                        if confirm(Vec::from([String::from("是否确定要提交更改？")])).await
                        {
                            let word_define = word_define.peek().to_owned();
                            let result = POST_WORD_API
                                .call(&(
                                    token.to_owned(),
                                    WordEntry {
                                        id: wid,
                                        word_define,
                                    },
                                ))
                                .await;
                            if let Ok(true) = result {
                                nav.push(AppRoute::WordPage { wid });
                            } else {
                                error!("提交失败");
                            }
                        };

                        *BUSYING.write() = false;
                    });
                })
            },
            disabled: post_disabled,
        });
        items.push(MenuItem {
            img: None,
            label: { TEXT.read().editor_page_action_submit_then_pass },
            onclick: EventHandler::new(move |_| {
                *BUSYING.write() = true;
                let token = token.to_owned();
                spawn(async move {
                    if confirm(Vec::from([String::from("是否确定要提交更改并确认通过？")])).await
                    {
                        let token = token.to_owned();
                        let word_define = word_define.peek().to_owned();
                        let result = UPDATE_MANY_API
                            .call(&(token, HashMap::from([(wid, word_define)])))
                            .await;
                        if let Ok(true) = result {
                            nav.push(AppRoute::WordPage { wid });
                        } else {
                            error!("提交失败");
                        }
                    };

                    *BUSYING.write() = false;
                });
            }),
            disabled: post_disabled,
        });

        items
    }]);

    TOP_NAVIGATION.set_menu_items(items);

    rsx! {
        div {
            Editor { word_define }
        }
    }
}
