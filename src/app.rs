use yew::prelude::*;
#[cfg(feature = "server")]
use yew_router::history::*;
use yew_router::prelude::*;

use crate::components::body::*;
use crate::components::header::*;
#[cfg(feature = "client")]
use crate::components::search_field::*;
use crate::pages::*;
use crate::utils::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/post/new")]
    NewPost,
    #[at("/post/edit/:id")]
    EditPost { id: u64 },
    #[at("/post/:slug/:id")]
    Post { slug: String, id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/posts/search")]
    PostsSearchRoot,
    #[at("/posts/search/:query")]
    PostsSearch { query: String },
    #[at("/author/:slug")]
    Author { slug: String },
    #[at("/authors")]
    Authors,
    #[at("/authors/search")]
    AuthorsSearchRoot,
    #[at("/authors/search/:query")]
    AuthorsSearch { query: String },
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn switch(route: Route) -> Html {
        match route {
            #[cfg(feature = "client")]
            Route::NewPost => html! { <EditPost id={ None } /> },
            #[cfg(not(feature = "client"))]
            Route::NewPost => html! {},
            #[cfg(feature = "client")]
            Route::EditPost { id } => html! { <EditPost { id } /> },
            #[cfg(not(feature = "client"))]
            Route::EditPost { id: _ } => html! {},
            Route::Post { slug, id } => html! { <Post { slug } { id } /> },
            Route::Home | Route::Posts => html! { <Posts />},
            #[cfg(feature = "client")]
            Route::PostsSearchRoot => {
                html! { <Search mode={ SearchMode::Posts { query: None } } />}
            }
            #[cfg(not(feature = "client"))]
            Route::PostsSearchRoot => html! {},
            #[cfg(feature = "client")]
            Route::PostsSearch { query } => {
                html! { <Search mode={ SearchMode::Posts { query: Some(query) } } />}
            }
            #[cfg(not(feature = "client"))]
            Route::PostsSearch { query: _ } => html! {},
            Route::Author { slug } => html! { <Author { slug } /> },
            Route::Authors => html! { <Authors /> },
            #[cfg(feature = "client")]
            Route::AuthorsSearchRoot => {
                html! { <Search mode={ SearchMode::Authors { query: None } } />}
            }
            #[cfg(not(feature = "client"))]
            Route::AuthorsSearchRoot => html! {},
            #[cfg(feature = "client")]
            Route::AuthorsSearch { query } => {
                html! { <Search mode={ SearchMode::Authors { query: Some(query) } } />}
            }
            #[cfg(not(feature = "client"))]
            Route::AuthorsSearch { query: _ } => html! {},
            Route::NotFound => html! { <PageNotFound /> },
        }
    }
}

#[function_component(Main)]
fn main() -> Html {
    let logged_user = use_reducer(|| Default::default());
    html! {
        <ContextProvider<LoggedUserContext> context={logged_user}>
            <Header />
            <Body />
        </ContextProvider<LoggedUserContext>>
    }
}

#[cfg(feature = "client")]
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Main />
        </BrowserRouter>
    }
}

#[cfg(feature = "client")]
pub fn app_renderer() -> yew::Renderer<impl BaseComponent> {
    let document = gloo::utils::document();
    let element = document.query_selector("#app").unwrap().unwrap();
    yew::Renderer::<App>::with_root(element)
}

#[cfg(feature = "server")]
#[derive(Properties, PartialEq, Eq, Debug)]
struct ServerAppProps {
    pub url: AttrValue,
    pub queries: std::collections::HashMap<String, String>,
}

#[cfg(feature = "server")]
#[function_component(ServerApp)]
fn server_app(props: &ServerAppProps) -> Html {
    let history = gloo_history::AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Main />
        </Router>
    }
}

#[cfg(feature = "server")]
pub fn server_renderer(
    url: String,
    query: std::collections::HashMap<String, String>,
) -> yew::ServerRenderer<impl BaseComponent> {
    yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries: query,
    })
}
