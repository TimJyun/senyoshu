#![allow(non_snake_case)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use tracing::{debug, Level};

use window::add_window_size_change_listener;

use crate::router::AppRoute;
use crate::singleton::confirm_box::ConfirmBox;
use crate::singleton::top_navigation::TOP_NAVIGATION;
use crate::storage::update;
use crate::task::TASK_DEQUE;

pub mod components;
pub mod file;
pub mod global;
pub mod i18n;
pub mod imgs;
pub mod native;
pub mod page;
pub mod request;
pub mod router;
pub mod singleton;
pub mod storage;
pub mod task;
pub mod text;
mod tts;
pub mod voice;
pub mod window;

// static BODY_HEIGHT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    if let Err(_) = dioxus_logger::init(Level::DEBUG) {
        let _ = js_sys::eval("console.log('logger failed to init');");
    } else {
        debug!("logger init success");
    };

    #[cfg(feature = "android")]
    {
        //禁止手机缩放 && 修复 android webview size 不正确的问题
        let _ = js_sys::eval("
            (function(){
                let meta_tags = document.getElementsByTagName('meta');
                let meta_viewport = meta_tags.namedItem('viewport');
                meta_viewport.content = 'width=device-width, initial-scale=1.0, maximum-scale=1.0,minimum-scale=1.0, user-scalable=0';
            })();
           (function(){
                document.querySelector('html').style.height = window.innerHeight + 'px';
            })();
        ");
    }

    launch(App);
}

static TO_BE_REFRESHED: GlobalSignal<bool> = Signal::global(|| false);

pub fn refresh_app() {
    *TO_BE_REFRESHED.write() = true;
}

pub fn App() -> Element {
    let _ = use_coroutine(|_rx: UnboundedReceiver<()>| async {
        add_window_size_change_listener();
        update().await;
        loop {
            TASK_DEQUE.exec();
            sleep(Duration::from_millis(10)).await;
        }
    });

    let to_be_refreshed = *TO_BE_REFRESHED.read();
    if to_be_refreshed {
        *TO_BE_REFRESHED.write() = false;
        return None;
    }

    let config = || {
        RouterConfig::<AppRoute>::default().on_update(|ctx| {
            let mut current_route = CURRENT_ROUTE.lock().unwrap();
            if *current_route != ctx.current() {
                debug!("Route: on change");
                *current_route = ctx.current();
                TOP_NAVIGATION.reset();
            }
            None
        })
    };

    rsx! {
        Router::<AppRoute> { config }
        ConfirmBox {}
    }
}

static CURRENT_ROUTE: Mutex<AppRoute> = Mutex::new(AppRoute::PageNotFound { route: vec![] });
