use yew::prelude::*;

use crate::components::warning::*;
use crate::utils::html_document;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Cтраница не найдена".to_string());
    html! {
        <Warning text="Cтраница не найдена!" />
    }
}
