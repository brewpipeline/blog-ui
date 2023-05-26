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
            Route::Home | 
            Route::NotFound |
            Route::PostsSearchRoot | 
            Route::Post { id: _ } | 
            Route::Posts  => Self::Posts { query: None },
            Route::AuthorsSearchRoot | 
            Route::Author { id: _ } | 
            Route::Authors => Self::Authors { query: None },
        }
    }
    pub fn decoded_query(&self) -> Option<String> {
        let query = match self {
            Self::Authors { query } | Self::Posts { query } => query,
        };
        query                
            .clone()
            .map(|q| 
                urlencoding::decode(q.as_str())
                    .map(|c| 
                        c.into_owned()
                    )
                    .ok()
            )
            .flatten()
    }
    pub fn new_query_route(&self, query: String) -> Route {
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
        use_effect_with_deps(move |mode| {
            let current_query = mode.decoded_query();
            query.set(current_query.unwrap_or("".to_string()));
        }, mode)
    }
    
    let debounce = {
        let mode = mode.clone();
        let query = query.clone();
        use_debounce(move || {
            let current_query = mode.decoded_query();
            let query = (*query).clone();
            if query.is_empty() && current_query == None || Some(query.clone()) == current_query {
                return
            }
            navigator.push(&mode.new_query_route(query))
        }, 700)
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
        <form role="search">
            <input type="search" class="form-control" placeholder={ mode.placeholder() } value={ (*query).clone() } { oninput } />
        </form>
    }
}
