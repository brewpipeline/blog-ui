use yew::prelude::*;
use yew_router::prelude::*;

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
        </>
    }
}
