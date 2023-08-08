use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::warning::*;
use crate::content::*;

use crate::Route;

#[function_component(Authors)]
pub fn authors() -> Html {
    html! {
        <>
            <Meta title="Авторы" />
            <List<API<AuthorsContainer>>
                params={ () }
                route_to_page={ Route::Authors }
                component={ |author| html! { <AuthorCard { author } link_to=true /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки авторов!" /> } }
            >
                <Warning text="Нет авторов." />
            </List<API<AuthorsContainer>>>
        </>
    }
}
