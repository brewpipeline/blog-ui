use gloo::utils::window;
use web_sys::{ScrollBehavior, ScrollToOptions};

pub fn document_scroll_height() -> f64 {
    if let Some(doc) = window().document() {
        let mut h: i32 = 0;
        if let Some(el) = doc.document_element() {
            h = h.max(el.scroll_height());
        }
        if let Some(body) = doc.body() {
            h = h.max(body.scroll_height());
        }
        h as f64
    } else {
        0.0
    }
}

pub fn scroll_to_bottom_instant() {
    let bottom = document_scroll_height();
    let scroll_to_options = ScrollToOptions::new();
    scroll_to_options.set_left(0.0);
    scroll_to_options.set_top(bottom);
    scroll_to_options.set_behavior(ScrollBehavior::Instant);
    window().scroll_to_with_scroll_to_options(&scroll_to_options);
}
