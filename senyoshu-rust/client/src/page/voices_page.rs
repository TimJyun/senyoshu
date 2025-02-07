use std::collections::HashMap;

use dioxus::prelude::*;
use itertools::Itertools;

use crate::imgs::{RESET_MENU_ITEM, SAVE_MENU_ITEM};
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::voice_setting::{SpeakerSetting, TextToSpeech};
use crate::text::TEXT;

pub fn VoicesPage() -> Element {
    let voices = use_resource(|| async { TextToSpeech::get_speakers().await });

    let voices = voices.value();
    let voices = voices.read();
    let voices = voices.to_owned();
    let voices = voices?;

    let mut example_word = use_signal(|| "こんにちは".to_string());
    let mut setting_temp = use_signal(|| TextToSpeech::get_setting());
    let list = voices.into_iter().map(|voice| {
        let voice_name = voice.name();
        let speaker_state = setting_temp
            .read()
            .speakers
            .get(&voice_name)
            .cloned()
            .unwrap_or_default();
        rsx! {
            div { key: "voice-name:{voice.name()}",
                {voice.name()},
                input {
                    r#type: "range",
                    min: 0,
                    max: 100,
                    step: 1,
                    value: speaker_state.volume as i64,
                    onchange: {
                        let voice = voice.to_owned();
                        move |evt| {
                            let volume = evt.value().to_string().parse::<u8>().unwrap_or(80);
                            setting_temp.write().speakers.entry(voice.name()).or_default().volume = volume;
                        }
                    }
                }
                input {
                    r#type: "button",
                    value: TEXT.read().voice_page_action_try_listen,
                    onclick: {
                        let voice = voice.to_owned();
                        move |_| {
                            let voice = voice.to_owned();
                            let example_word = example_word.to_string();
                            let speaker_volume = speaker_state.volume;
                            let global_volume = setting_temp.peek().volume;
                            let volumn = (global_volume as u16 * speaker_volume as u16) as f32
                                / 10000f32;
                            spawn(async move {
                                TextToSpeech::speak_with_speaker(
                                        example_word,
                                        Some(voice).as_ref(),
                                        volumn,
                                    )
                                    .await;
                            });
                        }
                    }
                }
                input {
                    r#type: "checkbox",
                    checked: speaker_state.enabled,
                    onchange: move |_| {
                        let mut selected_new = setting_temp.write();
                        let speaker_state = selected_new.speakers.entry(voice.name()).or_default();
                        speaker_state.enabled = !speaker_state.enabled;
                    }
                }
            }
        }
    });

    let items = vec![
        vec![MenuItem {
            img: Some(SAVE_MENU_ITEM),
            label: TEXT.read().voice_page_action_save_setting,
            onclick: EventHandler::new(move |_| {
                TextToSpeech::with_setting(|setting| {
                    *setting = (*setting_temp.peek()).to_owned();
                })
            }),
            disabled: false,
        }],
        vec![MenuItem {
            img: Some(RESET_MENU_ITEM),
            label: TEXT.read().voice_page_action_reset_to_default,
            onclick: EventHandler::new(move |_| {
                setting_temp.set(SpeakerSetting {
                    volume: 80,
                    speakers: HashMap::new(),
                });
            }),
            disabled: false,
        }],
    ];

    TOP_NAVIGATION.set_menu_items(items);

    rsx! {
        div {
            {TEXT.read().voice_page_global_volume},
            " : "
            input {
                r#type: "range",
                min: 0,
                max: 100,
                step: 1,
                value: setting_temp.read().volume as i64,
                onchange: move |evt| {
                    let volume = evt.value().to_string().parse::<u8>().unwrap_or(80);
                    setting_temp.write().volume = volume;
                }
            }
        }
        div {
            {
                TEXT.read().voice_page_example_word
            },
            " : "
            input {
                value: "{example_word}",
                onchange: move |evt| {
                    example_word.set(evt.value().to_string());
                }
            }
        }

        {list}
    }
}
