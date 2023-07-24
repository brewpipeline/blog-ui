use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::item::*;
use crate::content::*;
use crate::utils::logged_user_context::*;

use crate::Route;

#[function_component(AuthUserBlock)]
pub fn auth_user_block() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    html! {
        if let LoggedUserState::Active { token } = logged_user_context.state.clone() {
            <Item<API<AuthorContainer>, TokenParam<AuthorMeParam>>
                params={ TokenParam { token, data: AuthorMeParam } }
                component={ |author: Option<Author>| {
                    html! {
                        <div class="d-flex dropdown dropdown-menu-end">
                            <img height="38" src={ author.as_ref().map(|a| a.image_url()).unwrap_or_default() } type="button" alt={ author.as_ref().map(|u| u.slug.clone()) } class="img-block item d-flex rounded" data-bs-toggle="dropdown" aria-expanded="false" />
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
                                    <li><button class="dropdown-item" type="button" data-bs-toggle="modal" data-bs-target="#logoutModal"> { "Выход" } </button></li>
                                </ul>
                            }
                        </div>
                    }
                } }
                error_component={ move |_| {
                    logged_user_context.dispatch(LoggedUserState::None);
                    html! {}
                } }
            />
        } else {
            <button aria-label="Войти" type="button" class="item btn btn-light" data-bs-toggle="modal" data-bs-target="#loginModal">
                <div class="d-block d-lg-none">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-add" viewBox="0 0 16 16">
                        <path d="M12.5 16a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7Zm.5-5v1h1a.5.5 0 0 1 0 1h-1v1a.5.5 0 0 1-1 0v-1h-1a.5.5 0 0 1 0-1h1v-1a.5.5 0 0 1 1 0Zm-2-6a3 3 0 1 1-6 0 3 3 0 0 1 6 0ZM8 7a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"/>
                        <path d="M8.256 14a4.474 4.474 0 0 1-.229-1.004H3c.001-.246.154-.986.832-1.664C4.484 10.68 5.711 10 8 10c.26 0 .507.009.74.025.226-.341.496-.65.804-.918C9.077 9.038 8.564 9 8 9c-5 0-6 3-6 4s1 1 1 1h5.256Z"/>
                    </svg>
                </div>
                <div class="d-none d-lg-block"> { "Войти" } </div>
            </button>
        }
    }
}
