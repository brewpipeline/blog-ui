use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::body::*;
use crate::components::delayed_component::*;
use crate::components::header::*;
use crate::components::login_modal::*;
use crate::components::logout_modal::*;
use crate::components::svg_image::*;
use crate::components::yandex_token::*;

use crate::Route;

#[function_component(Content)]
pub fn content() -> Html {
    let route = use_route::<Route>().unwrap_or_default();
    html! {
        if route == Route::YandexToken {
            <YandexToken />
            <YandexImg />
        } else {
            <>
            <Header />
            <Body />
            <DelayedComponent<()> component={ |_| html! {
                <>
                    <LoginModal id="loginModal" />
                    <LogoutModal id="logoutModal" />
                </>
            } } deps={ () } />
        </>
        }
    }
}
