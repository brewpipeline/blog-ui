use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct ChatGptButtonProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(ChatGptButton)]
pub fn chatgpt_button(props: &ChatGptButtonProps) -> Html {
    let navigator = use_navigator().unwrap();
    let route = use_route::<Route>();
    let on_chat = matches!(route, Some(Route::ChatGPT));

    // Compute click handler (noop when already on ChatGPT)
    let onclick = {
        if on_chat {
            Callback::from(|_| ())
        } else {
            let navigator = navigator.clone();
            Callback::from(move |e: MouseEvent| {
                if e.meta_key() || e.ctrl_key() || e.shift_key() || e.alt_key() {
                    return;
                }
                e.prevent_default();
                navigator.push(&Route::ChatGPT);
            })
        }
    };

    // Build button classes and icon classes once
    let mut btn_classes = classes!("btn", "btn-purple");
    btn_classes.extend(props.classes.clone());
    if on_chat { btn_classes.push("disabled"); }

    let mut icon_classes = classes!("bi", "bi-openai");
    if on_chat { icon_classes.push("rotate"); }

    html! {
        <button type={"button"} class={btn_classes} {onclick} disabled={on_chat} title={"ChatGPT"}>
            <i class={icon_classes}></i>
        </button>
    }
}
