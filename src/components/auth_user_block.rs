use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::item::*;
use crate::components::svg_image::*;
use crate::content::*;
use crate::utils::logged_user_context::*;

use crate::Route;

#[function_component(AuthUserBlock)]
pub fn auth_user_block() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let Some(token) = logged_user_context.state.token().cloned() else {
       return html! {
            <button
                aria-label="Войти"
                type="button"
                class="item btn btn-light"
                data-bs-toggle="modal"
                data-bs-target="#loginModal"
            >
                <div class="d-block d-lg-none">
                    <PersonAddImg />
                </div>
                <div class="d-none d-lg-block"> { "Войти" } </div>
            </button>
        }
    };

    let params = TokenParam {
        token: token.clone(),
        data: AuthorMeParam,
    };

    let component = {
        let logged_user_context = logged_user_context.clone();
        let token = token.clone();
        move |author: Option<Author>| {
            if let Some(author) = author.clone() {
                logged_user_context.dispatch(LoggedUserState::ActiveAndLoaded {
                    token: token.clone(),
                    author,
                });
            }
            html! {
                <div class="d-flex dropdown dropdown-menu-end">
                    <img
                        height="38"
                        src={
                            author
                                .as_ref()
                                .map(|a| a.image_url())
                                .unwrap_or_default()
                        }
                        type="button"
                        alt={
                            author
                                .as_ref()
                                .map(|u| u.slug.clone())
                        }
                        class="img-block item d-flex rounded"
                        data-bs-toggle="dropdown"
                        aria-expanded="false"
                    />
                    if let Some(author) = author.as_ref() {
                        <ul class="dropdown-menu text-small" >
                            <li>
                                <Link<Route, Author>
                                    classes="dropdown-item"
                                    to={ Route::Author { slug: author.slug.clone() } }
                                    state={ Some(author.clone()) }
                                >
                                    { &author.slug }
                                </Link<Route, Author>>
                            </li>
                            // <li><a class="dropdown-item" href="#"> { "Настройки" } </a></li>
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
                        </ul>
                    }
                </div>
            }
        }
    };

    let error_component = {
        let logged_user_context = logged_user_context.clone();
        move |_| {
            logged_user_context.dispatch(LoggedUserState::None);
            html! {}
        }
    };

    html! {
        <Item<API<AuthorContainer>, TokenParam<AuthorMeParam>>
            { params }
            { component }
            { error_component }
        />
    }
}
