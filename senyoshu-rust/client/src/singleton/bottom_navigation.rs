use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_router::prelude::{Link, Outlet};
use dioxus_use_mounted::use_mounted;

use crate::imgs::{
    BOOK_BOTTOM_NAVIGATION_ITEM, CUSTOMER_BOTTOM_NAVIGATION_ITEM,
    SHELF_BOTTOM_NAVIGATION_ITEM_HEIGHT,
};
use crate::router::AppRoute;

pub const BOTTOM_NAVIGATION_HEIGHT: usize = 64;
pub const BOTTOM_NAVIGATION_ITEM_HEIGHT: usize = 32;

pub fn BottomNavigation() -> Element {
    let buttons = [
        (
            AppRoute::LearnPage {},
            rsx! {
                img { src: BOOK_BOTTOM_NAVIGATION_ITEM }
            },
        ),
        (
            AppRoute::CollectionPage {},
            rsx! {
                img { src: SHELF_BOTTOM_NAVIGATION_ITEM_HEIGHT }
            },
        ),
        (
            AppRoute::HomePage {},
            rsx! {
                img { src: CUSTOMER_BOTTOM_NAVIGATION_ITEM }
            },
        ),
    ]
    .into_iter()
    .map(|(href, label)| {
        rsx! {
            Link { to: href, style: "flex:1;display: inline-block;text-align: center;", {label} }
        }
    });

    rsx! {
        div { style: "width:100%;height: 100%;display:flex;flex-direction:column;",
            div { style: "flex:1;overflow: auto;", Outlet::<AppRoute> {} }
            div { style: "
                    background-color: #fff;
                    height:{BOTTOM_NAVIGATION_HEIGHT}px;
                    width:100%;
                    display:flex;
                    align-items: center;
                    ",
                {buttons}
            }
        }
    }
}
