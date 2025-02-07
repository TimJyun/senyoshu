pub struct AndroidInterface;

impl AndroidInterface {
    pub(crate) fn is_connected() -> bool {
        js_sys::eval("if (android){true} else {false}")
            .map(|js_value| js_value.as_bool() == Some(true))
            .unwrap_or(false)
    }
    pub fn launch() {
        let _ = js_sys::eval("android.launch()");
    }
    pub fn config() {
        let _ = js_sys::eval("android.config()");
    }
    pub fn select() {
        let _ = js_sys::eval("android.select()");
    }
    pub fn stop() {
        let _ = js_sys::eval("android.stop()");
    }

    pub fn open_url(url: &str) {
        let _ = js_sys::eval(format!("android.openUrl('{url}')").as_str());
    }

    pub fn is_surfing() -> bool {
        js_sys::eval("if (android.isSurfing()){true} else {false}")
            .map(|js_value| js_value.as_bool() == Some(true))
            .unwrap_or(false)
    }
}
