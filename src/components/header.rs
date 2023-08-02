use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(feature = "client")]
use crate::components::auth_user_block::*;
#[cfg(feature = "client")]
use crate::components::login_modal::*;
#[cfg(feature = "client")]
use crate::components::logout_modal::*;
#[cfg(feature = "client")]
use crate::components::search_button::*;
#[cfg(feature = "client")]
use crate::components::search_field::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    #[cfg(feature = "client")]
    let modals = html! {
        <>
            <LoginModal id="loginModal" />
            <LogoutModal id="logoutModal" />
        </>
    };
    #[cfg(not(feature = "client"))]
    let modals = html! {};

    #[cfg(feature = "client")]
    let search_field = html! {
        <SearchField />
    };
    #[cfg(not(feature = "client"))]
    let search_field = html! {};

    #[cfg(feature = "client")]
    let right_buttons = html! {
        <>
            <SearchButton />
            <AuthUserBlock />
        </>
    };
    #[cfg(not(feature = "client"))]
    let right_buttons = html! {};

    html! {
        <>
            { modals }
            <header class="header fixed-top bg-primary-subtle border-bottom d-flex flex-wrap align-items-center">
                <div class="container">
                    <div class="d-flex flex-wrap align-items-center justify-content-center">

                        <div class="col col-lg-2 d-flex justify-content-start justify-content-lg-center align-items-center">
                            <Link<Route> classes="d-flex link-body-emphasis text-decoration-none" to={ Route::Home }>
                                <img height="38" class="item" src="logo.svg" alt="LOGO" />
                            </Link<Route>>
                        </div>

                        <div class="col px-lg-3 d-none d-lg-block">
                            { search_field }
                        </div>

                        <div class="col col-lg-3 gap-2 d-flex justify-content-end align-items-center">
                            { right_buttons }
                        </div>

                    </div>
                </div>
            </header>
        </>
    }
}
