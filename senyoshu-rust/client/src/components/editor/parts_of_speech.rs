use dioxus::core_macro::rsx;
use dioxus::prelude::*;

use senyoshu_common::types::word::word_entry::WordDefine;

use crate::components::editor::set_parts_of_speech::SetPartsOfSpeech;

#[derive(Props, PartialEq, Clone)]
pub struct PartsOfSpeechProps {
    pub word_define: Signal<WordDefine>,
    pub mean_index: usize,
}

pub fn RenderPartsOfSpeech(props: PartsOfSpeechProps) -> Element {
    let word_define = props.word_define;
    let mean_index = props.mean_index;
    let pos: String = (&word_define.read().means[mean_index].parts_of_speech).to_string();

    rsx! {
        span { style: "font-size:1.25rem", "{pos}" }

        SetPartsOfSpeech { word_define, mean_index }
    }
}
