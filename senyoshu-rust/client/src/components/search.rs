use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use senyoshu_common::util::alias::alias_to_standard;

use crate::imgs::FIND_IMG;
use crate::page::glossary_page::GlossaryFilter;
use crate::router::AppRoute;
use crate::singleton::top_navigation::TOP_NAVIGATION_HEIGHT;

#[derive(Props, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct SearchComponentProps {
    kw: Option<String>,
    set: Option<String>,
}

pub fn SearchComponent(props: SearchComponentProps) -> Element {
    let kw = props
        .kw
        .as_ref()
        .map(|kw| kw.chars().map(|c| alias_to_standard(c)).collect::<String>())
        .unwrap_or_default();

    let set = props.set.to_owned();

    let nav = use_navigator();

    rsx! {
        form {
            style: "display:flex;align-items: center;justify-content: center;height:{TOP_NAVIGATION_HEIGHT}px",
            onsubmit: move |event| {
                let kw = event
                    .values()
                    .iter()
                    .find(|(k, _)| { k.as_str() == "kw" })
                    .map(|(_, kw)| { kw.iter().next().map(|kw| kw.to_string()) })
                    .flatten()
                    .unwrap_or_default();
                nav.push(AppRoute::GlossaryPage {
                    filter: GlossaryFilter {
                        kw: Some(kw.chars().map(|c| alias_to_standard(c)).collect::<String>()),
                        set: set.to_owned(),
                    },
                    order: Default::default(),
                });
            },

            input {
                style: "z-index: 2;width:200px",
                r#type: "text",
                name: "kw",
                value: "{kw}"
            }

            input {
                style: "z-index: 2;width:20px;height:20px;padding: 6px;user-select: none",
                r#type: "image",
                alt: "search",
                src: FIND_IMG
            }
        }
    }
}
