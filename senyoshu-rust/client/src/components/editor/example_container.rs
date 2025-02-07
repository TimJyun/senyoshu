use dioxus::core_macro::rsx;
use dioxus::prelude::*;

use senyoshu_common::types::word::word_entry::WordDefine;

use crate::components::button::Button;
use crate::components::editor::example_sentence::RenderEditorExampleSentence;

#[derive(Props, PartialEq, Clone)]
pub struct ExampleSentenceContainerProps {
    pub word_define: Signal<WordDefine>,
}

pub fn RenderEditorExampleSentenceContainer(props: ExampleSentenceContainerProps) -> Element {
    let mut word_define = props.word_define;
    let RenderSentences =
        (0usize..word_define.read().example_sentences.len()).map(|example_index| {
            rsx! {
                RenderEditorExampleSentence { word_define: props.word_define, example_index }
            }
        });

    rsx! {
        {RenderSentences}
    }
}
