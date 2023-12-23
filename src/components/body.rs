use yew::prelude::*;
#[cfg(feature = "client")]
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::components::delayed_component::*;
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

    let enabled_menu = use_state_eq(|| EnabledMenu::Second);

    {
        let route = route.clone();
        let enabled_menu = enabled_menu.clone();
        use_effect_with(route, move |_| {
            enabled_menu.set(EnabledMenu::Second);
        });
    }

    #[cfg(feature = "client")]
    {
        let window_size = window_size.clone();
        let enabled_menu = enabled_menu.clone();
        use_effect_with(window_size, move |_| {
            enabled_menu.set(EnabledMenu::Second);
        });
    }

    html! {
        <main class="body position-relative container">

            <div class="menu-nav btn-group d-flex d-lg-none sticky-top" role="group">
                <input
                    type="radio"
                    class="btn-check"
                    name="vbtn-radio"
                    id="vbtn-radio1"
                    autocomplete="off"
                    onchange={
                        let enabled_menu = enabled_menu.clone();
                        Callback::from(move |_| enabled_menu.set(EnabledMenu::First))
                    }
                    checked={ *enabled_menu == EnabledMenu::First }
                />
                <label class="btn btn-light d-block d-lg-none" for="vbtn-radio1"> { "Меню" } </label>
                <input
                    aria-label="Лента"
                    type="radio"
                    class="btn-check"
                    name="vbtn-radio"
                    id="vbtn-radio2"
                    autocomplete="off"
                    onchange={
                        let enabled_menu = enabled_menu.clone();
                        Callback::from(move |_| enabled_menu.set(EnabledMenu::Second))
                    }
                    checked={ *enabled_menu == EnabledMenu::Second }
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
                    onchange={
                        let enabled_menu = enabled_menu.clone();
                        Callback::from(move |_| enabled_menu.set(EnabledMenu::Third))
                    }
                    checked={ *enabled_menu == EnabledMenu::Third }
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
                        "d-lg-grid",
                        { if *enabled_menu == EnabledMenu::First { "d-grid" } else { "d-none" } }
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
                                onchange={
                                    let enabled_menu = enabled_menu.clone();
                                    Callback::from(move |_| enabled_menu.set(EnabledMenu::Second))
                                }
                                checked={ *enabled_menu == EnabledMenu::Second }
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
                                onchange={
                                    let enabled_menu = enabled_menu.clone();
                                    Callback::from(move |_| enabled_menu.set(EnabledMenu::Third))
                                }
                                checked={ *enabled_menu == EnabledMenu::Third }
                            />
                            <label title="Информация" class="btn btn-light" for="vbtn2-radio2">
                                <i class="bi bi-info-square"></i>
                            </label>
                        </div>
                        <DelayedComponent<()> component={ |_| html! {
                            <NavigationMenu />
                        } } deps={ () } />
                    </div>
                    <DelayedComponent<()> component={ |_| html! {
                        <div class="d-flex flex-wrap align-items-end justify-content-center">
                            <a href="https://github.com/tikitko/blog-ui/blob/main/MADEWITHLOVE.md" class="text-decoration-none text-center">
                                { "Сделано с ❤️" }
                            </a>
                        </div>
                    } } deps={ () } />
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
                        "d-xl-block",
                        { if *enabled_menu == EnabledMenu::Second { "d-block" } else { "d-none" } }
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
                        "d-xl-grid",
                        { if *enabled_menu == EnabledMenu::Third { "d-grid" } else { "d-none" } }
                    ) }
                >
                    <DelayedComponent<()> component={ |_| html! {
                        <InformationMenu />
                    } } deps={ () } />
                </div>

            </div>

        </main>
    }
}
