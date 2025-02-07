use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use senyoshu_common::types::api::dic::{GET_WORD_BY_PID_API, SET_ADOPTED_API};
use senyoshu_common::types::integer::Integer;
use senyoshu_common::types::state::State;

use crate::components::button::Button;
use crate::components::viewer::ViewerNode;
use crate::global::BUSYING;
use crate::singleton::top_navigation::{MenuItem, TOP_NAVIGATION};
use crate::storage::account::ACCOUNT;
use crate::storage::dictionary::{Dic, DIC};
use crate::window::is_widescreen;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InspectionPageProps {
    pid: Integer,
}

pub fn InspectionPage(props: InspectionPageProps) -> Element {
    let dic = DIC.read();
    let nav = use_navigator();

    let future =
        use_resource(move || async move { GET_WORD_BY_PID_API.call(&props.pid.0).await.ok()? });
    let _ = use_coroutine({
        |_rx: UnboundedReceiver<()>| async {
            Dic::update().await;
        }
    });

    let mut show_new = use_signal(|| true);

    if let Some(new_word_entry) = future()? {
        let accept = move |_| {
            *BUSYING.write() = true;
            let pid = props.pid.0;
            //没有登录的根本不会来到这个页面
            let token = { ACCOUNT.snap().unwrap().token };
            spawn(async move {
                let rv = SET_ADOPTED_API
                    .call(&(token.to_owned(), pid, State::Pass))
                    .await;
                if let Ok(true) = rv {
                    debug!("提交成功");
                    nav.go_back();
                } else {
                    error!("提交失败");
                }
                *BUSYING.write() = false;
            });
        };
        let cancel = move |_| {
            *BUSYING.write() = true;
            let pid = props.pid.0;
            //没有登录的根本不会来到这个页面
            let token = { ACCOUNT.snap().unwrap().token };
            spawn(async move {
                let rv = SET_ADOPTED_API
                    .call(&(token.to_owned(), pid, State::Cancel))
                    .await;
                if let Ok(true) = rv {
                    debug!("取消成功");
                    nav.go_back();
                } else {
                    debug!("取消失败");
                }
                *BUSYING.write() = false;
            });
        };

        let old_define_viewer_node = if let Some(wd) = dic.get(&new_word_entry.id) {
            rsx! {
                ViewerNode { word_define: wd.to_owned() }
            }
        } else {
            None
        };
        let viewer_node = if is_widescreen() || old_define_viewer_node.is_none() {
            rsx! {
                div { display: "flex", flex_direction: "row",
                    span { flex: 1,
                        ViewerNode { word_define: new_word_entry.word_define.to_owned() }
                    }
                    span { flex: 1, {old_define_viewer_node} }
                }
            }
        } else {
            let ui = if *show_new.read() {
                rsx! {
                    ViewerNode { word_define: new_word_entry.word_define.to_owned() }
                }
            } else {
                old_define_viewer_node
            };
            rsx! {
                div { style: "text-align:center",
                    Button {
                        disabled: *show_new.read(),
                        onclick: move |_| {
                            show_new.set(true);
                        },
                        "新しい(new)"
                    }
                    Button {
                        disabled: *show_new.read() == false,
                        onclick: move |_| {
                            show_new.set(false);
                        },
                        "古い(old)"
                    }
                    {ui}
                }
            }
        };

        let items = Vec::from([Vec::from([
            MenuItem {
                label: "accept",
                onclick: EventHandler::new(accept),
                disabled: *BUSYING.read(),
                ..Default::default()
            },
            MenuItem {
                label: "cancel",
                onclick: EventHandler::new(cancel),
                disabled: *BUSYING.read(),
                ..Default::default()
            },
        ])]);

        TOP_NAVIGATION.set_menu_items(items);
        rsx! {
            {viewer_node}
        }
    } else {
        rsx! { "pid not found or access denied" }
    }
}
