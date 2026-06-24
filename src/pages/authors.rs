use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::warning::*;
use crate::content::*;
use crate::lang;
use crate::utils::*;

use crate::Route;

#[function_component(Authors)]
pub fn authors() -> Html {
    html! {
        <Meta title={ lang::AUTHORS_TITLE } />
        <List<API<AuthorsContainer>>
            r#type={ LoadType::Params(()) }
            use_caches=true
            route_to_page={ Route::Authors }
            component={ |(i, author)| html! { <AuthorCard { author } link_to=true priority={ i < 4 } /> } }
            error_component={ |_| html! {
                <Meta title={ lang::AUTHORS_ERROR_TITLE } noindex=true />
                <Warning text={ lang::AUTHORS_ERROR_TEXT } />
            } }
        >
            <Meta title={ lang::AUTHORS_EMPTY_TITLE } noindex=true />
            <Warning text={ lang::AUTHORS_EMPTY_TEXT } />
        </List<API<AuthorsContainer>>>
    }
}
