use std::sync::atomic::Ordering;

use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_router::prelude::{use_navigator, Outlet};
use dioxus_use_mounted::use_mounted;
use manganis::ImageAsset;

use crate::global::BUSYING;
use crate::imgs::{LEFT_TOP_NAVIGATION_ITEM_HEIGHT, MORE_MENU, RELOAD_MENU_ITEM};
use crate::refresh_app;
use crate::router::AppRoute;
use crate::storage::setting::SETTING;

pub const TOP_NAVIGATION_HEIGHT: usize = 48;
pub const TOP_NAVIGATION_ITEM_HEIGHT: usize = 24;

//write only
pub(crate) static TOP_NAVIGATION: TopNavigationBox =
    TopNavigationBox(GlobalSignal::new(|| TopNavigation::default()));

pub struct TopNavigationBox(GlobalSignal<TopNavigation>);

impl TopNavigationBox {
    pub fn set_no_padding(&self, value: bool) {
        if self.0.peek().no_padding != value {
            self.0.write_unchecked().no_padding = value;
        }
    }

    pub fn set_no_background(&self, value: bool) {
        if self.0.peek().no_background != value {
            self.0.write_unchecked().no_background = value;
        }
    }

    pub fn set_no_back_button(&self, value: bool) {
        if self.0.peek().no_back_button != value {
            self.0.write_unchecked().no_back_button = value;
        }
    }

    pub fn set_menu_items(&self, value: Vec<Vec<MenuItem>>) {
        if self.0.peek().menu_items != value {
            self.0.write_unchecked().menu_items = value;
        }
    }

    pub fn set_content(&self, value: Option<Element>) {
        if self.0.peek().content != value {
            self.0.write_unchecked().content = value;
        }
    }

    pub fn reset(&self) {
        if *TOP_NAVIGATION.0.peek() != Default::default() {
            *TOP_NAVIGATION.0.write_unchecked() = Default::default();
        }
    }
}

#[derive(Default, PartialEq)]
pub struct TopNavigation {
    pub no_padding: bool,
    pub no_background: bool,
    pub no_back_button: bool,
    pub menu_items: Vec<Vec<MenuItem>>,
    pub content: Option<Element>,
}

pub fn TopNavigation() -> Element {
    let nav = use_navigator();
    let top_navigation_config = TOP_NAVIGATION.0.read();

    let position = if top_navigation_config.no_padding {
        ";position: fixed;"
    } else {
        ";"
    };

    rsx! {
        div { style: "width:100%;height: 100%;display:flex;flex-direction:column",
            div {
                style: "z-index: 1;
                    pointer-events: none;
                    {position}
                    height:{TOP_NAVIGATION_HEIGHT}px;
                    width:100%;
                    display:flex;
                    flex-direction:row;",
                background: if top_navigation_config.no_background == false { "#fff" },
                if top_navigation_config.no_back_button == false {
                    span {
                        style: "z-index: 3;pointer-events: auto;padding: 12px;margin:auto",
                        onclick: move |_| {
                            if *BUSYING.peek() == false {
                                refresh_app();
                                nav.go_back();
                            }
                        },
                        img { src: LEFT_TOP_NAVIGATION_ITEM_HEIGHT }
                    }
                }
                span { style: "z-index: 2;flex:1;margin:auto",
                    if top_navigation_config.content.is_some() {
                        span { style: "z-index: 3;pointer-events: auto;",
                            {top_navigation_config.content.as_ref()}
                        }
                    }
                }

                if top_navigation_config.menu_items.len() > 0 {
                    span { style: "z-index: 3;margin:auto;pointer-events: auto;",
                        Menu { items: top_navigation_config.menu_items.to_owned(), custom_style: "padding: 12px;" }
                    }
                }
            }

            div { style: "flex:1;overflow: auto;", Outlet::<AppRoute> {} }
        }
    }
}

#[derive(Props, PartialEq, Clone, Default)]
pub struct MenuItem {
    pub img: Option<ImageAsset>,
    pub label: &'static str,
    #[props(into)]
    pub onclick: EventHandler<MouseEvent>,
    pub disabled: bool,
}

pub const MENU_ITEM_SIZE: i32 = 12;

fn MenuItemNode(props: MenuItem) -> Element {
    let MenuItem {
        img,
        label,
        onclick,
        disabled,
    } = props;
    let img = if let Some(img) = img {
        rsx! {
            img { style: "height:12px;width:12px;margin-right: 4px", src: img }
        }
    } else {
        rsx! {
            span { style: "height:12px;width:12px;margin-right: 4px;" }
        }
    };

    const PADDING: &str = "padding: 8px;";
    if disabled {
        rsx! {
            div {
                style: PADDING,
                onclick: move |evt| { evt.stop_propagation() },
                {img},
                span { style: "color: gray;{PADDING}", {label} }
            }
        }
    } else {
        rsx! {
            div { style: PADDING, onclick: move |evt| { onclick.call(evt) },
                {img},
                span { style: PADDING, {label} }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct MenuProps {
    items: Vec<Vec<MenuItem>>,
    custom_style: &'static str,
}

fn Menu(props: MenuProps) -> Element {
    let mut show = use_signal(|| false);
    let mut first_group = true;

    let items = {
        if SETTING.read().show_refresh_app_button {
            vec![vec![
                MenuItem {
                    img: Some(RELOAD_MENU_ITEM),
                    label: "refresh app",
                    onclick: EventHandler::new(|_| refresh_app()),
                    disabled: *BUSYING.read(),
                }
            ]]
        } else {
            vec![]
        }.into_iter()
            .chain(props.items.into_iter())
            .map(|items_props| {
                let div = if first_group {
                    first_group = false;
                    None
                } else {
                    rsx! {
                        div { style: "height:1px;width:80%;background-color:#808080;margin-right: auto;margin-left: auto;" }
                    }
                };
                let menu_item_node = items_props.into_iter()
                    .map(|item_props| { MenuItemNode(item_props) });

                rsx! {
                    {div},
                    {menu_item_node}
                }
            })
    };

    let children = if *show.read() {
        rsx! {
            div {
                style: "
                    position: fixed;
                    z-index: 4;
                    left: 0;
                    top: 0;
                    width:100%;
                    height:100%;
                    overflow: auto;
                    background-color: rgb(0,0,0);
                    background-color: rgba(0,0,0,0.4);
                ",
                onclick: move |evt| {
                    show.set(false);
                    evt.stop_propagation();
                },
                span { style: "
                    z-index: 5;
                    top: {TOP_NAVIGATION_HEIGHT}px;
                    right: 8px;
                    position: fixed;
                    background: white;
                    border-width:1px;
                    border-style: solid;
                    border-color: grey;
                    padding:8px;
                    ",
                    {items}
                }
            }
        }
    } else {
        None
    };

    rsx! {
        img {
            src: MORE_MENU,
            style: "background: white;transform: rotate(90deg);{props.custom_style}",
            onclick: move |_| {
                show.set(true);
            }
        }
        {children}
    }
}
