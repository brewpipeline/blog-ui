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
        use_effect_with_deps(
            move |logged_user_context| {
                let LoggedUserState::InProgress(login_params) = (**logged_user_context).state.clone() else {
                    return
                };
                let logged_user_context = logged_user_context.clone();
                let close_node_ref = close_node_ref.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match AuthResult::get(login_params).await {
                        Ok(auth_result) => match auth_result {
                            AuthResult::Success(auth_user) => {
                                close_node_ref.cast::<HtmlInputElement>().unwrap().click();
                                logged_user_context.dispatch(LoggedUserState::Active(auth_user));
                            }
                            AuthResult::Error {
                                message: auth_error_message,
                            } => {
                                logged_user_context
                                    .dispatch(LoggedUserState::Error(auth_error_message));
                            }
                        },
                        Err(err) => {
                            logged_user_context.dispatch(LoggedUserState::Error(err.to_string()));
                        }
                    }
                });
            },
            logged_user_context,
        );
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
            logged_user_context.dispatch(LoggedUserState::InProgress(LoginParams {
                username,
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
        use_effect_with_deps(
            move |logged_user_context| {
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
                    match logged_user_context.state {
                        LoggedUserState::None | LoggedUserState::Active(_) => {}
                        LoggedUserState::Error(_) | LoggedUserState::InProgress(_) => {
                            logged_user_context.dispatch(LoggedUserState::None);
                        }
                    };
                });
                move || drop(listener)
            },
            logged_user_context,
        );
    }

    html! {
        <div class="modal fade" { id } tabindex="-1" aria-labelledby="loginModalLabel" aria-hidden="true" ref={ modal_node_ref }>
            <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header">
                        <h1 class="modal-title fs-5" id="loginModalLabel"> { "Войти" } </h1>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </div>
                    <div class="modal-body">
                        <div class="alert alert-info d-flex align-items-center" role="alert">
                            { "Login: atuny0" }
                            <br/>
                            { "Password: 9uQFF1Lh" }
                        </div>
                        if let LoggedUserState::Error(message) = logged_user_context.state.clone() {
                            <div class="alert alert-danger d-flex align-items-center" role="alert">
                                { message }
                            </div>
                        }
                        <div class="form-floating mb-3">
                            <input type="email" class="form-control" id="floatingInput" placeholder="Имя пользователя" ref={ username_node_ref } disabled={ !logged_user_context.state.action_available() } />
                            <label for="floatingInput"> { "Имя пользователя" } </label>
                        </div>
                        <div class="form-floating">
                            <input type="password" class="form-control" id="floatingPassword" placeholder="Password" ref={ password_node_ref } disabled={ !logged_user_context.state.action_available() } />
                            <label for="floatingPassword"> { "Пароль" } </label>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" ref={ close_node_ref }> { "Закрыть" } </button>
                        <button type="button" class="btn btn-primary" { onclick } disabled={ !logged_user_context.state.action_available() }>
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
