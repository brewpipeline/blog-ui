use gloo::utils::window;
use web_sys::{ScrollBehavior, ScrollToOptions};
use yew::prelude::*;
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
    window().scroll_to_with_scroll_to_options(
        ScrollToOptions::new()
            .left(0.0)
            .top(0.0)
            .behavior(ScrollBehavior::Instant),
    );

    let enabled_menu = use_state_eq(|| EnabledMenu::Second);

    let route = use_route::<Route>().unwrap_or_default();

    {
        let enabled_menu = enabled_menu.clone();
        let route = route.clone();
        use_effect_with(
            route,
            move |_| {
                enabled_menu.set(EnabledMenu::Second);
            },
        );
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
                    onchange={
                        let enabled_menu = enabled_menu.clone();
                        Callback::from(move |_| enabled_menu.set(EnabledMenu::First))
                    } checked={ *enabled_menu == EnabledMenu::First }
                />
                <label class="btn btn-light" for="vbtn-radio1"> { "Меню" } </label>
                <input
                    aria-label="Контент"
                    type="radio"
                    class="btn-check"
                    name="vbtn-radio"
                    id="vbtn-radio2"
                    autocomplete="off"
                    onchange={
                        let enabled_menu = enabled_menu.clone();
                        Callback::from(move |_| enabled_menu.set(EnabledMenu::Second))
                    } checked={ *enabled_menu == EnabledMenu::Second }
                />
                <label class="btn btn-light" for="vbtn-radio2">
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-card-heading" viewBox="0 0 16 16">
                        <path d="M14.5 3a.5.5 0 0 1 .5.5v9a.5.5 0 0 1-.5.5h-13a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5h13zm-13-1A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h13a1.5 1.5 0 0 0 1.5-1.5v-9A1.5 1.5 0 0 0 14.5 2h-13z"/>
                        <path d="M3 8.5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h6a.5.5 0 0 1 0 1h-6a.5.5 0 0 1-.5-.5zm0-5a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5v-1z"/>
                    </svg>
                </label>
                <input
                    type="radio"
                    class="btn-check"
                    name="vbtn-radio"
                    id="vbtn-radio3"
                    autocomplete="off"
                    onchange={
                        let enabled_menu = enabled_menu.clone();
                        Callback::from(move |_| enabled_menu.set(EnabledMenu::Third))
                    } checked={ *enabled_menu == EnabledMenu::Third }
                />
                <label class="btn btn-light" for="vbtn-radio3"> { "Инфо" } </label>
            </div>

            <div class="d-flex flex-wrap">

                <div
                    id="menu1"
                    class={ classes!("menu", "gap-2", "sticky-lg-top", "col", "col-lg-2", "d-lg-grid", { if *enabled_menu == EnabledMenu::First { "d-grid" } else { "d-none" } }) }
                >
                    <NavigationMenu />
                    <div class="d-flex flex-wrap align-items-end justify-content-center">
                        <a href="http://tikitko.su/" class="text-decoration-none text-center"> { "Сделано с ❤️" } </a>
                    </div>
                </div>

                <div
                    id="menu2"
                    class={ classes!("menu", "col", "px-0", "px-lg-3", "d-lg-block", { if *enabled_menu == EnabledMenu::Second { "d-block" } else { "d-none" } }) }
                >
                    <Switch<Route> render={Route::switch} />
                </div>

                <div
                    id="menu3"
                    class={ classes!("menu", "gap-2", "sticky-lg-top", "col", "col-lg-3", "d-lg-grid", { if *enabled_menu == EnabledMenu::Third { "d-grid" } else { "d-none" } }) }
                >
                    <InformationMenu />
                </div>

            </div>

        </main>
    }
}
