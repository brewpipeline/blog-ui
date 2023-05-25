mod components;
mod content;
mod pages;
mod utils;

#[macro_use]
extern crate async_trait;

use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlDocument;
use wasm_bindgen::JsCast;

use crate::pages::*;
use crate::components::body::*;
use crate::components::header::*;
use crate::utils::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/authors/:id")]
    Author { id: u64 },
    #[at("/authors")]
    Authors,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    fn switch(route: Route) -> Html {
        match route {
            Route::Post { id: post_id } => html! { <Post { post_id } /> },
            Route::Home | Route::Posts => html! { <Posts />},
            Route::Author { id: user_id } => html! { <Author { user_id } /> },
            Route::Authors => html! { <Authors /> },
            Route::NotFound => html! { <PageNotFound /> }
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let logged_user  = use_reducer(|| Default::default());
    let posts_cache = use_reducer(|| Default::default());
    let users_cache = use_reducer(|| Default::default());
    html! {
        <BrowserRouter>
            <ContextProvider<LoggedUserContext> context={logged_user}>
                <Header />
                <ContextProvider<HashMapContext<u64, content::Post>> context={posts_cache}>
                    <ContextProvider<HashMapContext<u64, content::User>> context={users_cache}>
                        <Body />
                    </ContextProvider<HashMapContext<u64, content::User>>>
                </ContextProvider<HashMapContext<u64, content::Post>>>
            </ContextProvider<LoggedUserContext>>
        </BrowserRouter>
    }
}

pub fn html_document() -> HtmlDocument {
    gloo::utils::document().unchecked_into::<HtmlDocument>()
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    wasm_logger::init(wasm_logger::Config::default());

    let document = gloo::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::Renderer::<App>::with_root(element).render();
}
