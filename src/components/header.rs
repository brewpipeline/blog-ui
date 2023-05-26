use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::login_modal::*;
use crate::components::logout_modal::*;
use crate::components::search_button::*;
use crate::components::search_field::*;
use crate::logged_user_context::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();
    html! {
        <>
            <LoginModal id="loginModal" />
            <LogoutModal id="logoutModal" />
            <header class="header fixed-top bg-primary-subtle border-bottom d-flex flex-wrap align-items-center">
                <div class="container">
                    <div class="d-flex flex-wrap align-items-center justify-content-center">
            
                        <div class="col col-lg-2 d-flex justify-content-start justify-content-lg-center align-items-center">
                            <Link<Route> classes="d-flex link-body-emphasis text-decoration-none" to={ Route::Home }>
                                <object class="item" style="pointer-events: none;" type="image/svg+xml" data="logo.svg"> { "LOGO" } </object>
                            </Link<Route>>
                        </div>
                
                        <div class="col px-lg-3 d-none d-lg-block">
                            <SearchField />
                        </div>
                
                        <div class="col col-lg-3 gap-2 d-flex justify-content-end align-items-center">
                            <SearchButton />
                            if let LoggedUserState::Active(auth_user) = logged_user_context.state.clone() {
                                <div class="d-flex dropdown dropdown-menu-end">
                                    <img src={ auth_user.image_url.clone() } type="button" alt={ auth_user.username.clone() } class="item d-flex rounded" data-bs-toggle="dropdown" aria-expanded="false" />
                                    <ul class="dropdown-menu text-small" >
                                        <li><Link<Route> classes="dropdown-item" to={ Route::Author { id: auth_user.id } }> { auth_user.username.clone() } </Link<Route>></li>
                                        // <li><a class="dropdown-item" href="#"> { "Настройки" } </a></li>
                                        <li><hr class="dropdown-divider" /></li>
                                        <li><button class="dropdown-item" type="button" data-bs-toggle="modal" data-bs-target="#logoutModal"> { "Выход" } </button></li>
                                    </ul>
                                </div>
                            } else {
                                <button type="button" class="item btn btn-light" data-bs-toggle="modal" data-bs-target="#loginModal">
                                    <div class="d-block d-lg-none">
                                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-add" viewBox="0 0 16 16">
                                            <path d="M12.5 16a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7Zm.5-5v1h1a.5.5 0 0 1 0 1h-1v1a.5.5 0 0 1-1 0v-1h-1a.5.5 0 0 1 0-1h1v-1a.5.5 0 0 1 1 0Zm-2-6a3 3 0 1 1-6 0 3 3 0 0 1 6 0ZM8 7a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"/>
                                            <path d="M8.256 14a4.474 4.474 0 0 1-.229-1.004H3c.001-.246.154-.986.832-1.664C4.484 10.68 5.711 10 8 10c.26 0 .507.009.74.025.226-.341.496-.65.804-.918C9.077 9.038 8.564 9 8 9c-5 0-6 3-6 4s1 1 1 1h5.256Z"/>
                                        </svg>
                                    </div>
                                    <div class="d-none d-lg-block"> { "Войти" } </div>
                                </button>
                            }
                        </div>
            
                    </div>
                </div>
            </header>
        </>
    }
}