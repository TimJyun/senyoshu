use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;
use tracing::debug;

use crate::components::button::Button;

static CONFIRM_BOX_SHOW: GlobalSignal<bool> = GlobalSignal::new(|| false);
static CONFIRM_BOX_TEXT: GlobalSignal<Vec<String>> = GlobalSignal::new(|| Vec::new());
static CONFIRM_BOX_RESULT: GlobalSignal<Option<bool>> = GlobalSignal::new(|| None);

pub(crate) fn ConfirmBox() -> Element {
    if *CONFIRM_BOX_SHOW.read() == false {
        return None;
    }

    let confirm_box_text = CONFIRM_BOX_TEXT.read();
    let paragraphs = confirm_box_text.iter().map(|p| {
        rsx! {
            p { "{p}" }
        }
    });

    rsx! {
        div {
            style: "
                    position: fixed;
                    z-index: 10086;
                    left: 0;
                    top: 0;
                    width:100%;
                    height:100%;
                    overflow: auto;
                    background-color: rgba(0,0,0,0.4);
                ",
            onclick: move |_evt| {
                debug!("confirm: cancel by background");
                *CONFIRM_BOX_SHOW.write() = false;
                *CONFIRM_BOX_TEXT.write() = Vec::new();
                *CONFIRM_BOX_RESULT.write() = Some(false);
            },
            div {
                style: "
                        z-index: 10087;
                        background-color: #fefefe;
                        margin: 15% auto;
                        padding: 20px;
                        border: 1px solid #888;
                        max-width:400px;
                    ",
                onclick: |evt| {
                    debug!("confirm: stop propagation");
                    evt.stop_propagation();
                },
                div {
                    style: "
                            z-index: 10088;
                            user-select:none;
                            color: #aaa;
                            text-align:right;
                            font-size: 32px;
                            font-weight: bold;
                            width:auto;
                        ",
                    onclick: move |_evt| {
                        debug!("confirm: ×");
                        *CONFIRM_BOX_SHOW.write() = false;
                        *CONFIRM_BOX_TEXT.write() = Vec::new();
                        *CONFIRM_BOX_RESULT.write() = Some(false);
                    },
                    "×"
                }
                div {
                    {paragraphs},
                    span { style: "display:flex",
                        Button {
                            custom_style: "flex:1;text-align:center;z-index: 10089",
                            onclick: |_| {
                                debug!("confirm: yes");
                                *CONFIRM_BOX_SHOW.write() = false;
                                *CONFIRM_BOX_TEXT.write() = Vec::new();
                                *CONFIRM_BOX_RESULT.write() = Some(true);
                            },
                            "yes"
                        }
                        Button {
                            custom_style: "flex:1;text-align:center;z-index: 10089",
                            onclick: |_| {
                                debug!("confirm: no");
                                *CONFIRM_BOX_SHOW.write() = false;
                                *CONFIRM_BOX_TEXT.write() = Vec::new();
                                *CONFIRM_BOX_RESULT.write() = Some(false);
                            },
                            "no"
                        }
                    }
                }
            }
        }
    }
}

pub async fn confirm(txt: Vec<String>) -> bool {
    while *CONFIRM_BOX_SHOW.peek() {
        sleep(Duration::from_millis(10)).await;
    }
    *CONFIRM_BOX_SHOW.write() = true;
    *CONFIRM_BOX_TEXT.write() = txt;
    while CONFIRM_BOX_RESULT.peek().is_none() {
        sleep(Duration::from_millis(10)).await;
    }

    let rv = CONFIRM_BOX_RESULT.peek().unwrap_or_default();

    *CONFIRM_BOX_RESULT.write() = None;

    rv
}
