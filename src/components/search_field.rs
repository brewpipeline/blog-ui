use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchMode {
    Authors { query: Option<String> },
    Posts { query: Option<String> },
}

impl SearchMode {
    pub fn new(route: Route) -> Self {
        match route {
            Route::PostsSearch { query } => Self::Posts { query: Some(query) },
            Route::AuthorsSearch { query } => Self::Authors { query: Some(query) },
            Route::Home
            | Route::NotFound
            | Route::PostsSearchRoot
            | Route::NewPost
            | Route::Post { slug: _ }
            | Route::Posts => Self::Posts { query: None },
            Route::AuthorsSearchRoot | Route::Author { slug: _ } | Route::Authors => {
                Self::Authors { query: None }
            }
        }
    }
    pub fn decoded_query(&self) -> Option<String> {
        let query = match self {
            Self::Authors { query } | Self::Posts { query } => query,
        };
        query
            .clone()
            .map(|q| urlencoding::decode(q.as_str()).map(|c| c.into_owned()).ok())
            .flatten()
    }
    pub fn encode_new_query_and_route(&self, query: String) -> Route {
        let query = urlencoding::encode(query.as_str()).into_owned();
        match self {
            Self::Authors { query: _ } => Route::AuthorsSearch { query },
            Self::Posts { query: _ } => Route::PostsSearch { query },
        }
    }
    pub fn placeholder(&self) -> &'static str {
        match self {
            Self::Authors { query: _ } => "Поиск авторов...",
            Self::Posts { query: _ } => "Поиск публикаций...",
        }
    }
    pub fn title(&self) -> String {
        let main_title = match self {
            Self::Authors { query: _ } => "Поиск авторов",
            Self::Posts { query: _ } => "Поиск публикаций",
        };
        if let Some(query) = self.decoded_query() {
            format!("{query} - {main_title}")
        } else {
            format!("{main_title}")
        }
    }
}

#[function_component(SearchField)]
pub fn search_field() -> Html {
    let navigator = use_navigator().unwrap();

    let route = use_route::<Route>().unwrap_or_default();

    let mode = SearchMode::new(route);

    let query = use_state_eq(|| "".to_string());

    {
        let mode = mode.clone();
        let query = query.clone();
        use_effect_with(mode, move |mode| {
            let current_query = mode.decoded_query().unwrap_or("".to_string());
            query.set(current_query);
        })
    }

    let debounce = {
        let mode = mode.clone();
        let query = query.clone();
        use_debounce(
            move || {
                let current_query = mode.decoded_query();
                let query = (*query).clone();
                let is_unique = Some(query.clone()) == current_query;
                if is_unique || query.is_empty() && current_query == None {
                    return;
                }
                navigator.push(&mode.encode_new_query_and_route(query))
            },
            700,
        )
    };

    let oninput = {
        let query = query.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();

            let search_input: HtmlInputElement = e.target_unchecked_into();
            query.set(search_input.value());
            debounce.run();
        })
    };

    html! {
        <input type="search" class="form-control" placeholder={ mode.placeholder() } value={ (*query).clone() } { oninput } />
    }
}
