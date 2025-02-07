use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::{GlobalSignal, Signal};

pub static BUSYING: GlobalSignal<bool> = Signal::global(|| false);

pub static MAX_ID: AtomicUsize = AtomicUsize::new(0);

pub fn new_id() -> String {
    let id = MAX_ID.fetch_add(1, Ordering::Acquire);
    format!("senyoshu-id-{id}")
}
