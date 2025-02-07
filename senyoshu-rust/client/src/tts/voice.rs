#[cfg(feature = "android")]
use js_sys::wasm_bindgen::JsValue;
#[cfg(not(feature = "android"))]
use web_sys::SpeechSynthesisVoice;

//SpeechSynthesisVoice
#[derive(Clone)]
pub struct Voice(
    // JsValue
    #[cfg(not(feature = "android"))] pub SpeechSynthesisVoice,
    #[cfg(feature = "android")] pub JsValue,
);

impl Voice {
    pub fn name(&self) -> String {
        #[cfg(feature = "android")]
        {
            return self.0.as_string().unwrap_or_default();
        }
        #[cfg(not(feature = "android"))]
        {
            return self.0.name();
        }
    }
}
