use gloo::events::EventListener;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::content::*;
use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct LoginModalProps {
    pub id: &'static str,
}

#[function_component(LoginModal)]
pub fn login_modal(props: &LoginModalProps) -> Html {
    let LoginModalProps { id } = props.clone();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let close_node_ref = use_node_ref();

    {
        let logged_user_context = logged_user_context.clone();
        let close_node_ref = close_node_ref.clone();
        use_effect_with(logged_user_context, move |logged_user_context| {
            let LoggedUserState::InProgress(auth_params) = (**logged_user_context).state.clone() else {
                return
            };
            let logged_user_context = logged_user_context.clone();
            let close_node_ref = close_node_ref.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match API::<TokenContainer>::get(auth_params).await {
                    Ok(auth_result) => match auth_result {
                        API::Success {
                            identifier: _,
                            description: _,
                            data: token_container,
                        } => {
                            close_node_ref.cast::<HtmlInputElement>().unwrap().click();
                            logged_user_context.dispatch(LoggedUserState::Active {
                                token: token_container.token,
                            });
                        }
                        API::Failure { identifier, reason } => {
                            logged_user_context
                                .dispatch(LoggedUserState::Error(reason.unwrap_or(identifier)));
                        }
                    },
                    Err(err) => {
                        logged_user_context.dispatch(LoggedUserState::Error(err.to_string()));
                    }
                }
            });
        });
    }

    let username_node_ref = use_node_ref();
    let password_node_ref = use_node_ref();

    let onclick = {
        let logged_user_context = logged_user_context.clone();
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();
        Callback::from(move |_event| {
            let username = username_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            let password = password_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value();
            logged_user_context.dispatch(LoggedUserState::InProgress(AuthParams {
                slug: username,
                password,
            }));
        })
    };

    let modal_node_ref = use_node_ref();

    {
        let logged_user_context = logged_user_context.clone();
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();
        let modal_node_ref = modal_node_ref.clone();
        use_effect_with(logged_user_context, move |logged_user_context| {
            let logged_user_context = logged_user_context.clone();
            let modal_element = modal_node_ref.cast::<HtmlElement>().unwrap();
            let listener = EventListener::new(&modal_element, "hidden.bs.modal", move |e| {
                e.prevent_default();
                username_node_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                password_node_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                if logged_user_context.state.action_available() {
                    logged_user_context.dispatch(LoggedUserState::None);
                };
            });
            move || drop(listener)
        });
    }

    html! {
        <div
            class="modal fade"
            { id }
            tabindex="-1"
            aria-labelledby="loginModalLabel"
            aria-hidden="true"
            ref={ modal_node_ref }
        >
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="modal-title fs-5" id="loginModalLabel"> { "Войти" } </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        if let LoggedUserState::Error(message) = logged_user_context.state.clone() {
                            <div class="alert alert-danger d-flex align-items-center" role="alert">
                                { "Ошибка авторизации: " }
                                { message }
                            </div>
                        }
                        <div class="form-floating mb-3">
                            <input
                                type="email"
                                class="form-control"
                                id="floatingInput"
                                placeholder="Имя пользователя"
                                ref={ username_node_ref }
                                disabled={ !logged_user_context.state.action_available() }
                            />
                            <label for="floatingInput"> { "Имя пользователя" } </label>
                        </div>
                        <div class="form-floating">
                            <input
                                type="password"
                                class="form-control"
                                id="floatingPassword"
                                placeholder="Password"
                                ref={ password_node_ref }
                                disabled={ !logged_user_context.state.action_available() }
                            />
                            <label for="floatingPassword"> { "Пароль" } </label>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button
                            type="button"
                            class="btn btn-secondary"
                            data-bs-dismiss="modal"
                            ref={ close_node_ref }
                        >
                            { "Закрыть" }
                        </button>
                        <button
                            type="button"
                            class="btn btn-primary"
                            { onclick }
                            disabled={ !logged_user_context.state.action_available() }
                        >
                            if let LoggedUserState::InProgress(_) = (*logged_user_context).state.clone() {
                                <div class="spinner-border spinner-border-sm" role="status">
                                    <span class="visually-hidden"> { "Загрузка..." } </span>
                                </div>
                                { " " }
                            }
                            { "Войти" }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
