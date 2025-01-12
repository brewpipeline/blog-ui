use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::delayed_component::*;
use crate::utils::*;

use crate::Route;

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    let route = use_route::<Route>().unwrap_or_default();
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();
    let is_editor = !logged_user_context.is_not_inited()
        && logged_user_context
            .author()
            .map(|a| a.editor == 1)
            .unwrap_or(false);
    let is_tg_button_available = if !logged_user_context.is_not_inited() {
        logged_user_context.token() == None
    } else {
        false
    };

    #[cfg(feature = "telegram")]
    let tg_button: Option<Html> = if is_tg_button_available {
        Some(html! {
            <button
                title="Войти через Telegram"
                aria-label="Войти через Telegram"
                type="button"
                data-bs-toggle="modal"
                data-bs-target="#loginModal"
                class="btn btn-light telegram-button"
            >
                <div class="btn inner">
                    <p>
                        <span>{ "Войти через" }</span>
                        <br/>
                        <span>{ "Telegram" }</span>
                    </p>
                </div>
            </button>
        })
    } else {
        None
    };
    #[cfg(not(feature = "telegram"))]
    let tg_button: Option<Html> = None;

    html! {
        <>
            <Link<Route>
                classes={
                    classes!(
                        "btn",
                        "btn-light",
                        if route == Route::Posts { "active" } else { "" }
                    )
                }
                to={ Route::Posts }
            >
                { "Публикации" }
            </Link<Route>>
            <Link<Route>
                classes={
                    classes!(
                        "btn",
                        "btn-light",
                        if route == Route::Authors { "active" } else { "" }
                    )
                }
                to={ Route::Authors }
            >
                { "Авторы" }
            </Link<Route>>
            <Link<Route>
                classes={
                    classes!(
                        "btn",
                        "btn-light",
                        if route == Route::NewPost { "active" } else { "" }
                    )
                }
                to={ Route::NewPost }
            >
                { "Новая публикация" }
            </Link<Route>>
            if is_editor {
                <Link<Route>
                    classes={
                        classes!(
                            "btn",
                            "btn-light",
                            if route == Route::UnpublishedPosts { "active" } else { "" }
                        )
                    }
                    to={ Route::UnpublishedPosts }
                >
                    { "Неопубликованное" }
                </Link<Route>>
                <Link<Route>
                    classes={
                        classes!(
                            "btn",
                            "btn-light",
                            if route == Route::HiddenPosts { "active" } else { "" }
                        )
                    }
                    to={ Route::HiddenPosts }
                >
                    { "Скрытое" }
                </Link<Route>>
            }
            // Workaround: to avoid blink on hydrate
            <DelayedComponent<Option<Html>> component={ move |tg_button| html! {
                <>
                    { tg_button }
                </>
            } } deps={ tg_button.clone() } />
        </>
    }
}
