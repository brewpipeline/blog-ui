use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::auth_user_block::*;
use crate::components::delayed_component::*;
use crate::components::login_modal::*;
use crate::components::logout_modal::*;
use crate::components::search_button::*;
use crate::components::search_field::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <DelayedComponent<()> component={ |_| html! {
                <>
                    <LoginModal id="loginModal" />
                    <LogoutModal id="logoutModal" />
                </>
            } } deps={ () } />
            <header class="header fixed-top bg-primary-subtle d-flex flex-wrap align-items-center">
                <div class="container">
                    <div class="d-flex flex-wrap align-items-center justify-content-center">

                        <div class="col col-lg-2 d-flex justify-content-start justify-content-lg-center align-items-center">
                            <Link<Route> classes="d-flex link-body-emphasis text-decoration-none" to={ Route::Posts }>
                                <h1 class="item mb-0">
                                    { crate::TITLE }
                                </h1>
                            </Link<Route>>
                        </div>

                        <div class="col px-lg-3 d-none d-lg-block">
                            <DelayedComponent<()> component={ |_| html! {
                                <SearchField />
                            } } deps={ () } />
                        </div>

                        <div class="col col-lg-3 gap-2 d-flex justify-content-end align-items-center">
                            <DelayedComponent<()> component={ |_| html! {
                                <>
                                    <SearchButton />
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
