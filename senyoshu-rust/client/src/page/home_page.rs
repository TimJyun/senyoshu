use chrono::Utc;
use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::components::button::Button;
use crate::imgs::{FORWARD_12_12, LOGOUT_MENU_ITEM, SETTING_BUTTON, SWITCH_OFF, SWITCH_ON, SYNC};
use crate::native::AndroidInterface;
use crate::refresh_app;
use crate::router::AppRoute;
use crate::singleton::top_navigation::MenuItem;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::singleton::top_navigation::TOP_NAVIGATION_HEIGHT;
use crate::storage::account::{AccountInfo, ACCOUNT};
use crate::storage::workbook::WorkBook;
use crate::text::TEXT;

pub fn HomePage() -> Element {
    let _ = use_coroutine(|_rx: UnboundedReceiver<()>| async move {
        ACCOUNT.refresh().await;
    });

    //todo:sync from native
    let mut is_surfing = use_signal(|| AndroidInterface::is_surfing());

    let account_info = ACCOUNT.snap();
    let nav = use_navigator();

    let menu_items = Vec::from([Vec::from([MenuItem {
        img: Some(LOGOUT_MENU_ITEM),
        label: TEXT.read().home_page_sign_out,
        onclick: EventHandler::new(move |_| {
            ACCOUNT.login_out();
        }),
        disabled: account_info.is_none(),
    }])]);

    let mut is_content_maintainer = false;

    let login_info = if let Some(AccountInfo { user_info, token }) = account_info {
        is_content_maintainer = user_info.content_maintainer;

        let last_sync = WorkBook::get_last_sync_time()
            .map(|date| {
                let sub = (Utc::now().timestamp() - date.timestamp()) / 60;
                format!("{sub} min ago")
            })
            .unwrap_or(String::from("unknown"));

        //todo:fix android webview style
        const STYLE: &str =
            "font-size:1.25rem;display:flex;flex-direction:row;height:32px;align-items: center";
        rsx! {
            div { style: STYLE,
                span { style: "flex:1", "username：" }
                span { style: "flex:1", { user_info.username } }
            }
            div { style: STYLE,
                span { style: "flex:1", "uid：" }
                span { style: "flex:1", "{user_info.uid}" }
            }
            div { style: STYLE,
                span { style: "flex:1", "last update：" }
                span { style: "flex:1;display:flex;flex-direction:row;align-items: center",
                    span { "{last_sync}" }
                    img {
                        style: "padding:4px",
                        src: SYNC,
                        onclick: move |_| {
                            let token = token.to_owned();
                            spawn(async {
                                if WorkBook::sync(token).await {
                                    refresh_app()
                                }
                            });
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div { style: "text-align:center",
                div { {TEXT.read().home_page_to_login_in_now_button_hint} }
                div { height: "24px" }
                div {
                    Button {
                        custom_style: "font-size: 2rem",
                        onclick: move |_| {
                            nav.push(AppRoute::LoginPage {});
                        },
                        {TEXT.read().home_page_to_login_in_now_button}
                    }
                }
            }
        }
    };

    TOP_NAVIGATION.reset();
    TOP_NAVIGATION.set_menu_items(menu_items);
    TOP_NAVIGATION.set_no_back_button(true);

    rsx! {
        //todo:check
        //top nav padding
        div { style: "height:{TOP_NAVIGATION_HEIGHT}px;width:100%;" }

        fieldset { style: "margin: 0px auto;max-width:84%;width:360px;min-height:240px;display:flex;flex-direction: column;justify-content: center;",
            {login_info}
        }

        div { style: "height:150px" }

        div { style: "font-size:1.25rem",
            div { style: "margin:16px",
                Link { to: AppRoute::VoicesPage {},
                    {TEXT.read().home_page_to_voices_page},
                    img { style: "margin-left: 4px", src: FORWARD_12_12 }
                }
            }
            div { style: "margin:16px",
                Link { to: AppRoute::KnowledgePage {},
                    {TEXT.read().home_page_to_knowledge_page},
                    img { style: "margin-left: 4px", src: FORWARD_12_12 }
                }
            }
            if is_content_maintainer {
                div { style: "margin:16px",
                    Link { to: AppRoute::ManagementPage {},
                        {TEXT.read().home_page_to_management_page},
                        img { style: "margin-left: 4px", src: FORWARD_12_12 }
                    }
                }
            }

            div { style: "margin:16px",
                Link { to: AppRoute::SettingPage {},
                    {TEXT.read().home_page_to_setting_page},
                    img { style: "margin-left: 4px", src: FORWARD_12_12 }
                }
            }
            div { style: "margin:16px",
                Link { to: AppRoute::AboutPage {},
                    {TEXT.read().home_page_to_about_page},
                    img { style: "margin-left: 4px", src: FORWARD_12_12 }
                }
            }
            if AndroidInterface::is_connected() {
                div {
                    style: "margin:16px;align_items:center",
                    onclick: move |evt| {
                        AndroidInterface::select();
                        evt.stop_propagation();
                    },
                    {TEXT.read().home_page_connect_to_japan_internet},
                    img {
                        style: "height:24px;width:24px;margin-left: 4px",
                        src: SETTING_BUTTON,
                        onclick: move |evt| {
                            AndroidInterface::config();
                            evt.stop_propagation();
                        }
                    }
                    img {
                        style: "height:24px;width:24px;margin-left: 4px",
                        src: if *is_surfing.read() { SWITCH_ON } else { SWITCH_OFF },

                        onclick: move |evt| {
                            if *is_surfing.read() {
                                AndroidInterface::stop();
                                is_surfing.set(false);
                            } else {
                                AndroidInterface::launch();
                                is_surfing.set(true);
                            }
                            evt.stop_propagation();
                        }
                    }
                }
            }
        }
    }
}
