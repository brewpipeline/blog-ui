use yew::prelude::*;

use crate::components::meta::*;
use crate::components::warning::*;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <>
            <Meta title="Cтраница не найдена" noindex=true />
            <Warning text="Cтраница не найдена!" />
        </>
    }
}
