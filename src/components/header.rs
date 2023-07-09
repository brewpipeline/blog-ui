use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::auth_user_block::*;
use crate::components::login_modal::*;
use crate::components::logout_modal::*;
use crate::components::search_button::*;
use crate::components::search_field::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <LoginModal id="loginModal" />
            <LogoutModal id="logoutModal" />
            <header class="header fixed-top bg-primary-subtle border-bottom d-flex flex-wrap align-items-center">
                <div class="container">
                    <div class="d-flex flex-wrap align-items-center justify-content-center">

                        <div class="col col-lg-2 d-flex justify-content-start justify-content-lg-center align-items-center">
                            <Link<Route> classes="d-flex link-body-emphasis text-decoration-none" to={ Route::Home }>
                                <img height="38" class="item" src="logo.svg" alt="LOGO" />
                            </Link<Route>>
                        </div>

                        <div class="col px-lg-3 d-none d-lg-block">
                            <SearchField />
                        </div>

                        <div class="col col-lg-3 gap-2 d-flex justify-content-end align-items-center">
                            <SearchButton />
                            <AuthUserBlock />
                        </div>

                    </div>
                </div>
            </header>
        </>
    }
}
