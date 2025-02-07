use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use futures::FutureExt;

use senyoshu_common::types::word::mean_entry::{MeanEntry, Sentence, SentenceIndex};
use senyoshu_common::types::word::parts_of_speech::PartsOfSpeech;
use senyoshu_common::types::word::word_entry::{ExampleSentence, WordDefine};
use senyoshu_common::util::iter_util::WithNextMutMapItertool;

use crate::components::word_node::WordNode;

#[derive(Props, PartialEq, Clone)]
pub struct ViewerNodeProps {
    pub word_define: WordDefine,
    pub align_left: Option<bool>,
}

// "編集"
pub fn ViewerNode(props: ViewerNodeProps) -> Element {
    let word_define = props.word_define;

    let word = word_define.word;

    let loan = word_define.loan.map(|loan| {
        rsx! {
            "("
            {loan.language},
            ")"
            {loan.source_word}
        }
    });

    let means = word_define.means;
    let render_mean_container = RenderMeanContainer(means);
    let detailed_nodes = {
        let is_empty = word_define.detailed.is_empty();

        let detailed_nodes = word_define
            .detailed
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| line.is_empty() == false)
            .map(|line| {
                rsx! {
                    div { {line} }
                }
            });

        if is_empty {
            None
        } else {
            rsx! {
                fieldset { style: "max-width: 75%;",
                    legend { "詳細" }
                    {detailed_nodes}
                }
            }
        }
    };

    rsx! {
        div { style: "font-size:1rem",
            div { style: if props.align_left.unwrap_or(false) == false { "text-align:center;" },
                WordNode { word, to_kanji: true, font_size: 2. }
            }
            div { style: "height:2px;width:50%;background-color:#808080;" }
            div { style: "font-size:1.25rem", {loan} }
            {render_mean_container},
            div { { RenderExampleSentenceContainer(word_define.example_sentences)} }
            {detailed_nodes}
        }
    }
}

pub fn RenderExampleSentence(example: ExampleSentence) -> impl Iterator<Item = Element> {
    [
        ("範例:", {
            let exam_sent = example.ja.into_iter().map(|ele| {
                rsx! {
                    ruby { style: "user-select: none;",
                        {ele.txt},
                        rt { {ele.ruby} }
                    }
                }
            });
            rsx! {
                {exam_sent}
            }
        }),
        (
            "例句:",
            if example.zh.is_empty() {
                None
            } else {
                rsx! {
                    {example.zh}
                }
            },
        ),
        (
            "e. g:",
            if example.en.is_empty() {
                None
            } else {
                rsx! {
                    {example.en}
                }
            },
        ),
    ]
    .into_iter()
    .filter(|(_, example)| example.is_some())
    .with_next_mut_map(|(label, example), next| {
        let dividing_line = if next.is_none() {
            rsx! {
                div { style: "height:1px;width:50%;background-color:#808080;" }
            }
        } else {
            None
        };

        rsx! {
            div {
                span { style: "width:30px;display:inline-block;" }
                span { style: "width:60px;display:inline-block", "{label}" }
                {example}
            }
            {dividing_line}
        }
    })
}

pub fn RenderMean(mean_entry: MeanEntry) -> Element {
    let explanation = mean_entry.explanation;
    let explanation_nodes = [
        (
            "中文释义:",
            explanation.get_by_index(SentenceIndex::ZH).to_string(),
        ),
        (
            "explanation:",
            explanation.get_by_index(SentenceIndex::EN).to_string(),
        ),
    ]
    .into_iter()
    .filter(|(_, content)| !content.is_empty())
    .map(|(label, content)| {
        rsx! {
            div {
                span { style: "width:120px;display:inline-block", "{label}" }
                {content}
            }
            div { style: "height:1px;width:50%;background-color:#808080;" }
        }
    });

    rsx! {
        div {
            div { style: "font-size:1.25rem", "{mean_entry.parts_of_speech}" }

            {explanation_nodes}
        }
    }
}

pub fn RenderMeanContainer(means: Vec<MeanEntry>) -> impl Iterator<Item = Element> {
    means.into_iter().enumerate().map(|(_index, mean_entry)| {
        rsx! {
            {RenderMean(mean_entry)}
        }
    })
}

pub fn RenderExampleSentenceContainer(
    example_sentences: Vec<ExampleSentence>,
) -> impl Iterator<Item = Element> {
    example_sentences.into_iter().map(|s| {
        rsx! {
            {RenderExampleSentence(s)}
        }
    })
}
