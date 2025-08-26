use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::search_field::*;
use crate::pages::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/settings")]
    Settings,
    #[at("/post/new")]
    NewPost,
    #[at("/post/edit/:id")]
    EditPost { id: u64 },
    #[at("/post/:slug/:id")]
    Post { slug: String, id: u64 },
    #[at("/")]
    Posts,
    #[at("/posts/search")]
    PostsSearchRoot,
    #[at("/posts/search/:query")]
    PostsSearch { query: String },
    #[at("/posts/unpublished")]
    UnpublishedPosts,
    #[at("/posts/my/unpublished")]
    MyUnpublishedPosts,
    #[at("/posts/hidden")]
    HiddenPosts,
    #[at("/tag/:slug/:id")]
    Tag { slug: String, id: u64 },
    #[at("/author/:slug")]
    Author { slug: String },
    #[at("/authors")]
    Authors,
    #[at("/authors/search")]
    AuthorsSearchRoot,
    #[at("/authors/search/:query")]
    AuthorsSearch { query: String },
    #[at("/chat")]
    Chat,
    #[cfg(feature = "yandex")]
    #[at("/yandexToken")]
    YandexToken,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn recognize_path(pathname: &str) -> Option<Self> {
        Self::recognize(pathname)
    }
    pub(crate) fn is_search(&self) -> bool {
        match self {
            Route::PostsSearchRoot
            | Route::PostsSearch { query: _ }
            | Route::AuthorsSearchRoot
            | Route::AuthorsSearch { query: _ } => true,
            Route::Settings
            | Route::NewPost
            | Route::EditPost { id: _ }
            | Route::Post { slug: _, id: _ }
            | Route::Posts
            | Route::UnpublishedPosts
            | Route::MyUnpublishedPosts
            | Route::HiddenPosts
            | Route::Tag { slug: _, id: _ }
            | Route::Author { slug: _ }
            | Route::Authors
            | Route::Chat
            | Route::NotFound => false,
            #[cfg(feature = "yandex")]
            Route::YandexToken => false,
        }
    }
    pub(crate) fn switch(route: Route) -> Html {
        match route {
            Route::Settings => html! { <Settings /> },
            Route::NewPost => html! { <EditPost id={ None } /> },
            Route::EditPost { id } => html! { <EditPost { id } /> },
            Route::Post { slug, id } => html! { <Post { slug } { id } /> },
            Route::Posts => html! { <Posts /> },
            Route::PostsSearchRoot => {
                html! { <Search mode={ SearchMode::Posts { query: None } } /> }
            }
            Route::PostsSearch { query } => {
                html! { <Search mode={ SearchMode::Posts { query: Some(query) } } /> }
            }
            Route::UnpublishedPosts => html! { <UnpublishedPosts /> },
            Route::MyUnpublishedPosts => html! { <MyUnpublishedPosts /> },
            Route::HiddenPosts => html! { <HiddenPosts /> },
            Route::Tag { slug, id } => html! { <Tag { slug } { id } /> },
            Route::Author { slug } => html! { <Author { slug } /> },
            Route::Authors => html! { <Authors /> },
            Route::AuthorsSearchRoot => {
                html! { <Search mode={ SearchMode::Authors { query: None } } /> }
            }
            Route::AuthorsSearch { query } => {
                html! { <Search mode={ SearchMode::Authors { query: Some(query) } } /> }
            }
            Route::Chat => html! { <AiChatPage /> },
            #[cfg(feature = "yandex")]
            Route::YandexToken => unreachable!(),
            Route::NotFound => html! { <PageNotFound /> },
        }
    }
}
