#[cfg(feature = "client")]
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[cfg(feature = "client")]
use crate::utils::*;
use crate::Route;

#[function_component(Footer)]
pub fn footer() -> Html {
    #[cfg(feature = "client")]
    let window_width = use_window_width();
    let _ = use_route::<Route>().unwrap_or_default();

    let large_menu_default_ref = use_node_ref();

    #[cfg(feature = "client")]
    {
        let window_width = window_width.clone();
        let large_menu_default_ref = large_menu_default_ref.clone();
        use_effect_with(window_width, move |_| {
            large_menu_default_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .set_checked(true);
        });
    }

    html! {
        <>
            <footer class="footer fixed-bottom bg-primary-subtle d-flex d-lg-none">
                <div class="container">
                        <div class="d-flex justify-content-center gap-2">
                            <input
                                type="radio"
                                class="btn-check"
                                name="vbtn-radio"
                                id="vbtn-radio1"
                                autocomplete="off"
                            />
                            <label class="btn btn-light tab-label" for="vbtn-radio1">
                                <i class="bi bi-list"></i>
                                { "Меню" }
                            </label>
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
                            <label class="btn btn-light tab-label" for="vbtn-radio2">
                                <i class="bi bi-card-heading"></i>
                                { "Лента" }
                            </label>
                            <input
                                type="radio"
                                class="btn-check"
                                name="vbtn-radio"
                                id="vbtn-radio3"
                                autocomplete="off"
                            />
                            <label class="btn btn-light tab-label" for="vbtn-radio3">
                                <i class="bi bi-info-square"></i>
                                { "Инфо" }
                            </label>
                        </div>
                </div>
            </footer>
        </>
    }
}
