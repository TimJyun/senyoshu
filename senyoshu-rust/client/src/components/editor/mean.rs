use dioxus::core_macro::rsx;
use dioxus::prelude::*;

use senyoshu_common::types::word::mean_entry::{MeanEntry, SentenceIndex};
use senyoshu_common::types::word::word_entry::WordDefine;

use crate::components::button::Button;
use crate::components::editor::parts_of_speech::RenderPartsOfSpeech;

#[derive(Props, PartialEq, Clone)]
pub struct MeanProps {
    pub word_define: Signal<WordDefine>,
    pub mean_index: usize,
}

pub fn RenderMean(mut props: MeanProps) -> Element {
    let mut word_define = props.word_define;
    let mean_index = props.mean_index;
    let word_define_current = word_define.read();
    let mean_entry: &MeanEntry = word_define_current.means.get(mean_index).expect("");

    let explanation_nodes = [
        ("中文释义:", SentenceIndex::ZH),
        ("explanation:", SentenceIndex::EN),
    ]
        .into_iter()
        .map(|(label, index)| {
            let content = mean_entry.explanation.get_by_index(index);
            rsx! {
                div { style: "margin:2px;",
                    span { style: "width:120px;display:inline-block;border-bottom-width:1px;border-bottom-style:solid",
                        "{label}"
                    }
                    input {
                        style: "width:75%",
                        value: "{content}",
                        onchange: move |evt| {
                            *props
                                .word_define
                                .write()
                                .means
                                .get_mut(mean_index)
                                .unwrap()
                                .explanation
                                .get_mut_by_index(index) = evt.value().to_owned()
                        }
                    }
                }
            }
        });

    rsx! {
        div {
            div {
                RenderPartsOfSpeech { word_define, mean_index: mean_index.to_owned() }
                Button {
                    custom_style: "font-size:0.75rem;margin:2px",
                    onclick: move |_| {
                        word_define.write().means.remove(mean_index);
                    },
                    "削除"
                }
            }

            {explanation_nodes}
        }
    }
}
