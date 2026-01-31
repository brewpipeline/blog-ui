use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::author_image::*;
use crate::content::*;
use crate::lang;
use crate::utils::*;

use crate::Route;

#[function_component(AuthUserBlock)]
pub fn auth_user_block() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let social_tries = use_state_eq(|| 0);

    if logged_user_context.is_not_inited() {
        return html! {};
    }

    let social_tries_out = *social_tries >= 2;

    let Some(_) = logged_user_context.token().cloned() else {
        #[cfg(feature = "telegram")]
        let tg_button: Option<Html> = /* Some(html! {
            <button
                title={ lang::AUTH_LOGIN_VIA_TELEGRAM_TITLE }
                aria-label={ lang::AUTH_LOGIN_VIA_TELEGRAM_ARIA }
                type="button"
                class={ classes!(
                    "item",
                    "btn",
                    "bg-color-tg",
                    { if social_tries_out { "d-none" } else { "d-block" } }
                ) }
                ONCLICK="return TWidgetLogin.auth();"
                onclick={
                    let social_tries = social_tries.clone();
                    move |_e: MouseEvent| social_tries.set(*social_tries + 1)
                }
            >
                <i class="bi bi-tg"></i>
            </button>
        }); */ None;
        #[cfg(not(feature = "telegram"))]
        let tg_button: Option<Html> = None;

        return html! {
            <>
                { tg_button.clone() }
                <button
                    title={ lang::AUTH_LOGIN_ARIA }
                    aria-label={ lang::AUTH_LOGIN_ARIA }
                    type="button"
                    class={ classes!(
                        "item",
                        "btn",
                        "btn-light",
                        { if tg_button == None || social_tries_out { "d-block" } else { "d-none" } }
                    ) }
                    data-bs-toggle="modal"
                    data-bs-target="#loginModal"
                >
                    <div class="d-block d-lg-none">
                        <i class="bi bi-person-add"></i>
                    </div>
                    <div class="d-none d-lg-block">
                        { lang::AUTH_LOGIN }
                    </div>
                </button>
            </>
        };
    };

    let author = logged_user_context.author().cloned();

    html! {
        <div class="d-flex dropdown dropdown-menu-end">
            <div
                title={ lang::AUTH_PROFILE }
                class="img-block item d-flex rounded"
                style="overflow:hidden;width:38px;"
                data-bs-toggle="dropdown"
                aria-expanded="false"
                type="button"
            >
                <AuthorImage author={ author.clone() } />
            </div>
            <ul class="dropdown-menu text-small" >
                if let Some(author) = author.as_ref() {
                    <li>
                        <Link<Route, (), Author>
                            classes="dropdown-item"
                            to={ Route::Author { slug: author.slug.clone() } }
                            state={ Some(author.clone()) }
                        >
                            { author_slug_formatter(&author) }
                        </Link<Route, (), Author>>
                    </li>
                    <li>
                        <Link<Route, ()>
                            classes="dropdown-item"
                            to={ Route::MyUnpublishedPosts }
                        >
                            { lang::AUTH_MY_UNPUBLISHED }
                        </Link<Route, ()>>
                    </li>
                    <li>
                        <Link<Route, ()>
                            classes="dropdown-item"
                            to={ Route::Settings }
                        >
                            { lang::AUTH_SETTINGS }
                        </Link<Route, ()>>
                    </li>
                    <li><hr class="dropdown-divider" /></li>
                    <li>
                        <button
                            class="dropdown-item"
                            type="button"
                            data-bs-toggle="modal"
                            data-bs-target="#logoutModal"
                        >
                            { lang::AUTH_LOGOUT_MENU }
                        </button>
                    </li>
                }
            </ul>
        </div>
    }
}
