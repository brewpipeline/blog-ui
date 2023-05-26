use yew::prelude::*;

use crate::components::warning::*;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <Warning text="Cтраница не найдена!" />
    }
}