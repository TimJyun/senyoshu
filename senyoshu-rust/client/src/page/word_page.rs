use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use serde::{Deserialize, Serialize};
use tracing::debug;

use senyoshu_common::types::api::dic::DELETE_WORD_API;
use senyoshu_common::types::word::wid::WordIdentity;

use crate::components::viewer::ViewerNode;
use crate::router::AppRoute;
use crate::singleton::confirm_box::confirm;
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::account::{AccountInfo, ACCOUNT};
use crate::storage::dictionary::DIC;
use crate::text::TEXT;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WordPageProps {
    wid: WordIdentity,
}

pub fn WordPage(props: WordPageProps) -> Element {
    TOP_NAVIGATION.reset();
    let dic = DIC.read();
    let wid = props.wid;
    let word_define_opt = dic.get(&wid).cloned();
    let nav = use_navigator();
    let account_info = ACCOUNT.snap();
    if let Some(word_define) = word_define_opt {
        let items = account_info
            .map(|AccountInfo { token, user_info }| {
                let mut items = Vec::with_capacity(2);
                items.push(Vec::from([MenuItem {
                    label: TEXT.read().word_page_action_edite,
                    onclick: EventHandler::new(move |_| {
                        nav.push(AppRoute::EditorPage { wid: props.wid });
                    }),
                    ..Default::default()
                }]));
                if user_info.content_maintainer {
                    let delete_button_onclick = move |_| {
                        let token = token.to_owned();
                        spawn(async move {
                            if confirm(Vec::from([
                                String::from("您确定要删除该词汇？"),
                                String::from("Are you sure you want to delete this word?"),
                            ]))
                            .await
                            {
                                if let Ok(true) = DELETE_WORD_API.call(&(token, *wid)).await {
                                    nav.go_back();
                                } else {
                                    debug!("删除失败");
                                }
                            }
                        });
                    };
                    items.push(Vec::from([MenuItem {
                        label: TEXT.read().word_page_action_delete,
                        onclick: EventHandler::new(delete_button_onclick),
                        ..Default::default()
                    }]));
                };
                items
            })
            .unwrap_or_default();

        TOP_NAVIGATION.set_menu_items(items);

        return rsx! {
            ViewerNode { word_define }
        };
    }

    rsx! { "the word that id is {wid} is not found in dictionary" }
}
