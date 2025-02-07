use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use tracing::debug;

#[derive(Props, PartialEq, Clone)]
pub struct DialogProps {
    pub dialog_show: Signal<bool>,
    pub on_close: Option<EventHandler<MouseEvent>>,

    //form
    pub children: Element,
}

//这里的几个按钮回调函数有时候会莫名地失效，但是每个回调开头插个日志就会莫名地修复
//参考:https://www.runoob.com/w3cnote/javascript-modals.html
pub fn Dialog(props: DialogProps) -> Element {
    let mut dialog_show_signal = props.dialog_show;
    if *props.dialog_show.read() {
        rsx! {
            div {
                style: "
                    position: fixed;
                    z-index: 1;
                    left: 0;
                    top: 0;
                    width:100%;
                    height:100%;
                    overflow: auto;
                    background-color: rgba(0,0,0,0.4);
                ",
                onclick: move |evt| {
                    debug!("dialog: close by background");
                    evt.stop_propagation();
                    dialog_show_signal.set(false);
                    if let Some(on_close) = props.on_close.as_ref() {
                        on_close.call(evt)
                    }
                },
                div {
                    style: "
                        z-index: 2;
                        background-color: #fefefe;
                        margin: 15% auto;
                        padding: 20px;
                        border: 1px solid #888;
                        max-width:80%;
                    ",
                    onclick: |evt| {
                        debug!("dialog: stop propagation");
                        evt.stop_propagation();
                    },
                    div { style: "text-align:right",
                        span {
                            style: "
                            user-select:none;
                            color: #aaa;
                            font-size: 32px;
                            font-weight: bold;
                        ",
                            onclick: move |evt| {
                                debug!("dialog: close by ×");
                                evt.stop_propagation();
                                dialog_show_signal.set(false);
                                if let Some(on_close) = props.on_close.as_ref() {
                                    on_close.call(evt)
                                }
                            },
                            "×"
                        }
                    }
                    div { {props.children} }
                }
            }
        }
    } else {
        None
    }
}
