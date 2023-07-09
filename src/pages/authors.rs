use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::list::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::html_document;

use crate::Route;

#[function_component(Authors)]
pub fn authors() -> Html {
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Авторы".to_string());
    html! {
        <List<API<AuthorsContainer>>
            params={ () }
            route_to_page={ Route::Authors }
            component={ |author| html! { <AuthorCard { author } link_to=true /> } }
            error_component={ |_| html! { <Warning text="Ошибка загрузки авторов" /> } }
        >
            <Warning text="Нет авторов" />
        </List<API<AuthorsContainer>>>
    }
}
