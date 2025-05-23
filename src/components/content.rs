#[cfg(feature = "client")]
use gloo::utils::window;
#[cfg(feature = "client")]
use web_sys::{ScrollBehavior, ScrollToOptions};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::body::*;
use crate::components::delayed_component::*;
use crate::components::footer::*;
use crate::components::header::*;
use crate::components::login_modal::*;
use crate::components::logout_modal::*;
#[cfg(feature = "yandex")]
use crate::components::meta::*;
#[cfg(feature = "yandex")]
use crate::components::yandex_token::*;

use crate::Route;

#[function_component(Content)]
pub fn content() -> Html {
    let route = use_route::<Route>().unwrap_or_default();
    let location = use_location();

    #[cfg(feature = "client")]
    {
        let location = location.clone();
        use_effect_with(location, move |_| {
            let scroll_to_options = ScrollToOptions::new();
            scroll_to_options.set_left(0.0);
            scroll_to_options.set_top(0.0);
            scroll_to_options.set_behavior(ScrollBehavior::Instant);
            window().scroll_to_with_scroll_to_options(&scroll_to_options);
        });
    }

    let main_content = html! {
        <>
            <Header />
            <Body />
            <Footer />

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
            <svg viewBox="0 0 377 90.9" xmlns="http://www.w3.org/2000/svg" width="2500" height="603" style="width: 100%; height: 100%; padding: 50px;">
                <path d="M218.2 90.9c7.2 0 13.1-3.4 17.3-9.7l.8 8.4h14.2V0h-15.3v32.4c-3.8-5.8-9.6-8.9-16.3-8.9-14.8 0-25.2 12.5-25.2 34.2 0 21.3 10.1 33.2 24.5 33.2zm94.4-5.6V72.8c-4.8 3.2-12.8 6.1-20.3 6.1-11.2 0-15.5-5.3-16.1-16.1h37v-8.2c0-22.6-9.9-31.1-25.3-31.1-18.7 0-27.6 14.3-27.6 33.9 0 22.6 11.1 33.5 30.7 33.5 9.8 0 17-2.5 21.6-5.6zM147.3 43.1c2.9-3.5 7.3-6.4 12.9-6.4 5.4 0 7.8 2.3 7.8 7.2v45.8h15.3V42.3c0-12.9-5.1-18.6-17.7-18.6-9.1 0-14.6 3.4-17.7 6.4h-.8l-.4-5.4h-15v64.9H147zm-27.9 2.3c0-15.7-8-21.7-24.3-21.7-10.1 0-18.2 4.3-22.8 7.6v13.3c4.9-4 12.4-8.6 21-8.6 7.3 0 10.7 2.6 10.7 9.6v4.1h-2.4c-23.5 0-33.9 7.6-33.9 21.2 0 12.5 8 19.7 19.9 19.7 9 0 12.9-3 15.9-6.1h.6c.1 1.7.6 3.8 1.1 5.1h15c-.5-5.3-.8-10.6-.8-15.9zm240.3 44.3H377l-21.2-33.4 18.3-31.4h-15.3l-11.1 19.6-12.4-19.6h-17.4L337.5 56l-20.4 33.8h15.6l13-21.9zM222.8 35.6c8.3 0 12.4 6.6 12.4 21.5 0 15.1-4.4 21.7-13 21.7-8.4 0-12.5-6.4-12.5-21.2-.1-15.3 4.3-22 13.1-22zm64.9 0c7.6 0 9.9 6.3 9.9 14.4v1.3h-21.4c.3-10.3 4.1-15.7 11.5-15.7zM104.1 74.2c-1.9 2.9-5.6 5.1-11 5.1-6.4 0-9.7-3.7-9.7-9.4 0-7.5 5.3-10.1 18.4-10.1h2.2v14.4z"/><path d="M45 74.4v15.2H29.4V64L0 0h16.3l22.9 50c4.4 9.6 5.8 12.9 5.8 24.4zM74.3 0L55.2 43.3H39.3L58.5 0z" fill="#fc3f1d"/>
            </svg>
        } else {
            { main_content }
        }
    }
    #[cfg(not(feature = "yandex"))]
    main_content
}
