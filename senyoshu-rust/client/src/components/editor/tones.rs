use dioxus::core_macro::rsx;
use dioxus::html::i;
use dioxus::prelude::*;
use gloo::console::{console, log};
use tracing::log::debug;

use senyoshu_common::types::word::tones::TONE_SIGNS;
use senyoshu_common::types::word::word_entry::{WordDefine, WordEntry};

#[derive(Props, PartialEq, Clone)]
pub struct TonesProps {
    pub word_define: Signal<WordDefine>,
}

pub fn Tones(props: TonesProps) -> Element {
    let mut word_define = props.word_define;
    let tone = word_define.read().word.tones.to_owned();
    let tone_checkboxes = (0..6).map(|index| {
        let checked = tone.0[index];
        rsx! {
            span {
                style: "user-select:none;",
                onclick: move |_| {
                    let mut word_define = word_define.write();
                    word_define.word.tones.0[index] = !checked;
                    log!("tone-index:", index, "checked:",! checked);
                    let tones = word_define.word.tones.to_string();
                    log!("tone:", tones);
                },
                "{TONE_SIGNS[index]}"
                input { r#type: "checkbox", checked }
            }
        }
    });
    rsx! {
        fieldset { style: "display:block;width:240px",
            legend { style: "margin:auto", "編集アクセント" }
            {tone_checkboxes}
        }
    }
}
