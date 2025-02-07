use dioxus::core_macro::rsx;
use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;
use tracing::debug;

use crate::global::new_id;
use crate::window::WINDOW_HEIGHT;

#[derive(Props, Clone)]
pub struct LazyListProps<F: 'static + Fn(usize) -> Option<Element>> {
    #[props(into)]
    container_height: f64,
    #[props(into)]
    item_height: f64,
    estimate_item_count: Option<usize>,
    make_item: F,
}

impl<F: 'static + Fn(usize) -> Option<Element>> PartialEq for LazyListProps<F> {
    fn eq(&self, other: &Self) -> bool {
        self.container_height == other.container_height
            && self.item_height == other.item_height
            && self.estimate_item_count == other.estimate_item_count
    }
}

pub fn LazyList<F: 'static + Fn(usize) -> Option<Element>>(props: LazyListProps<F>) -> Element {
    let container_height = props.container_height;
    let default_height = props.item_height;

    let id = use_memo(new_id);

    let get_scroll_top = move || {
        js_sys::eval(format!("document.querySelector('#{id}').scrollTop").as_str())
            .map(|result| result.as_f64())
            .unwrap_or(Some(0.))
            .unwrap_or(0.)
    };

    let mut item_count = use_signal(|| props.estimate_item_count.unwrap_or(0usize));

    let mut scroll_top = use_signal(get_scroll_top);
    let mut start_index = use_memo(move || (*scroll_top.read() / default_height).floor() as usize);
    let end_index = use_memo(move || {
        let container_height = if container_height > 0f64 {
            container_height
        } else {
            WINDOW_HEIGHT.peek().unwrap_or(0f64)
        };
        start_index() + (container_height / default_height).ceil() as usize + 4
    });

    let first_element_mounted = use_mounted();
    let first_element_height = use_size(first_element_mounted).height();

    let first_element_position = use_memo(move || {
        if first_element_height > 0f64 {
            ((start_index() as f64) * default_height) + (scroll_top() % default_height)
                - (((scroll_top() % default_height) / default_height) * first_element_height)
        } else {
            (start_index() as f64) * default_height
        }
    });

    let mut items = Vec::with_capacity(end_index() + 1 - start_index());
    for idx in (start_index()..=end_index()).into_iter() {
        if let Some(ele) = (props.make_item)(idx) {
            items.push(if idx == start_index() {
                rsx! {
                    div {
                        id: "{id}-item-{idx}",
                        onmounted: move |event| first_element_mounted.onmounted(event),
                        {ele}
                    }
                }
            } else {
                rsx! {
                    div { id: "{id}-item-{idx}", {ele} }
                }
            });

            let update = { idx > *item_count.peek() };
            if update {
                *item_count.write() = idx;
            }
        } else {
            let idx = idx.max(1) - 1;
            let update = { idx > *item_count.peek() };
            if update {
                *item_count.write() = idx;
            }
            break;
        }
    }

    // debug!("\nitem_count:{}", item_count);
    // debug!("\nstart_index:{}", start_index);
    // debug!("\nend_index:{}", end_index);
    // debug!("\ncontainer_height:{}", container_height);
    // debug!("\ndefault_height:{}", default_height);

    let padding = (default_height * (*item_count.read() as f64))
        + ((container_height / default_height).max(1.0) - 1.0).floor() * default_height;

    rsx! {
        div {
            style: "height:{container_height}px; overflow:auto; position:relative",
            id,
            onscroll: move |evet| {
                *scroll_top.write() = get_scroll_top();
            },
            div { style: "height:{padding}px" }
            div { style: "position:absolute;top:{first_element_position}px;width: 100%",
                {items.into_iter()}
            }
        }
    }
}
