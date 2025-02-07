use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

use async_std::task::sleep;
use gloo::storage::{LocalStorage, Storage};
use itertools::Itertools;
use js_sys::wasm_bindgen::prelude::Closure;
use js_sys::wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use tracing::debug;
use web_sys::{SpeechSynthesisUtterance, SpeechSynthesisVoice};

use crate::tts::voice::Voice;

#[derive(Clone)]
pub struct TextToSpeech;

const VOICE_SETTING: &str = "voice_setting";

#[derive(Clone, Serialize, Deserialize)]
pub struct SpeakerSetting {
    pub(crate) volume: u8,
    pub(crate) speakers: HashMap<String, SpeakerState>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpeakerState {
    pub enabled: bool,
    pub volume: u8,
}

impl Default for SpeakerState {
    fn default() -> Self {
        Self {
            enabled: false,
            volume: 80,
        }
    }
}

impl TextToSpeech {
    pub fn get_setting() -> SpeakerSetting {
        LocalStorage::get::<SpeakerSetting>(VOICE_SETTING).unwrap_or(SpeakerSetting {
            volume: 50,
            speakers: Default::default(),
        })
    }
    pub fn with_setting(closure: impl FnOnce(&mut SpeakerSetting)) {
        let mut speaker_setting =
            LocalStorage::get::<SpeakerSetting>(VOICE_SETTING).unwrap_or(SpeakerSetting {
                volume: 50,
                speakers: Default::default(),
            });

        closure(&mut speaker_setting);
        let _ = LocalStorage::set(VOICE_SETTING, speaker_setting);
    }
    pub async fn speak(text: String) -> Option<()> {
        let speakers = TextToSpeech::get_speakers().await;
        let mut speaker = None;
        let setting = TextToSpeech::get_setting();
        let mut speaker_volume = 50u8;

        for (key, speaker_state) in setting
            .speakers
            .iter()
            .filter(|(_key, speaker_state)| speaker_state.enabled)
        {
            speaker = speakers.iter().find(|v| &v.name() == key);
            if speaker.is_some() {
                speaker_volume = speaker_state.volume;
                break;
            }
        }

        TextToSpeech::speak_with_speaker(
            text,
            speaker,
            (setting.volume as u16 * speaker_volume as u16) as f32 / 10000f32,
        )
        .await;

        Some(())
    }

    pub async fn get_speakers() -> Vec<Voice> {
        #[cfg(not(feature = "android"))]
        {
            let mut type_get_speaker_time = 0;
            for _ in 0..100 {
                type_get_speaker_time += 1;
                if let Some(window) = web_sys::window() {
                    if let Ok(ss) = window.speech_synthesis() {
                        let voice = ss
                            .get_voices()
                            .into_iter()
                            .map(|it| SpeechSynthesisVoice::from(it))
                            .filter(|it| it.lang().contains("ja") || it.lang().contains("jp"))
                            .map(|it| Voice(it))
                            .collect_vec();
                        if voice.len() > 0 {
                            debug!("type_get_speaker_time:{type_get_speaker_time}");
                            return voice;
                        }
                    }
                }
                sleep(std::time::Duration::from_millis(50)).await;
            }
            debug!("type_get_speaker_time:{type_get_speaker_time}");
        }

        #[cfg(feature = "android")]
        {
            let _ = js_sys::eval("console.log(tts.getVoices())");

            match js_sys::eval("tts.getVoices()") {
                Ok(voices) => {
                    if voices.is_null() {
                        debug!("voices:is_null");
                    } else if voices.is_undefined() {
                        debug!("voices:is_undefined");
                    }

                    let json = voices.as_string().unwrap();
                    let voices = serde_json::from_str::<Vec<String>>(json.as_str()).unwrap();
                    let voices: Vec<_> = voices
                        .into_iter()
                        .map(|js_value| Voice(js_value.into()))
                        .collect();
                    let len = voices.len();
                    debug!("voices-len:{len}",);
                    return voices;
                }
                Err(error) => {
                    let err = error.is_null();
                    if error.is_null() {
                        debug!("is_null");
                    } else if error.is_undefined() {
                        debug!("is_undefined");
                    } else if error.is_object() {
                        debug!("is_object");
                    } else if error.is_function() {
                        debug!("is_function");
                    }
                }
            }

            // debug!("get android voices fail");
        }

        Vec::new()
    }

    pub async fn speak_with_speaker(
        text: String,
        speaker: Option<&Voice>,
        volume: f32,
    ) -> Option<()> {
        #[cfg(not(feature = "android"))]
        {
            debug!("try to speak,volume:{volume}");

            let window = web_sys::window()?;
            let ss = window.speech_synthesis().ok()?;
            let ssu = SpeechSynthesisUtterance::new_with_text(text.as_str()).ok()?;
            ssu.set_voice(speaker.map(|s| &s.0));
            ssu.set_lang("ja");
            ssu.set_volume(volume);

            let playing = Rc::new(AtomicBool::new(true));
            let playing_in_closure = playing.to_owned();
            let closure: Closure<dyn Fn()> =
                Closure::new(move || playing_in_closure.store(false, Ordering::Relaxed));
            ssu.set_onend(Some(closure.as_ref().unchecked_ref()));
            ss.speak(&ssu);

            while playing.load(Ordering::Relaxed) {
                sleep(std::time::Duration::from_millis(10)).await;
            }

            drop(closure);
        }

        #[cfg(feature = "android")]
        {
            let speaker = speaker
                .map(|v| v.0.as_string())
                .flatten()
                .unwrap_or(String::from("undefined"));
            debug!(
                "{}",
                format!("tts.speakWithSpeaker('{text}','{speaker}',{volume});")
            );

            let handle = js_sys::eval(&format!(
                "tts.speakWithSpeaker('{text}','{speaker}',{volume});"
            ))
            .unwrap()
            .as_string()
            .unwrap();

            while !js_sys::eval(format!("tts.isDone('{handle}')").as_str())
                .map(|r| r.is_truthy())
                .unwrap_or(false)
            {
                sleep(std::time::Duration::from_millis(10)).await;
            }
        }

        Some(())
    }
}
