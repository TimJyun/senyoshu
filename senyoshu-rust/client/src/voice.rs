use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use async_std::task::sleep;
use js_sys::wasm_bindgen::prelude::Closure;
use js_sys::wasm_bindgen::JsCast;
use js_sys::Uint8Array;
use tracing::debug;
use web_sys::AudioBuffer;

use senyoshu_common::types::word::tones::Tone;

use crate::storage::setting::Setting;
use crate::storage::voice_setting::TextToSpeech;

// pub async fn play_sound(sid: i64) -> Option<()> {
//     //todo:包装一下,解决缓存
//     // let sound_u8vec = get_sound_api(sid).await.ok()?;
//     // play_sound_u8v(sound_u8vec).await;
//     Some(())
// }

pub const PLAYING_SOUND: AtomicBool = AtomicBool::new(false);

pub async fn play(kana: String, _tone: Option<Tone>) -> Option<()> {
    //todo:调用服务器

    debug!("try to play: {kana}");

    let silent_mode = Setting::get().silent_mode;
    if silent_mode {
        sleep(Duration::from_millis(1000)).await;
    } else {
        TextToSpeech::speak(kana).await?;
    }

    Some(())
}

pub async fn play_sound_u8v(sound_u8vec: Vec<u8>) -> Option<()> {
    let sound_u8a = Uint8Array::from(sound_u8vec.as_slice());
    let audio_context = web_sys::AudioContext::new().ok()?;
    let v = audio_context.decode_audio_data(&sound_u8a.buffer()).ok()?;
    let b = wasm_bindgen_futures::JsFuture::from(v).await.ok()?;
    let source = audio_context.create_buffer_source().ok()?;
    source.set_buffer(Some(&AudioBuffer::from(b)));
    source
        .connect_with_audio_node(&audio_context.destination())
        .ok()?;

    let onended = Arc::new(AtomicBool::new(false));
    let onended_in_closure = onended.to_owned();
    let closure: Closure<dyn Fn()> = Closure::new(move || {
        let start = onended_in_closure.load(Ordering::Relaxed);
        debug!("onended_in_closure_start: {start}");
        let _ = (&onended_in_closure).compare_exchange(
            false,
            true,
            Ordering::Relaxed,
            Ordering::Relaxed,
        );
        let end = onended_in_closure.load(Ordering::Relaxed);
        debug!("onended_in_closure_end: {end}");
    });
    source.set_onended(Some(closure.as_ref().unchecked_ref()));

    if let Ok(_) = source.start() {
        while !onended.load(Ordering::Relaxed) {
            sleep(Duration::from_millis(10)).await;
        }
        Some(())
    } else {
        None
    }
}
