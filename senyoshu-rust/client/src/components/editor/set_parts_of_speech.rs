use async_std::io::WriteExt;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;

use senyoshu_common::types::word::parts_of_speech::{Compound, PartsOfSpeech, VerbConjugation};
use senyoshu_common::types::word::word_entry::WordDefine;

use crate::components::button::Button;
use crate::components::dialog::Dialog;

#[derive(Props, PartialEq, Clone)]
pub struct SetPartsOfSpeechProps {
    pub word_define: Signal<WordDefine>,
    pub mean_index: usize,
}

pub fn SetPartsOfSpeech(props: SetPartsOfSpeechProps) -> Element {
    let mut word_define = props.word_define;
    let mean_index = props.mean_index;
    let parts_of_speech = (&word_define.read().means[mean_index].parts_of_speech).to_owned();

    const WORD_CLASS_CONVERSION: [&str; 3] = ["five_row_verb", "one_row_verb", "irregular_verb"];

    let mut dialog_show = use_signal(|| false);

    let verb_class_nodes = if let Some(verb_class) = parts_of_speech.verb.to_owned() {
        let verb_conjugation_radios = [
            ("一段動詞", VerbConjugation::OneRowVerb),
            ("五段動詞", VerbConjugation::FiveRowVerb),
            ("不規則動詞", VerbConjugation::IrregularVerb),
        ]
        .into_iter()
        .map(|(label, conjugation)| {
            let checked = verb_class.conjugation == Some(conjugation.to_owned());
            rsx! {
                div {
                    label {
                        input {
                            r#type: "radio",
                            name: "conjugation",
                            checked,
                            onchange: move |_| {
                                let mut word_entry_mut = word_define.write();
                                let verb_class = word_entry_mut
                                    .means[mean_index]
                                    .parts_of_speech
                                    .verb
                                    .as_mut()
                                    .unwrap();
                                verb_class.conjugation = Some(conjugation.to_owned());
                            }
                        }
                        "{label}"
                    }
                }
            }
        });

        let verb_target_class = rsx! {
            div {
                label {
                    input {
                        r#type: "checkbox",
                        checked: verb_class.transitive,
                        onchange: move |_| {
                            word_define
                                .write()
                                .means[mean_index]
                                .parts_of_speech
                                .verb
                                .as_mut()
                                .unwrap()
                                .transitive = !verb_class.transitive;
                        }
                    }
                    "他動詞"
                }
            }
            div {
                label {
                    input {
                        r#type: "checkbox",
                        checked: verb_class.intransitive,
                        onchange: move |_| {
                            word_define
                                .write()
                                .means[mean_index]
                                .parts_of_speech
                                .verb
                                .as_mut()
                                .unwrap()
                                .intransitive = !verb_class.intransitive;
                        }
                    }
                    "自動詞"
                }
            }
        };
        rsx! {
            div {
                details { style: if parts_of_speech.verb.is_none() { "display:none;" } else { "margin-left:1rem;" },
                    summary { "動詞活用" }
                    {verb_conjugation_radios}
                }
            }
            div {

                details { style: if parts_of_speech.verb.is_none() { "display:none;" } else { "margin-left:1rem;" },
                    summary { "動詞結合価" }
                    {verb_target_class}
                }
            }
        }
    } else {
        None
    };

    #[derive(Copy, Clone)]
    enum Pos {
        Adjective,
        NaAdjective,
        Adverb,
        Interjection,
        Phrase,
        Verb,
    }

    fn SetPos(
        mut word_define: Signal<WordDefine>,
        mean_index: usize,
        parts_of_speech: &PartsOfSpeech,
        pos: Pos,
    ) {
        match pos {
            Pos::Adjective => {
                word_define
                    .write()
                    .means
                    .get_mut(mean_index)
                    .unwrap()
                    .parts_of_speech
                    .adjective = !parts_of_speech.adjective;
            }
            Pos::NaAdjective => {
                word_define
                    .write()
                    .means
                    .get_mut(mean_index)
                    .unwrap()
                    .parts_of_speech
                    .na_adjective = !parts_of_speech.na_adjective;
            }
            Pos::Adverb => {
                word_define
                    .write()
                    .means
                    .get_mut(mean_index)
                    .unwrap()
                    .parts_of_speech
                    .adverb = !parts_of_speech.adverb;
            }
            Pos::Interjection => {
                word_define
                    .write()
                    .means
                    .get_mut(mean_index)
                    .unwrap()
                    .parts_of_speech
                    .interjection = !parts_of_speech.interjection;
            }
            Pos::Phrase => {
                word_define
                    .write()
                    .means
                    .get_mut(mean_index)
                    .unwrap()
                    .parts_of_speech
                    .phrase = !parts_of_speech.phrase;
            }
            Pos::Verb => {
                if parts_of_speech.verb.is_some() {
                    word_define
                        .write()
                        .means
                        .get_mut(mean_index)
                        .unwrap()
                        .parts_of_speech
                        .verb = None;
                } else {
                    word_define
                        .write()
                        .means
                        .get_mut(mean_index)
                        .unwrap()
                        .parts_of_speech
                        .verb = Some(Default::default());
                }
            }
        }
    }

    let mut new_custom_pos = use_signal(String::new);

    let others_nodes = parts_of_speech
        .others
        .iter()
        .cloned()
        .map(|p| {
            rsx! {
                span { style: "padding: 2px;border: 1px;border-style: solid;border-radius: 4px;user-select: none;",
                    span { "{p}" }
                    span { " | " }
                    span {
                        onclick: move |_| {
                            word_define
                                .write()
                                .means
                                .get_mut(mean_index)
                                .unwrap()
                                .parts_of_speech
                                .others
                                .remove(&p);
                        },
                        "×"
                    }
                }
            }
        })
        .chain([rsx! {
            div {
                span { "新品詞：" }
                input {
                    style: "width:200px",
                    oninput: move |evt| {
                        new_custom_pos.set(evt.value());
                    }
                }
                button {
                    onclick: move |_| {
                        {
                            let new_custom_pos = new_custom_pos.to_string();
                            if new_custom_pos.is_empty() {
                                return;
                            }
                            word_define
                                .write()
                                .means
                                .get_mut(mean_index)
                                .unwrap()
                                .parts_of_speech
                                .others
                                .insert(new_custom_pos);
                        }
                        new_custom_pos.set(String::new());
                    },
                    "追加"
                }
            }
        }]);

    let pos_checkboxes = [
        ("形容詞", parts_of_speech.adjective, Pos::Adjective),
        ("形容動詞", parts_of_speech.na_adjective, Pos::NaAdjective),
        ("副詞", parts_of_speech.adverb, Pos::Adverb),
        ("感嘆詞", parts_of_speech.interjection, Pos::Interjection),
        ("句", parts_of_speech.phrase, Pos::Phrase),
        ("動詞", parts_of_speech.verb.is_some(), Pos::Verb),
    ]
    .into_iter()
    .map(|(label, checked, pos)| {
        rsx! {
            div {
                label { style: "width:96px;display:inline-block",
                    input {
                        r#type: "checkbox",
                        checked,
                        onchange: {
                            let parts_of_speech = parts_of_speech.to_owned();
                            move |_| { SetPos(word_define, mean_index, &parts_of_speech, pos) }
                        }
                    }
                    "{label}"
                }
            }
        }
    });

    let ui = rsx! {
        div {
            label { style: "width:96px;display:inline-block",
                input {
                    r#type: "checkbox",
                    checked: parts_of_speech.noun.is_some(),
                    onchange: move |_| {
                        if parts_of_speech.noun.is_some() {
                            word_define
                                .write()
                                .means
                                .get_mut(mean_index)
                                .unwrap()
                                .parts_of_speech
                                .noun = None;
                        } else {
                            word_define
                                .write()
                                .means
                                .get_mut(mean_index)
                                .unwrap()
                                .parts_of_speech
                                .noun = Some(Default::default());
                        }
                    }
                }
                "名詞"
            }
            details { style: if parts_of_speech.noun.is_none() { "display:none;" } else { "margin-left:1rem;" },
                summary { "固有名詞" }

                div {
                    label {
                        input {
                            r#type: "checkbox",
                            checked: parts_of_speech.noun.map(|n| n.people_name).unwrap_or(false),
                            onchange: move |_| {
                                word_define
                                    .write()
                                    .means[mean_index]
                                    .parts_of_speech
                                    .noun
                                    .as_mut()
                                    .unwrap()
                                    .people_name = !parts_of_speech.noun.map(|n| n.people_name).unwrap_or(false);
                            }
                        }
                        "人名"
                    }
                }
                div {
                    label {
                        input {
                            r#type: "checkbox",
                            checked: parts_of_speech.noun.map(|n| n.place_name).unwrap_or(false),
                            onchange: move |_| {
                                word_define
                                    .write()
                                    .means[mean_index]
                                    .parts_of_speech
                                    .noun
                                    .as_mut()
                                    .unwrap()
                                    .place_name = !parts_of_speech.noun.map(|n| n.place_name).unwrap_or(false);
                            }
                        }
                        "地名"
                    }
                }
            }
        }

        {pos_checkboxes},
        {verb_class_nodes},
        details { open: parts_of_speech.compound != Compound::IsNot,
            summary { "接辞" }
            div {
                label {
                    input {
                        r#type: "radio",
                        checked: parts_of_speech.compound == Compound::IsNot,
                        onchange: move |_| {
                            let mut word_entry_mut = word_define.write();
                            word_entry_mut.means.get_mut(mean_index).unwrap().parts_of_speech.compound = Compound::IsNot;
                        }
                    }
                    "接辞ではありません"
                }
            }
            div {
                label {
                    input {
                        r#type: "radio",
                        checked: parts_of_speech.compound == Compound::Prefix,
                        onchange: move |_| {
                            let mut word_entry_mut = word_define.write();
                            word_entry_mut.means.get_mut(mean_index).unwrap().parts_of_speech.compound = Compound::Prefix;
                        }
                    }
                    "接頭辞"
                }
            }
            div {
                label {
                    input {
                        r#type: "radio",
                        checked: parts_of_speech.compound == Compound::Suffix,
                        onchange: move |_| {
                            let mut word_entry_mut = word_define.write();
                            word_entry_mut.means.get_mut(mean_index).unwrap().parts_of_speech.compound = Compound::Suffix;
                        }
                    }
                    "接尾辞"
                }
            }
        }

        div { {others_nodes} }
    };

    rsx! {
        Button {
            custom_style: "font-size:0.75rem;margin:2px",
            onclick: move |_| { dialog_show.set(true) },
            "編集"
        }
        Dialog { dialog_show,
            // style:"max-width:400px",
            fieldset { display: "block", margin: "0 auto", width: "360px",
                legend { margin: "0 auto", "品詞" }
                div { text_align: "center", style: "text-align:center",
                    div { style: "text-align:left;display:inline", {ui} }
                }
            }
        }
    }
}
