use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod content;
mod generator;
mod pages;
use pages::author::Author;
use pages::author_list::AuthorList;
use pages::page_not_found::PageNotFound;
use pages::post::Post;
use pages::post_list::PostList;

use crate::components::body::Body;
use crate::components::header::Header;

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
            Route::Post { id } => {
                html! { <Post {id} /> }
            }
            Route::Home | Route::Posts => {
                html! { <PostList /> }
            }
            Route::Author { id } => {
                html! { <Author {id} /> }
            }
            Route::Authors => {
                html! { <AuthorList /> }
            }
            Route::NotFound => {
                html! { <PageNotFound /> }
            }
        }
    }
}

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Header />
                <Body />
            </BrowserRouter>
        }
    }
}

fn main() {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    wasm_logger::init(wasm_logger::Config::default());

    let document = gloo::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::Renderer::<App>::with_root(element).render();
}
