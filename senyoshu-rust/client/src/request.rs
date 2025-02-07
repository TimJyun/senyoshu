pub fn get_local_host() -> Option<String> {
    if let Some(window) = web_sys::window() {
        let location = window.location();
        if let (Ok(protocol), Ok(host)) = (location.protocol(), location.host()) {
            Some(format!("{protocol}//{host}/"))
        } else {
            None
        }
    } else {
        None
    }
}
