use js_sys::wasm_bindgen::JsValue;
use js_sys::Reflect;
use tracing::debug;

pub fn download(data: String, file_name: String) {
    debug!(
        "try to download file:{file_name} , file-len:{}",
        data.as_bytes().len()
    );
    let download_js = "(function(){
        let aTag = document.createElement('a');
        let to_be_downloaded_file = window.to_be_downloaded_file;
        let blob = new Blob([to_be_downloaded_file]);
        aTag.download = '"
        .to_string()
        + file_name.as_str()
        + "';
        aTag.href = URL.createObjectURL(blob);
        aTag.click();
        URL.revokeObjectURL(blob);
    })();";

    Reflect::set(
        &JsValue::from(web_sys::window().unwrap()),
        &JsValue::from("to_be_downloaded_file"),
        &JsValue::from(data),
    )
    .unwrap();
    js_sys::eval(download_js.as_str()).unwrap();
    Reflect::set(
        &JsValue::from(web_sys::window().unwrap()),
        &JsValue::from("to_be_downloaded_file"),
        &JsValue::from(JsValue::UNDEFINED),
    )
    .unwrap();
}
