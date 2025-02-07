use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::debug;

use senyoshu_common::types::word::tones::Tone;

use crate::voice::play;

#[derive(Props, Clone, PartialEq, Debug)]
pub struct SoundProps {
    pub text: String,
    pub children: Element,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum SoundText {
    Word { ruby: String, tone: Option<Tone> },
    Any(String),
}

pub fn Sound(props: SoundProps) -> Element {
    let play_onclick = move |evt: Event<MouseData>| {
        let text = props.text.to_string();
        debug!("play sound by click:{text}");
        spawn(async move {
            play(text, None).await;
        });
        evt.stop_propagation();
    };

    rsx! {
        span { onclick: play_onclick, {props.children} }
    }
}
