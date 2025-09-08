use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::auth_user_block::*;
#[cfg(feature = "chatgpt")]
use crate::components::chatgpt_button::*;
use crate::components::delayed_component::*;
use crate::components::search_button::*;
use crate::components::search_field::*;
use crate::components::subscribe_button::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    #[cfg(feature = "chatgpt")]
    let chat_btn_desktop: Html = html! { <ChatGptButton /> };
    #[cfg(not(feature = "chatgpt"))]
    let chat_btn_desktop: Html = html! {};

    #[cfg(feature = "chatgpt")]
    let chat_btn_mobile: Html =
        html! { <ChatGptButton classes={classes!("d-block", "d-lg-none")} /> };
    #[cfg(not(feature = "chatgpt"))]
    let chat_btn_mobile: Html = html! {};

    html! {
        <>
            <header class="header fixed-top bg-primary-subtle d-flex flex-wrap align-items-center">
                <div class="container">
                    <div class="d-flex flex-wrap align-items-center justify-content-center">

                        <div class="col col-lg-3 col-xl-2 d-flex justify-content-start justify-content-lg-center align-items-center">
                            <Link<Route> classes="d-flex link-body-emphasis text-decoration-none" to={ Route::Posts }>
                                <img id="logo-image" height="38" width="149" style="pointer-events:none;" class="item mb-0" alt={ crate::TITLE } src="tikitko-light.svg"/>
                            </Link<Route>>
                        </div>

                        <div class="col ps-lg-3 pe-lg-2 px-xl-3 d-none d-lg-block">
                            <DelayedComponent<()> component={ move |_| html! {
                                <div class="d-flex align-items-center gap-2">
                                    { chat_btn_desktop.clone() }
                                    <SearchField id="headerSearchField" />
                                </div>
                            } } deps={ () } />
                        </div>

                        <div class="col col-lg-auto col-xl-3 gap-1 gap-sm-2 d-flex justify-content-end align-items-center">
                            <DelayedComponent<()> component={ move |_| html! {
                                <>
                                    { chat_btn_mobile.clone() }
                                    <SearchButton />
                                    <SubscribeButton />
                                    <AuthUserBlock />
                                </>
                            } } deps={ () } />
                        </div>

                    </div>
                </div>
            </header>
        </>
    }
}
