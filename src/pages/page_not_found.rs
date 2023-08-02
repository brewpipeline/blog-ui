use yew::prelude::*;

use crate::components::warning::*;
use crate::utils::head;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    head::reset_title_and_meta();
    head::set_prefix_default_title("Cтраница не найдена".to_string());
    html! {
        <Warning text="Cтраница не найдена!" />
    }
}
