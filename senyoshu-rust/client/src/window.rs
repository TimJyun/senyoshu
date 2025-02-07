use dioxus::prelude::{GlobalSignal, Readable, Signal};
use js_sys::wasm_bindgen::closure::Closure;
use js_sys::wasm_bindgen::JsCast;
use once_cell::sync::Lazy;

use crate::storage::setting::Language;
use crate::task::TASK_DEQUE;

pub static WINDOW_LANGUAGE: Lazy<Option<Language>> = Lazy::new(|| {
    let window = web_sys::window()?;
    let nav = window.navigator();
    let language_str = nav.language()?;
    if language_str.starts_with("zh") {
        Some(Language::Zh)
    } else if language_str.starts_with("en") {
        Some(Language::En)
    } else if language_str.starts_with("ja") {
        Some(Language::Ja)
    } else {
        None
    }
});

pub static WINDOW_WIDTH: GlobalSignal<Option<f64>> = Signal::global(|| {
    gloo::utils::window()
        .inner_width()
        .map(|width| width.as_f64())
        .ok()
        .flatten()
});
pub static WINDOW_HEIGHT: GlobalSignal<Option<f64>> = Signal::global(|| {
    gloo::utils::window()
        .inner_height()
        .map(|height| height.as_f64())
        .ok()
        .flatten()
});

pub(super) fn add_window_size_change_listener() {
    if let Some(window) = web_sys::window() {
        let on_window_size_change: Closure<dyn Fn()> = Closure::new(move || {
            TASK_DEQUE.add(|| {
                if let Some(window) = web_sys::window() {
                    *WINDOW_WIDTH.write() = window
                        .inner_width()
                        .map(|width| width.as_f64())
                        .ok()
                        .flatten();
                    *WINDOW_HEIGHT.write() = window
                        .inner_height()
                        .map(|height| height.as_f64())
                        .ok()
                        .flatten();
                }
                #[cfg(feature = "android")]
                {
                    let _ = js_sys::eval("(function(){document.querySelector('html').style.height = window.innerHeight + 'px';})();");
                }
            });
        });
        let _ = window.add_event_listener_with_callback(
            "resize",
            on_window_size_change.as_ref().unchecked_ref(),
        );
        on_window_size_change.forget();
    };
}

pub fn is_widescreen() -> bool {
    if let (Some(inner_width), Some(inner_height)) = (*WINDOW_WIDTH.read(), *WINDOW_HEIGHT.read()) {
        inner_width * 2. >= inner_height * 3.
    } else {
        false
    }
}
