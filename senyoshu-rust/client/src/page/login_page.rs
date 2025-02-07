use std::ops::Deref;
use std::time::Duration;

use async_std::prelude::StreamExt;
use async_std::task::sleep;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::hooks::use_navigator;

use senyoshu_common::types::api::account::REGISTER_API;
use senyoshu_common::util::passwd_hasher::{get_passwd_hash, is_legal_username};

use crate::components::button::Button;
use crate::global::BUSYING;
use crate::router::AppRoute;
use crate::storage::account::ACCOUNT;
use crate::text::TEXT;

pub const MAIL_REGEX_STR: &str = ".+@.+\\..+";
// Regex::new(MAIL_REGEX_STR)
// .map(|it| !it.is_match())
// .unwrap_or(true)

pub fn LoginPage() -> Element {
    let nav = use_navigator();
    let mut forwarded = use_signal(|| false);
    let mut register_mode = use_signal(|| false);
    let mut username = use_signal(|| String::new());
    let mut passwd = use_signal(|| String::new());
    let mut passwd2 = use_signal(|| String::new());
    let mut note = use_signal(|| "");

    if ACCOUNT.snap().is_some() {
        if *forwarded.read() == false {
            forwarded.set(true);
            spawn(async move {
                sleep(Duration::from_secs(1)).await;
                if nav.can_go_back() {
                    nav.go_back();
                } else {
                    nav.replace(AppRoute::HomePage {});
                }
            });
        }
        return rsx! {
            div { "have logged " }
            div { "forward to home page in 1 second" }
        };
    }

    let width = "width:120px;display:inline-block;text-align:right;";

    let login_onclick = move |_| {
        let is_legal_username = is_legal_username(username.peek().as_str());
        let is_passwd_empty = passwd.peek().is_empty();
        if is_legal_username == false {
            note.set("note: username is error");
            return;
        } else if is_passwd_empty {
            note.set("note: password should not be empty");
            return;
        } else {
            *BUSYING.write() = true;
            let username = username.peek().to_string();
            let passwd = passwd.peek().to_string();
            spawn(async move {
                if ACCOUNT.login(username, passwd).await == false {
                    note.set("note: password may be is error .");
                }
                *BUSYING.write() = false;
            });
        }
    };

    let register_onclick = move |_| {
        let is_legal_username = is_legal_username(username.peek().as_str());
        let is_passwd_empty = passwd.peek().is_empty();
        let is_register_mode = *register_mode.peek();
        let two_passwd_match = String::eq(passwd.peek().deref(), passwd2.peek().deref());

        if is_legal_username == false {
            note.set("note: username is error");
            return;
        } else if is_passwd_empty {
            note.set("note: password should not be empty");
            return;
        } else if is_register_mode == false {
            register_mode.set(true);
            note.set("note: please repeat the password");
            return;
        } else if two_passwd_match == false {
            note.set("note: tow password input should be same");
            return;
        } else {
            *BUSYING.write() = true;
            let username = username.peek().to_string();
            let passwd = passwd.peek().to_string();
            spawn(async move {
                match REGISTER_API
                    .call(&(username.to_string(), get_passwd_hash(&passwd)))
                    .await
                {
                    Ok(true) => {
                        if ACCOUNT.login(username, passwd).await == false {
                            note.set("error: register success but login failed");
                        }
                    }
                    _ => {
                        note.set("note: failed , maybe the username has been registered");
                    }
                }
                *BUSYING.write() = false;
            });
        }
    };

    let passwd_repeat = if *register_mode.read() {
        rsx! {
            div {
                span { style: width,
                    {TEXT.read().login_page_password},
                    ":"
                }
                input {
                    r#type: "password",
                    onchange: move |evt| {
                        passwd2.set(evt.value().to_string());
                    }
                }
            }
        }
    } else {
        None
    };

    rsx! {
        div {
            div {
                span { style: width,
                    {TEXT.read().login_page_username},
                    ":"
                }
                input {
                    onchange: move |evt| {
                        username.set(evt.value().to_string());
                    }
                }
            }
            div {
                span { style: width,
                    {TEXT.read().login_page_password},
                    ":"
                }
                input {
                    r#type: "password",
                    onchange: move |evt| {
                        passwd.set(evt.value().to_string());
                    }
                }
            }
            {passwd_repeat},

            div {
                Button { disabled: *BUSYING.read(), onclick: login_onclick, {TEXT.read().login_page_sign_in} }
                Button { disabled: *BUSYING.read(), onclick: register_onclick, {TEXT.read().login_page_register} }
            }

            div { {note} }
        }
    }
}
