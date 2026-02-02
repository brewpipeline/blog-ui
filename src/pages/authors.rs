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
        <>
            <Meta title={ lang::AUTHORS_TITLE } />
            <List<API<AuthorsContainer>>
                r#type={ LoadType::Params(()) }
                use_caches=true
                route_to_page={ Route::Authors }
                component={ |author| html! { <AuthorCard { author } link_to=true /> } }
                error_component={ |_| html! { <Warning text={ lang::AUTHORS_ERROR } /> } }
            >
                <Warning text={ lang::AUTHORS_EMPTY } />
            </List<API<AuthorsContainer>>>
        </>
    }
}
