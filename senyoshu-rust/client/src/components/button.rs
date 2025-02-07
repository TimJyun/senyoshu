use dioxus::core_macro::rsx;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(into)]
    pub onclick: EventHandler<MouseEvent>,
    pub disabled: Option<bool>,
    #[props(into)]
    pub custom_style: Option<String>,
    #[props(into)]
    pub children: Element,
}

pub fn Button(props: ButtonProps) -> Element {
    const STYLE: &str =
        "display:inline-block;padding: 4px;border: 1px;border-style: solid;border-radius: 4px;user-select: none;";
    let custom_style = props.custom_style.unwrap_or_default();

    if let Some(true) = props.disabled {
        rsx! {
            span { style: "{custom_style};{STYLE};color: gray", {props.children} }
        }
    } else {
        rsx! {
            span {
                style: "{custom_style};{STYLE}",
                onclick: move |evt| { props.onclick.call(evt) },
                {props.children}
            }
        }
    }
}
