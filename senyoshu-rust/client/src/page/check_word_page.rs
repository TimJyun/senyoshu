use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use itertools::Itertools;

use senyoshu_common::types::api::api::WordHistoryEntry;
use senyoshu_common::types::api::dic::GET_CHANGE_REQUEST_API;

use crate::components::word_node::WordNode;
use crate::router::AppRoute;
use crate::storage::account::ACCOUNT;

pub fn CheckWordPage() -> Element {
    let mut change_word_requests = use_signal(|| Vec::<WordHistoryEntry>::new());

    let _ = use_coroutine(|_rx: UnboundedReceiver<()>| {
        //没有登录的根本不会来到这个页面
        let token = { ACCOUNT.snap().unwrap().token };
        async move {
            if let Ok(word_history_entry) = GET_CHANGE_REQUEST_API.call(&token).await {
                change_word_requests.set(word_history_entry);
            }
        }
    });

    let request_list = change_word_requests
        .iter()
        .map(|it| ((it.word.get_txt(), it.word.get_ruby()), it.to_owned()))
        .into_group_map();

    let s = request_list.into_iter().map(|((_txt, _ruby), requests)| {
        let word = requests.first().map(|it| it.word.to_owned()).unwrap();
        let entries = requests.into_iter().map(|it| {
            rsx! {
                Link {
                    to: AppRoute::InspectionPage {
                        pid: it.pid.into(),
                    },
                    div { style: "display:flex;flex-direction:row",
                        span { style: " flex:1", "{it.author}" }
                        span { style: "flex:3", "{it.post_date}" }
                    }
                }
            }
        });

        rsx! {
            details {
                summary {
                    WordNode { word }
                }
                {entries}
            }
        }
    });

    rsx! {
        // input { r#type: "button", value: "pass", onclick: |_| {} }
        // input { r#type: "button", value: "no pass", onclick: |_| {} }
        {s}
    }
}
