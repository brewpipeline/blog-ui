use gloo::utils::window;
use web_sys::{ScrollToOptions, ScrollBehavior};
use yew::prelude::*;
use yew::html::Scope;
use yew_router::prelude::*;
use yew_router::scope_ext::RouterScopeExt;

use crate::Route;

#[derive(PartialEq, Clone, Copy)]
pub enum EnabledMenu {
    First,
    Second,
    Third,
}

pub struct Body {
    enabled_menu: EnabledMenu,
    _listener: LocationHandle,
}

impl Component for Body {
    type Message = EnabledMenu;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let listener = ctx
            .link()
            .add_location_listener(ctx.link().callback(move |_| EnabledMenu::Second))
            .unwrap();
        Self {
            enabled_menu: EnabledMenu::Second,
            _listener: listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.enabled_menu = msg;
        true
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        window().scroll_to_with_scroll_to_options(
            ScrollToOptions::new()
                .left(0.0)
                .top(0.0)
                .behavior(ScrollBehavior::Instant)
        );
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self { enabled_menu, .. } = *self;

        html! {
            <main class="body position-relative container">

                <div class="menu-nav btn-group d-flex d-lg-none" role="group">
                    <input 
                        type="radio" 
                        class="btn-check" 
                        name="vbtn-radio" 
                        id="vbtn-radio1" 
                        autocomplete="off" 
                        onchange={ ctx.link().callback(|_| EnabledMenu::First) } checked={ enabled_menu == EnabledMenu::First }
                    />
                    <label class="btn btn-light" for="vbtn-radio1"> { "Меню" } </label>
                    <input 
                        aria-label="Контент"
                        type="radio" 
                        class="btn-check" 
                        name="vbtn-radio" 
                        id="vbtn-radio2" 
                        autocomplete="off" 
                        onchange={ ctx.link().callback(|_| EnabledMenu::Second) } checked={ enabled_menu == EnabledMenu::Second } 
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
                        onchange={ ctx.link().callback(|_| EnabledMenu::Third) } checked={ enabled_menu == EnabledMenu::Third } 
                    />
                    <label class="btn btn-light" for="vbtn-radio3"> { "Инфо" } </label>
                </div>

                <div class="d-flex flex-wrap">

                    <div 
                        id="menu1" 
                        class={ classes!("menu", "gap-2", "sticky-lg-top", "col", "col-lg-2", "d-lg-grid", { if enabled_menu == EnabledMenu::First { "d-grid" } else { "d-none" } }) }
                    >
                        { self.navigation_menu(ctx.link()) }
                        <div class="d-flex flex-wrap align-items-end justify-content-center">
                            <a href="http://tikitko.su/" class="text-decoration-none text-center"> { "Сделано с ❤️" } </a>
                        </div>
                    </div>

                    <div 
                        id="menu2" 
                        class={ classes!("menu", "col", "px-0", "px-lg-3", "d-lg-block", { if enabled_menu == EnabledMenu::Second { "d-block" } else { "d-none" } }) }
                    >
                        // TO REMOVE START
                        <div class="card alert alert-info d-flex align-items-center" role="alert">
                            <img style="width: 100%;" src="https://cdn.discordapp.com/attachments/779662621610737706/1111962675832950865/GZGp1AKHU2XmF1AlOQYqFg.png" />
                            <h2>{ "Дорогая Аналстейша, ты лучший лучик бобра в наших сердОчках и сегодня в этой день мы поздравляем тебя с пасхой…. т.е. твоим днём повышения уровня в игре по названием «ворлд оф работай, чтобы потом не работать!»" }</h2>
                        </div>
                        // TO REMOVE END
                        <Switch<Route> render={Route::switch} />
                    </div>

                    <div 
                        id="menu3" 
                        class={ classes!("menu", "gap-2", "sticky-lg-top", "col", "col-lg-3", "d-lg-grid", { if enabled_menu == EnabledMenu::Third { "d-grid" } else { "d-none" } }) }
                    >
                        { self.information_menu(ctx.link()) }
                    </div>

                </div>

            </main>
        }
    }
}
impl Body {
    fn navigation_menu(&self, link: &Scope<Self>) -> Html {
        let route = link.route::<Route>().unwrap_or_default();
        html! {
            <div class="d-grid gap-2">
                <Link<Route> classes={classes!("btn", "btn-light", if route == Route::Home { "active" } else { "" })} to={ Route::Home }>{ "Главная" }</Link<Route>>
                <Link<Route> classes={classes!("btn", "btn-light", if route == Route::Posts { "active" } else { "" })} to={ Route::Posts }>{ "Публикации" }</Link<Route>>
                <Link<Route> classes={classes!("btn", "btn-light", if route == Route::Authors { "active" } else { "" })} to={ Route::Authors }>{ "Авторы" }</Link<Route>>
            </div>
        }
    }
    fn information_menu(&self, _link: &Scope<Self>) -> Html {
        html! {
            <div class="accordion" id="accordionExample">
                <div class="accordion-item">
                    <h2 class="accordion-header">
                        <button class="accordion-button" type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" aria-expanded="true" aria-controls="collapseOne">
                        { "О блоге" } 
                        </button>
                    </h2>
                    <div id="collapseOne" class="accordion-collapse collapse show" data-bs-parent="#accordionExample">
                        <div class="accordion-body">
                            <strong> 
                                { "Ты ошибка эволюции." } 
                            </strong>
                            <br/>
                            { "А блог этот про хороших людей в плохое время." } 
                        </div>
                    </div>
                    </div>
                    <div class="accordion-item">
                    <h2 class="accordion-header">
                        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
                        { "Accordion Item #2" } 
                        </button>
                    </h2>
                    <div id="collapseTwo" class="accordion-collapse collapse" data-bs-parent="#accordionExample">
                        <div class="accordion-body">
                        <strong /> { "This is the second item's accordion body." } <strong/> { " It is hidden by default, until the collapse plugin adds the appropriate classes that we use to style each element. These classes control the overall appearance, as well as the showing and hiding via CSS transitions. You can modify any of this with custom CSS or overriding our default variables. It's also worth noting that just about any HTML can go within the <code>.accordion-body</code>, though the transition does limit overflow." } 
                        </div>
                    </div>
                    </div>
                    <div class="accordion-item">
                    <h2 class="accordion-header">
                        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                        { "Accordion Item #3" } 
                        </button>
                    </h2>
                    <div id="collapseThree" class="accordion-collapse collapse" data-bs-parent="#accordionExample">
                        <div class="accordion-body">
                        <strong /> { "This is the third item's accordion body." } <strong/> { " It is hidden by default, until the collapse plugin adds the appropriate classes that we use to style each element. These classes control the overall appearance, as well as the showing and hiding via CSS transitions. You can modify any of this with custom CSS or overriding our default variables. It's also worth noting that just about any HTML can go within the <code>.accordion-body</code>, though the transition does limit overflow." } 
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}