use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::body::*;
use crate::components::delayed_component::*;
use crate::components::header::*;
use crate::components::login_modal::*;
use crate::components::logout_modal::*;
#[cfg(feature = "yandex")]
use crate::components::meta::*;
#[cfg(feature = "yandex")]
use crate::components::svg_image::*;
#[cfg(feature = "yandex")]
use crate::components::yandex_token::*;

use crate::Route;

#[function_component(Content)]
pub fn content() -> Html {
    let route = use_route::<Route>().unwrap_or_default();

    let main_content = html! {
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
    };

    #[cfg(feature = "yandex")]
    html! {
        if route == Route::YandexToken {
            <Meta noindex=true />
            <YandexToken />
            <YandexImg />
        } else {
            { main_content }
        }
    }
    #[cfg(not(feature = "yandex"))]
    main_content
}
