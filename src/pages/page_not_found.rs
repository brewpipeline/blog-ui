use yew::prelude::*;

use crate::components::warning::*;
use crate::utils::*;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    let app_meta = use_context::<AppMetaContext>().unwrap();
    app_meta.dispatch([AppMetaAction::Title("Cтраница не найдена".to_string())].into());
    html! {
        <Warning text="Cтраница не найдена!" />
    }
}
