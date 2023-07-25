use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NavigationMenu)]
pub fn navigation_menu() -> Html {
    let route = use_route::<Route>().unwrap_or_default();

    html! {
        <div class="d-grid gap-2">
            <Link<Route>
                classes={
                    classes!(
                        "btn",
                        "btn-light",
                        if route == Route::Posts || route == Route::Home { "active" } else { "" }
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
                { "Новая публикации" }
            </Link<Route>>
        </div>
    }
}
