use dioxus::core_macro::rsx;
use dioxus::prelude::*;

use senyoshu_common::types::word::word_entry::WordDefine;

use crate::components::button::Button;
use crate::components::editor::mean::RenderMean;

#[derive(Props, PartialEq, Clone)]
pub struct MeanContainerProps {
    pub word_define: Signal<WordDefine>,
}

pub fn RenderMeanContainer(props: MeanContainerProps) -> Element {
    let mut word_define = props.word_define;
    let means = &word_define.read().means;

    let RenderMeans = means.iter().enumerate().map(|(mean_index, _)| {
        rsx! {
            RenderMean { word_define, mean_index }
        }
    });

    rsx! {
        {RenderMeans}
    }
}
