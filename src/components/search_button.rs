use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::search_field::*;
use crate::components::svg_image::*;

use crate::Route;

#[function_component(SearchButton)]
pub fn search_button() -> Html {
    let route = use_route::<Route>().unwrap_or_default();

    let mode = SearchMode::new(route);

    html! {
        <Link<Route> classes="btn btn-light d-block d-lg-none" to={
            match mode {
                SearchMode::Authors { query: _ } => Route::AuthorsSearchRoot,
                SearchMode::Posts { query: _ } => Route::PostsSearchRoot,
            }
         }>
            <SearchImg />
        </Link<Route>>
    }
}
