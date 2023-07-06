use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::history::*;

use crate::components::body::*;
use crate::components::header::*;
use crate::components::search_field::*;
use crate::pages::*;
use crate::utils::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/posts/:id")]
    Post { id: u64 },
    #[at("/posts")]
    Posts,
    #[at("/posts/search")]
    PostsSearchRoot,
    #[at("/posts/search/:query")]
    PostsSearch { query: String },
    #[at("/authors/:id")]
    Author { id: u64 },
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
            Route::Post { id: post_id } => html! { <Post { post_id } /> },
            Route::Home | Route::Posts => html! { <Posts />},
            Route::PostsSearchRoot => {
                html! { <Search mode={ SearchMode::Posts { query: None } } />}
            }
            Route::PostsSearch { query } => {
                html! { <Search mode={ SearchMode::Posts { query: Some(query) } } />}
            }
            Route::Author { id: user_id } => html! { <Author { user_id } /> },
            Route::Authors => html! { <Authors /> },
            Route::AuthorsSearchRoot => {
                html! { <Search mode={ SearchMode::Authors { query: None } } />}
            }
            Route::AuthorsSearch { query } => {
                html! { <Search mode={ SearchMode::Authors { query: Some(query) } } />}
            }
            Route::NotFound => html! { <PageNotFound /> },
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let logged_user = use_reducer(|| Default::default());
    html! {
        <BrowserRouter>
            <ContextProvider<LoggedUserContext> context={logged_user}>
                <Header />
                <Body />
            </ContextProvider<LoggedUserContext>>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: std::collections::HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

        let logged_user = use_reducer(|| Default::default());
        html! {
            <Router history={history}>
                <ContextProvider<LoggedUserContext> context={logged_user}>
                    <Header />
                    <Body />
                </ContextProvider<LoggedUserContext>>
            </Router>
        }
}
