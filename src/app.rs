use yew::prelude::*;
#[cfg(feature = "server")]
use yew_router::history::*;
#[cfg(any(feature = "client", feature = "server"))]
use yew_router::prelude::*;

use crate::components::body::*;
use crate::components::header::*;
use crate::components::meta::*;
use crate::utils::*;

#[derive(Properties, PartialEq, Debug)]
struct MainProps {
    app_content: Option<AppContent>,
}

#[function_component(Main)]
fn main(props: &MainProps) -> Html {
    let app_content_container = use_reducer_eq(|| AppContentContainer {
        is_used: false,
        app_content: props.app_content.clone(),
    });
    let logged_user = use_reducer_eq(|| LoggedUser {
        state: LoggedUserState::None,
    });
    html! {
        <>
            <script
                id="page-content"
                type={ app_content_container.app_content.as_ref().map(|c| c.r#type.clone()) }
            >
                { app_content_container.app_content.as_ref().map(|c| c.value.clone()) }
            </script>
            <Meta />
            <ContextProvider<AppContentContext> context={ app_content_container }>
                <ContextProvider<LoggedUserContext> context={ logged_user }>
                    <Header />
                    <Body />
                </ContextProvider<LoggedUserContext>>
            </ContextProvider<AppContentContext>>
        </>
    }
}

#[cfg(feature = "client")]
#[function_component(App)]
fn app() -> Html {
    let app_content = gloo::utils::document()
        .query_selector("#page-content")
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
        <Router { history }>
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
