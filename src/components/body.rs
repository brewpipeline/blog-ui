#[cfg(feature = "client")]
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::ai_chat::*;
use crate::components::information_menu::*;
use crate::components::navigation_menu::*;
use crate::components::recommended_post::*;

#[cfg(feature = "client")]
use crate::utils::*;
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
    let window_width = use_window_width();
    let route = use_route::<Route>().unwrap_or_default();

    let small_menu_default_ref = use_node_ref();

    #[cfg(feature = "client")]
    {
        let window_width = window_width.clone();
        let small_menu_default_ref = small_menu_default_ref.clone();
        use_effect_with(window_width, move |_| {
            small_menu_default_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .set_checked(true);
        });
    }

    html! {
        <main class="body position-relative container">

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
                    <div class="d-flex flex-column gap-2 align-items-center justify-content-end">
                        <AiChat />
                        <p class="mb-0 text-center">
                            <a href="https://github.com/tikitko/blog-ui/blob/main/MADEWITHLOVE.md" class="text-decoration-none">
                                { "Сделано с ❤️" }
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
                        "d-xl-block"
                    ) }
                >
                    <InformationMenu />
                    <RecommendedPost />
                </div>

            </div>

        </main>
    }
}
