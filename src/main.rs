mod components;
mod content;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::page_not_found::PageNotFound;

use crate::components::author_card::AuthorCard;
use crate::components::body::Body;
use crate::components::comment_card::CommentCard;
use crate::components::header::Header;
use crate::components::item::Item;
use crate::components::list::List;
use crate::components::post_card::PostCard;
use crate::content::{PostsContainer, UsersContainer, User, Post, CommentsContainer, CommentsContainerUrlProps};

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
                let url_props = CommentsContainerUrlProps {
                    post_id: Some(id),
                };
                html! {
                    <>
                        <Item<Post> { id } component={ |post| html! { <PostCard { post } fetch_author={true} /> } } />
                        <List<CommentsContainer> { url_props } items_per_page={100} route_to_page={ Route::Post { id } } component={ |comment| html! { <CommentCard { comment } /> } } />
                    </>
                }
            }
            Route::Home | Route::Posts => {
                html! { <List<PostsContainer> route_to_page={ Route::Posts } component={ |post| html! { <PostCard { post } /> } } /> }
            }
            Route::Author { id } => {
                html! { <Item<User> { id } component={ |user| html! { <AuthorCard { user } /> } } /> }
            }
            Route::Authors => {
                html! { <List<UsersContainer> route_to_page={ Route::Authors } component={ |user| html! { <AuthorCard { user } /> } } /> }
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
