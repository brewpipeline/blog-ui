use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::author_image::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[function_component(AuthUserBlock)]
pub fn auth_user_block() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    if logged_user_context.is_not_inited() {
        return html! {};
    }

    let Some(_) = logged_user_context.token().cloned() else {
        #[cfg(feature = "telegram")]
        let tg_button = Some(html! {
            <button
                title="Войти через Telegram"
                aria-label="Войти через Telegram"
                type="button"
                class="item btn bg-color-tg"
                ONCLICK="return TWidgetLogin.auth();"
            >
                <i class="bi bi-tg"></i>
            </button>
        });
        #[cfg(not(feature = "telegram"))]
        let tg_button = None;

        return html! {
            <>
                { tg_button.clone() }
                <button
                    title="Войти"
                    aria-label="Войти"
                    type="button"
                    class={ classes!(
                        "item",
                        "btn",
                        "btn-light",
                        { if tg_button == None { "d-block" } else { "d-none" } }
                    ) }
                    data-bs-toggle="modal"
                    data-bs-target="#loginModal"
                >
                    <div class="d-block d-lg-none">
                        <i class="bi bi-person-add"></i>
                    </div>
                    <div class="d-none d-lg-block">
                        { "Войти" }
                    </div>
                </button>
            </>
        };
    };

    let author = logged_user_context.author().cloned();

    html! {
        <div class="d-flex dropdown dropdown-menu-end">
            <div
                title="Профиль"
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
                            { "Неопубликованное" }
                        </Link<Route, ()>>
                    </li>
                    <li>
                        <Link<Route, ()>
                            classes="dropdown-item"
                            to={ Route::Settings }
                        >
                            { "Настройки" }
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
                            { "Выход" }
                        </button>
                    </li>
                }
            </ul>
        </div>
    }
}
