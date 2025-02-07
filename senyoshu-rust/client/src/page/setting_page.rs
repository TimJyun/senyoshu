use dioxus::prelude::*;

use crate::storage::setting::{Language, SETTING};
use crate::text::TEXT;

pub fn SettingPage() -> Element {
    let setting = SETTING.read();

    rsx! {
        fieldset {
            legend { "Choose GUI language" }
            div {
                label {
                    input {
                        r#type: "radio",
                        checked: setting.language.is_none(),
                        onclick: |_| {
                            SETTING.write().language = None;
                        }
                    }
                    "Default/默认/デフォルト"
                }
            }
            div {
                label {
                    input {
                        r#type: "radio",
                        checked: setting.language == Some(Language::Zh),
                        onclick: |_| {
                            SETTING.write().language = Some(Language::Zh);
                        }
                    }
                    "中文"
                }
            }
            div {
                label {
                    input {
                        r#type: "radio",
                        checked: setting.language == Some(Language::En),
                        onclick: |_| {
                            SETTING.write().language = Some(Language::En);
                        }
                    }
                    "English"
                }
            }
            div {
                label {
                    input {
                        r#type: "radio",
                        checked: setting.language == Some(Language::Ja),
                        onclick: |_| {
                            SETTING.write().language = Some(Language::Ja);
                        }
                    }
                    "日本語"
                }
            }
        }
        div {
            label {
                input {
                    r#type: "checkbox",
                    checked: setting.show_refresh_app_button,
                    onclick: |_| {
                        let change_to = !SETTING.peek().show_refresh_app_button;
                        SETTING.write().show_refresh_app_button = change_to;
                    }
                }
                {TEXT.read().setting_page_menu_show_refresh_app}
            }
        }
    }
}
