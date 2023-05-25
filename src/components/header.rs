use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::login_modal::*;
use crate::logged_user_context::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();
    html! {
        <>
            <LoginModal id="loginModal" />
            <header class="header fixed-top bg-primary-subtle border-bottom d-flex flex-wrap align-items-center">
                <div class="container">
                    <div class="d-flex flex-wrap align-items-center justify-content-center">
            
                        <div class="col col-lg-2 d-flex justify-content-start justify-content-lg-center align-items-center">
                            <Link<Route> classes="d-flex link-body-emphasis text-decoration-none" to={ Route::Home }>
                                <object class="item" style="pointer-events: none;" type="image/svg+xml" data="logo.svg"> { "LOGO" } </object>
                            </Link<Route>>
                        </div>
                
                        <form class="col px-3 d-flex justify-content-center align-items-center d-none d-lg-block" role="search">
                            <input type="search" class="form-control" list="datalistOptions" placeholder="Поиск..." />
                            <datalist id="datalistOptions">
                                <option value="San Francisco" />
                                <option value="New York" />
                                <option value="Seattle" />
                                <option value="Los Angeles" />
                                <option value="Chicago" />
                            </datalist>
                        </form>
                
                        <div class="col col-lg-3 gap-2 d-flex justify-content-end align-items-center">
                        <button type="button" class="btn btn-light d-block d-lg-none">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-search" viewBox="0 0 16 16">
                            <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"></path>
                            </svg>
                        </button>
                        if let LoggedUserState::Active(auth_user) = logged_user_context.state.clone() {
                            <div class="d-flex dropdown dropdown-menu-end">
                                <img src={ auth_user.image_url.clone() } type="button" alt={ auth_user.username.clone() } class="item d-flex rounded" data-bs-toggle="dropdown" aria-expanded="false" />
                                <ul class="dropdown-menu text-small" >
                                    <li><Link<Route> classes="dropdown-item" to={ Route::Author { id: auth_user.id } }> { auth_user.username.clone() } </Link<Route>></li>
                                    // <li><a class="dropdown-item" href="#"> { "Настройки" } </a></li>
                                    <li><hr class="dropdown-divider" /></li>
                                    <li><button class="dropdown-item" onclick={ move |_| logged_user_context.dispatch(LoggedUserState::None) }> { "Выход" } </button></li>
                                </ul>
                            </div>
                        } else {
                            <button type="button" class="item btn btn-light" data-bs-toggle="modal" data-bs-target="#loginModal"> { "Войти" } </button>
                        }
                        </div>
            
                    </div>
                </div>
            </header>
        </>
    }
}