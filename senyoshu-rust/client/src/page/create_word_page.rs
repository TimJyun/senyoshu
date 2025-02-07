use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use tracing::debug;

use senyoshu_common::types::api::account::UserInfo;
use senyoshu_common::types::api::dic::CREATE_WORD_API;
use senyoshu_common::types::word::word_entry::WordDefine;
use senyoshu_common::util::string_util::StringUtil;

use crate::components::editor::editor::Editor;
use crate::global::BUSYING;
use crate::router::AppRoute;
use crate::singleton::confirm_box::confirm;
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::account::{AccountInfo, ACCOUNT};
use crate::storage::dictionary::Dic;
use crate::storage::use_storage::GlobalSignalStorage;

static WORD_DEFINE_DRAFT: GlobalSignalStorage<WordDefine> =
    GlobalSignalStorage::local("create_word_draft", WordDefine::template);

pub fn CreateWordPage() -> Element {
    let nav = use_navigator();

    let mut word_define_signal = use_signal(|| WORD_DEFINE_DRAFT.peek().to_owned());
    {
        let not_eq = { *WORD_DEFINE_DRAFT.read() != *word_define_signal.read() };
        if not_eq {
            *WORD_DEFINE_DRAFT.write() = word_define_signal.peek().to_owned();
        }
    }

    if let Some(AccountInfo {
        user_info: UserInfo {
            content_maintainer: true,
            ..
        },
        token,
    }) = ACCOUNT.peek()
    {
        let post_disabled =
            *BUSYING.read()
                || WORD_DEFINE_DRAFT.read().word.elements.is_empty()
                || WORD_DEFINE_DRAFT.read().word.elements.iter().any(|it| {
                    it.ruby.is_empty() || it.ruby.chars().any(|c| !StringUtil::is_kana(c))
                });

        let items = vec![vec![MenuItem {
            img: None,
            label: "提出",
            onclick: EventHandler::new(move |_| {
                *BUSYING.write() = true;
                let token = token.to_owned();
                let word_define = WORD_DEFINE_DRAFT.peek().to_owned();
                spawn(async move {
                    if confirm(vec![String::from("是否确认要创建单词？")]).await {
                        debug!("正在创建单词");
                        let mut success = false;
                        for _ in 0..3 {
                            let param = (token.to_owned(), word_define.to_owned());
                            let result = CREATE_WORD_API.call(&param).await;
                            if let Ok(Some(wid)) = result {
                                WORD_DEFINE_DRAFT.reset();
                                *word_define_signal.write() = WORD_DEFINE_DRAFT.peek().to_owned();
                                success = true;
                                Dic::update().await;
                                nav.push(AppRoute::WordPage { wid });
                                break;
                            }
                        }
                        if success == false {
                            debug!("提交失败");
                        }
                    } else {
                        debug!("创建单词已取消");
                    }
                    *BUSYING.write() = false;
                });
            }),
            disabled: post_disabled,
        }]];

        TOP_NAVIGATION.set_menu_items(items);

        rsx! {
            div {
                Editor { word_define: word_define_signal }
            }
        }
    } else {
        debug!("无权限");
        None
    }
}
