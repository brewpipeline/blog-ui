mod components;
mod content;
mod pages;
mod hash_map_context;
mod logged_user_context;

#[macro_use]
extern crate async_trait;

use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlDocument;

use crate::pages::page_not_found::PageNotFound;
use crate::components::author_card::AuthorCard;
use crate::components::body::Body;
use crate::components::comment_card::CommentCard;
use crate::components::header::Header;
use crate::components::item::Item;
use crate::components::list::List;
use crate::components::post_card::PostCard;
use crate::content::{PostsContainer, UsersContainer, User, Post, CommentsContainer, CommentsContainerUrlProps};
use crate::hash_map_context::HashMapContext;
use crate::logged_user_context::LoggedUserContext;

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
            Route::Post { id: post_id } => html! {
                <>
                    <Item<Post> 
                        item_id={ post_id } 
                        component={ |post| html! { <PostCard { post } 
                        fetch_author={ true } /> } } 
                    />
                    <List<CommentsContainer> 
                        url_props={ CommentsContainerUrlProps { post_id: Some(post_id) } } 
                        items_per_page={ 100 } 
                        route_to_page={ Route::Post { id: post_id } } 
                        component={ |comment| html! { <CommentCard { comment } /> } } 
                    />
                </>
            },
            Route::Home | Route::Posts => html! { 
                <List<PostsContainer> 
                    route_to_page={ Route::Posts } 
                    component={ |post| html! { <PostCard { post } /> } } 
                /> 
            },
            Route::Author { id: user_id } => html! { 
                <Item<User> 
                    item_id={ user_id } 
                    component={ |user| html! { <AuthorCard { user } /> } } 
                /> 
            },
            Route::Authors => html! { 
                <List<UsersContainer> 
                    route_to_page={ Route::Authors } 
                    component={ |user| html! { <AuthorCard { user } /> } } 
                /> 
            },
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
                <ContextProvider<HashMapContext<u64, Post>> context={posts_cache}>
                    <ContextProvider<HashMapContext<u64, User>> context={users_cache}>
                        <Header />
                        <Body />
                    </ContextProvider<HashMapContext<u64, User>>>
                </ContextProvider<HashMapContext<u64, Post>>>
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
