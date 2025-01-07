#[cfg(feature = "client")]
use web_sys::HtmlInputElement;
use yew::prelude::*;
#[cfg(feature = "client")]
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::components::information_menu::*;
use crate::components::navigation_menu::*;

use crate::Route;

#[derive(PartialEq, Clone, Copy)]
pub enum EnabledMenu {
    First,
    Second,
    Third,
}

#[function_component(Body)]
pub fn body() -> Html {
    #[cfg(feature = "client")]
    let window_size = use_window_size();
    let route = use_route::<Route>().unwrap_or_default();

    let large_menu_default_ref = use_node_ref();
    let small_menu_default_ref = use_node_ref();

    #[cfg(feature = "client")]
    {
        let window_size = window_size.clone();
        let large_menu_default_ref = large_menu_default_ref.clone();
        let small_menu_default_ref = small_menu_default_ref.clone();
        use_effect_with(window_size, move |_| {
            large_menu_default_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .set_checked(true);
            small_menu_default_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .set_checked(true);
        });
    }

    html! {
        <main class="body position-relative container">

            <div class="menu-nav btn-group d-flex d-lg-none" role="group">
                <input
                    type="radio"
                    class="btn-check"
                    name="vbtn-radio"
                    id="vbtn-radio1"
                    autocomplete="off"
                />
                <label class="btn btn-light d-block d-lg-none" for="vbtn-radio1"> { "Меню" } </label>
                <input
                    aria-label="Лента"
                    type="radio"
                    class="btn-check"
                    name="vbtn-radio"
                    id="vbtn-radio2"
                    autocomplete="off"
                    checked={ true }
                    ref={ large_menu_default_ref }
                />
                <label title="Лента" class="btn btn-light" for="vbtn-radio2">
                    <i class="bi bi-card-heading"></i>
                </label>
                <input
                    type="radio"
                    class="btn-check"
                    name="vbtn-radio"
                    id="vbtn-radio3"
                    autocomplete="off"
                />
                <label class="btn btn-light" for="vbtn-radio3"> { "Инфо" } </label>
            </div>

            <div class="d-flex flex-wrap">

                <div
                    id="menu1"
                    class={ classes!(
                        "menu",
                        "gap-2",
                        "sticky-lg-top",
                        "col-12",
                        "col-lg-3",
                        "col-xl-2",
                        "d-lg-grid"
                    ) }
                >
                    <div class="d-grid gap-2">
                        <div class="btn-group d-none d-flex d-lg-flex d-xl-none flex-wrap" role="group">
                            <input
                                aria-label="Лента"
                                type="radio"
                                class="btn-check"
                                name="vbtn2-radio"
                                id="vbtn2-radio1"
                                autocomplete="off"
                                checked={ true }
                                ref={ small_menu_default_ref }
                            />
                            <label title="Лента" class="btn btn-light" for="vbtn2-radio1">
                                <i class="bi bi-card-heading"></i>
                            </label>
                            <input
                                aria-label="Информация"
                                type="radio"
                                class="btn-check"
                                name="vbtn2-radio"
                                id="vbtn2-radio2"
                                autocomplete="off"
                            />
                            <label title="Информация" class="btn btn-light" for="vbtn2-radio2">
                                <i class="bi bi-info-square"></i>
                            </label>
                        </div>
                        <NavigationMenu />
                    </div>
                    <div class="d-flex flex-wrap align-items-end justify-content-center">
                        <p class="mb-0 text-center">
                            <span title="GIT_SHORT_HASH" aria-label="GIT_SHORT_HASH" style="color:var(--bs-body-bg);">
                                { env!("GIT_SHORT_HASH") }
                            </span>
                            <br/>
                            <a href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" class="text-decoration-none">
                                { "Правила" }
                            </a>
                            <br/>
                            <a href="https://about.tikitko.su/" class="text-decoration-none">
                                { "О Tikitko" }
                            </a>
                        </p>
                    </div>
                </div>

                <div
                    id="menu2"
                    class={ classes!(
                        "menu",
                        "col-12",
                        "col-lg-9",
                        "col-xl-7",
                        "px-0",
                        "ps-lg-3",
                        "px-xl-3",
                        "d-xl-block"
                    ) }
                >
                    <Switch<Route> render={Route::switch} />
                </div>

                <div
                    id="menu3"
                    class={ classes!(
                        "menu",
                        "gap-2",
                        "sticky-lg-top",
                        "col-12",
                        "col-lg-9",
                        "col-xl-3",
                        "ps-lg-3",
                        "ps-xl-0",
                        "d-xl-grid"
                    ) }
                >
                    <InformationMenu />
                </div>

            </div>

        </main>
    }
}
