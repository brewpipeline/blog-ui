mod components;
mod content;
mod pages;
mod keyed_reducible;

use std::rc::Rc;

use content::AuthUser;
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_cookies::CookieOptions;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::HtmlDocument;

use pages::page_not_found::PageNotFound;

use crate::components::author_card::AuthorCard;
use crate::components::body::Body;
use crate::components::comment_card::CommentCard;
use crate::components::header::Header;
use crate::components::item::Item;
use crate::components::list::List;
use crate::components::post_card::PostCard;
use crate::content::{PostsContainer, UsersContainer, User, Post, CommentsContainer, CommentsContainerUrlProps};
use crate::keyed_reducible::UseKeyedReducerHandle;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoggedUser {
    pub auth_user: Option<AuthUser>,
}

impl LoggedUser {
    #[cfg(target_arch = "wasm32")]
    fn load_auth_user() -> Option<AuthUser> {
        let cookie = wasm_cookies::get("LoggedUser")?.ok()?;
        let auth_user: AuthUser = serde_json::from_str(cookie.as_str()).ok()?;
        Some(auth_user)
    }

    #[cfg(target_arch = "wasm32")]
    fn save_auth_user(auth_user: &Option<AuthUser>) -> Option<()> {
        if let Some(auth_user) = &auth_user {
            let auth_user_string = serde_json::to_string(auth_user).ok()?;
            wasm_cookies::set("LoggedUser", &auth_user_string, &CookieOptions::default());
        } else {
            wasm_cookies::delete("LoggedUser")
        }
        Some(())
    }
}

impl Default for LoggedUser {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let auth_user = Self::load_auth_user();
        #[cfg(not(target_arch = "wasm32"))]
        let auth_user = None;
        Self { auth_user }
    }
}

impl Reducible for LoggedUser {
    type Action = Option<AuthUser>;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        #[cfg(target_arch = "wasm32")]
        Self::save_auth_user(&action);
        LoggedUser { auth_user: action }.into()
    }
}

pub type LoggedUserContext = UseReducerHandle<LoggedUser>;

#[function_component(App)]
pub fn app() -> Html {
    let logged_user  = use_reducer(|| Default::default());
    let posts_cache = use_reducer(|| Default::default());
    let users_cache = use_reducer(|| Default::default());
    html! {
        <BrowserRouter>
            <ContextProvider<LoggedUserContext> context={logged_user}>
                <ContextProvider<UseKeyedReducerHandle<u64, Post>> context={posts_cache}>
                    <ContextProvider<UseKeyedReducerHandle<u64, User>> context={users_cache}>
                        <Header />
                        <Body />
                    </ContextProvider<UseKeyedReducerHandle<u64, User>>>
                </ContextProvider<UseKeyedReducerHandle<u64, Post>>>
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
