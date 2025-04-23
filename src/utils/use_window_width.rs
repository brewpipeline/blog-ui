use gloo::utils::window;
use yew::prelude::*;

use yew_hooks::{use_event_with_window, use_mount};

use crate::utils::*;

#[hook]
pub fn use_window_width() -> f64 {
    let state = use_raf_state_eq(|| window().inner_width().unwrap().as_f64().unwrap());

    {
        let state = state.clone();
        use_event_with_window("resize", move |_: Event| {
            state.set(window().inner_width().unwrap().as_f64().unwrap());
        });
    }

    {
        let state = state.clone();
        use_mount(move || {
            state.set(window().inner_width().unwrap().as_f64().unwrap());
        });
    }

    *state
}
