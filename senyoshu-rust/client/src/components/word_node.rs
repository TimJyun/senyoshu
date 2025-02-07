use derive_more::From;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use gloo::console::log;

use senyoshu_common::types::kanji_alias::Kanji;
use senyoshu_common::types::word::wid::WordIdentity;
use senyoshu_common::types::word::word::Word;

use crate::components::sound::Sound;
use crate::imgs::AUDIO_IMG;
use crate::router::AppRoute;
use crate::storage::dictionary::DIC;

#[derive(Props, PartialEq, Clone)]
pub struct WordNodeProps {
    #[props(into)]
    pub word: WordSource,
    pub to_kanji: Option<bool>,
    pub hide_ruby: Option<bool>,
    pub hide_sound: Option<bool>,
    pub font_size: Option<f32>,
}

#[derive(From, PartialEq, Clone)]
pub enum WordSource {
    Word(Word),
    WordIdentity(WordIdentity),
}

pub fn WordNode(props: WordNodeProps) -> Element {
    let dic = DIC.read();
    let word = match &props.word {
        WordSource::Word(w) => w,
        WordSource::WordIdentity(wid) => &dic.get(&wid)?.word,
    };

    let tones = word.tones.to_string();
    let font_size = props.font_size.unwrap_or(1f32);

    let word_node = word.elements.iter().cloned().map(|it| {
        let txt_chars = it.txt.chars().map(|c| {
            if let (Some(true), Ok(kanji)) = (props.to_kanji, Kanji::try_from(c)) {
                let target = AppRoute::KanjiPage { kanji };
                rsx! {
                    Link {
                        to: target,
                        onclick: move |evt: Event<MouseData>| {
                            evt.stop_propagation();
                        },
                        "{c}"
                    }
                }
            } else {
                rsx! { "{c}" }
            }
        });

        //不渲染，并不是隐藏
        let ruby = if let Some(true) = props.hide_ruby {
            None
        } else {
            rsx! {
                rt { {it.ruby} }
            }
        };
        rsx! {
            ruby { style: "user-select: none;font-size: {font_size}rem",
                {txt_chars.into_iter()},
                {ruby}
            }
        }
    });

    let word_node = if let WordSource::WordIdentity(wid) = props.word {
        rsx! {
            Link {
                to: AppRoute::WordPage { wid },
                onclick: move |evt: Event<MouseData>| {
                    evt.stop_propagation();
                },
                {word_node}
            }
        }
    } else {
        rsx! {
            { word_node }
        }
    };

    let sound = if let Some(true) = props.hide_sound {
        None
    } else {
        rsx! {
            Sound { text: word.get_ruby(),
                img {
                    src: AUDIO_IMG,
                    style: "width:{font_size*1.25}rem;cursor:pointer;"
                }
            }
        }
    };

    rsx! {
        { word_node },
        {tones},
        {sound}
    }
}
