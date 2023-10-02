use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::auth_user_block::*;
use crate::components::delayed_component::*;
use crate::components::search_button::*;
use crate::components::search_field::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <>
            <header class="header fixed-top bg-primary-subtle d-flex flex-wrap align-items-center">
                <div class="container">
                    <div class="d-flex flex-wrap align-items-center justify-content-center">

                        <div class="col col-lg-2 d-flex justify-content-start justify-content-lg-center align-items-center">
                            <Link<Route> classes="d-flex link-body-emphasis text-decoration-none" to={ Route::Posts }>
                                <img id="logo-image" style="pointer-events:none;" class="item mb-0" alt={ crate::TITLE } src="tikitko-light.svg"/>
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
