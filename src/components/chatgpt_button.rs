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
    let route = use_route::<Route>();
    let on_chat = matches!(route, Some(Route::ChatGPT));

    if on_chat {
        return html! {
            <button type="button" class={classes!("btn","btn-purple", "disabled", props.classes.clone())} disabled=true title="ChatGPT">
                <i class="bi bi-openai rotate" title="ChatGPT"></i>
            </button>
        };
    }

    html! {
        <Link<Route> to={Route::ChatGPT} classes={classes!("btn","btn-purple", props.classes.clone())}>
            <i class="bi bi-openai" title="ChatGPT"></i>
        </Link<Route>>
    }
}
