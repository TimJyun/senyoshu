use dioxus::prelude::*;

use crate::imgs::APP_ICON_256_256;
use crate::native::AndroidInterface;
use crate::request::get_local_host;

pub fn AboutPage() -> Element {
    let version_future = use_resource(|| async {
        let client = reqwest::Client::new();
        let host = get_local_host()?.trim_end_matches("/").to_string();
        let url = format!("{host}/version.txt");
        client.get(url).send().await.ok()?.json::<i64>().await.ok()
    });

    let version = if let Some(Some(version)) = version_future.value().read().as_ref() {
        version.to_string()
    } else {
        "unknown".to_string()
    };

    rsx! {
        div { text_align: "center",
            div {
                img { src: APP_ICON_256_256 }
            }
            "版本信息 ： {version}"
            div {
                onclick: move |_| {
                    if AndroidInterface::is_connected() {
                        AndroidInterface::open_url("https://t.me/senyoshu")
                    } else if let Some(window) = web_sys::window() {
                        let _ = window.open_with_url("https://t.me/senyoshu");
                    }
                },
                "官方telegram频道(点击跳转)"
            }
        }
    }
}
