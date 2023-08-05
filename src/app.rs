use yew::prelude::*;
#[cfg(feature = "server")]
use yew_router::history::*;
use yew_router::prelude::*;

use crate::components::body::*;
use crate::components::header::*;
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
            Route::NewPost => html! { <EditPost id={ None } /> },
            Route::EditPost { id } => html! { <EditPost { id } /> },
            Route::Post { slug, id } => html! { <Post { slug } { id } /> },
            Route::Home | Route::Posts => html! { <Posts />},
            Route::PostsSearchRoot => {
                html! { <Search mode={ SearchMode::Posts { query: None } } />}
            }
            Route::PostsSearch { query } => {
                html! { <Search mode={ SearchMode::Posts { query: Some(query) } } />}
            }
            Route::Author { slug } => html! { <Author { slug } /> },
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

#[derive(Properties, PartialEq, Debug)]
struct MainProps {
    app_content: Option<AppContent>,
}

#[function_component(Main)]
fn main(props: &MainProps) -> Html {
    let app_meta = use_reducer_eq(|| AppMeta::default());
    let app_content_container = use_reducer_eq(|| AppContentContainer {
        is_used: false,
        app_content: props.app_content.clone(),
    });
    let logged_user = use_reducer_eq(|| LoggedUser {
        state: LoggedUserState::None,
    });
    html! {
        <>
            <script id="app-title" type="text/plain"> { app_meta.title.clone() } </script>
            <script id="app-description" type="text/plain"> { app_meta.description.clone() } </script>
            <script id="app-keywords" type="text/plain"> { app_meta.keywords.clone() } </script>
            <script
                id="app-content"
                type={ app_content_container.app_content.as_ref().map(|c| c.r#type.clone()) }
            >
                { app_content_container.app_content.as_ref().map(|c| c.value.clone()) }
            </script>
            <ContextProvider<AppMetaContext> context={ app_meta }>
                <ContextProvider<AppContentContext> context={ app_content_container }>
                    <ContextProvider<LoggedUserContext> context={logged_user}>
                        <Header />
                        <Body />
                    </ContextProvider<LoggedUserContext>>
                </ContextProvider<AppContentContext>>
            </ContextProvider<AppMetaContext>>
        </>
    }
}

#[cfg(feature = "client")]
#[function_component(App)]
fn app() -> Html {
    let app_content = gloo::utils::document()
        .query_selector("#app-content")
        .ok()
        .flatten()
        .map(|e| {
            let (
                "script",
                Some(r#type),
                Some(value)
            ) = (
                e.tag_name().to_lowercase().as_str(),
                e.get_attribute("type"),
                e.text_content().map(|s| s.trim().to_owned())
            ) else {
                return None
            };
            Some(AppContent { r#type, value })
        })
        .flatten();
    html! {
        <BrowserRouter>
            <Main { app_content } />
        </BrowserRouter>
    }
}

#[cfg(feature = "client")]
pub fn app_renderer() -> yew::Renderer<impl BaseComponent> {
    let element = gloo::utils::document()
        .query_selector("#app")
        .unwrap()
        .unwrap();
    yew::Renderer::<App>::with_root(element)
}

#[cfg(feature = "server")]
#[derive(Clone, Debug, PartialEq, Eq, Properties)]
struct ServerAppProps {
    pub url: AttrValue,
    pub queries: std::collections::HashMap<String, String>,
    pub app_content: Option<AppContent>,
}

#[cfg(feature = "server")]
#[function_component(ServerApp)]
fn server_app(props: &ServerAppProps) -> Html {
    let history = gloo_history::AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();
    let app_content = props.app_content.clone();
    html! {
        <Router history={history}>
            <Main { app_content } />
        </Router>
    }
}

#[cfg(feature = "server")]
pub fn server_renderer(
    url: String,
    query: std::collections::HashMap<String, String>,
    app_content: Option<AppContent>,
) -> yew::ServerRenderer<impl BaseComponent> {
    yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries: query,
        app_content,
    })
}
