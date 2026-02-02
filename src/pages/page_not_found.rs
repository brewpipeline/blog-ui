use yew::prelude::*;

use crate::components::meta::*;
use crate::components::warning::*;
use crate::lang;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <>
            <Meta title={ lang::COMMON_PAGE_NOT_FOUND_TITLE } noindex=true />
            <Warning text={ lang::COMMON_PAGE_NOT_FOUND_TEXT } />
        </>
    }
}
