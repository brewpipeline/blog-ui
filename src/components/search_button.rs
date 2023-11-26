use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::search_field::*;

use crate::Route;

#[function_component(SearchButton)]
pub fn search_button() -> Html {
    let navigator = use_navigator().unwrap();

    let route = use_route::<Route>().unwrap_or_default();

    let onclick = {
        let navigator = navigator.clone();
        let route = route.clone();

        Callback::from(move |e: MouseEvent| {
            if e.meta_key() || e.ctrl_key() || e.shift_key() || e.alt_key() {
                return;
            }
            e.prevent_default();
            
            navigator.push(&match SearchMode::new(route.clone()) {
                SearchMode::Authors { query: _ } => Route::AuthorsSearchRoot,
                SearchMode::Posts { query: _ } => Route::PostsSearchRoot,
            });
        })
    };

    html! {
        <button title="Поиск" class="btn btn-light d-block d-lg-none" { onclick }>
            <i class="bi bi-search"></i>
        </button>
    }
}
