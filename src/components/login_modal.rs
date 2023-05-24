use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo_net::http::Request;

use crate::content::{LoginParams, AuthResult};
use crate::logged_user_context::{LoggedUserContext, LoggedUserState};

#[derive(PartialEq, Properties)]
pub struct LoginModalProps {
    pub id: &'static str,
}

#[function_component(LoginModal)]
pub fn login_modal(props: &LoginModalProps) -> Html {
    let id = props.id;
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let close_node_ref = use_node_ref();

    {
        let logged_user_context = logged_user_context.clone();
        let close_node_ref = close_node_ref.clone();
        use_effect_with(logged_user_context, move |logged_user_context| {
            if let LoggedUserState::InProgress(login_params) = (**logged_user_context).state.clone() {
                let logged_user_context = logged_user_context.clone();
                let close_node_ref = close_node_ref.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let auth_result: AuthResult = Request::post("https://dummyjson.com/auth/login")
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_string(&login_params).unwrap())
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    match auth_result {
                        AuthResult::Success(auth_user) => {
                            close_node_ref.cast::<HtmlInputElement>().unwrap().click();
                            logged_user_context.dispatch(LoggedUserState::Active(auth_user));
                        },
                        AuthResult::Error { message } => {
                            logged_user_context.dispatch(LoggedUserState::Error(message));
                        },
                    }
                });
            }
            || ()
        });
    }

    let username_node_ref = use_node_ref();
    let password_node_ref = use_node_ref();

    /*let reload = use_force_update();
    {
        let logged_user_context = logged_user_context.clone();
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();
        let reload = reload.clone();
        use_effect_with(logged_user_context, move |logged_user_context| {
            match (**logged_user_context).state.clone() {
                LoggedUserState::None | LoggedUserState::Active(_) => {
                    reload.
                    // username_node_ref.cast::<HtmlInputElement>().unwrap().set_value("");
                    // password_node_ref.cast::<HtmlInputElement>().unwrap().set_value("");
                }
                LoggedUserState::InProgress(_) | LoggedUserState::Error(_) => {}
            }
            || ()
        });
    }*/

    let onclick = {
        let logged_user_context = logged_user_context.clone();
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();
        Callback::from(move |_event| {
            let username: String = username_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let password: String = password_node_ref.cast::<HtmlInputElement>().unwrap().value();
            logged_user_context.dispatch(LoggedUserState::InProgress(LoginParams { username, password }));
        })
    };

    html! {
        <div class="modal fade" { id } tabindex="-1" aria-labelledby="loginModalLabel" aria-hidden="true">
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
                        {
                            if let LoggedUserState::Error(message) = logged_user_context.state.clone() {
                                html! {
                                    <div class="alert alert-danger d-flex align-items-center" role="alert">
                                        { message }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
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
                            {
                                if let LoggedUserState::InProgress(_) = (*logged_user_context).state.clone() {
                                    html! {
                                        <>
                                            <div class="spinner-border spinner-border-sm" role="status">
                                                <span class="visually-hidden"> { "Загрузка..." } </span>
                                            </div>
                                            { " " }
                                        </>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                            { "Войти" }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}