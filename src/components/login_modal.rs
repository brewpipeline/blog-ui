use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo_net::http::Request;

use crate::{LoggedUserContext, content::{LoginParams, AuthResult}};

#[derive(PartialEq, Clone)]
enum LoginState {
    Idle,
    InProgress(LoginParams),
    Error(String),
    Finished,
}

#[derive(PartialEq, Properties)]
pub struct LoginModalProps {
    pub id: &'static str,
}

#[function_component(LoginModal)]
pub fn login_modal(props: &LoginModalProps) -> Html {
    let id = props.id;
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let username_node_ref = use_node_ref();
    let password_node_ref = use_node_ref();
    let enter_node_ref = use_node_ref();
    let close_node_ref = use_node_ref();

    let login_state = use_state(|| LoginState::Idle);
    {
        let logged_user_context = logged_user_context.clone();
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();
        let enter_node_ref = enter_node_ref.clone();
        let close_node_ref = close_node_ref.clone();
        let login_state = login_state.clone();
        use_effect_with(login_state, move |login_state| {
            if let LoginState::InProgress(login_params) = (**login_state).clone() {
                username_node_ref.cast::<HtmlInputElement>().unwrap().set_disabled(true);
                password_node_ref.cast::<HtmlInputElement>().unwrap().set_disabled(true);
                enter_node_ref.cast::<HtmlInputElement>().unwrap().set_disabled(true);
                let logged_user_context = logged_user_context.clone();
                let username_node_ref = username_node_ref.clone();
                let password_node_ref = password_node_ref.clone();
                let close_node_ref = close_node_ref.clone();
                let login_state = login_state.clone();
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
                            logged_user_context.dispatch(Some(auth_user));
                            login_state.set(LoginState::Finished);
                        },
                        AuthResult::Error { message } => {
                            username_node_ref.cast::<HtmlInputElement>().unwrap().set_disabled(false);
                            password_node_ref.cast::<HtmlInputElement>().unwrap().set_disabled(false);
                            enter_node_ref.cast::<HtmlInputElement>().unwrap().set_disabled(false);
                            login_state.set(LoginState::Error(message));
                        },
                    }
                });
            }
            || ()
        });
    }

    let onclick = {
        let login_state = login_state.clone();
        let username_node_ref = username_node_ref.clone();
        let password_node_ref = password_node_ref.clone();
        Callback::from(move |_event| {
            let username: String = username_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let password: String = password_node_ref.cast::<HtmlInputElement>().unwrap().value();
            login_state.set(LoginState::InProgress(LoginParams { username, password }));
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
                        {
                            if let LoginState::Error(message) = (*login_state).clone() {
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
                            <input type="email" class="form-control" id="floatingInput" placeholder="Имя пользователя" value={ username_node_ref.cast::<HtmlInputElement>().map(|e| e.value()).unwrap_or("atuny0".to_string()) } ref={ username_node_ref } />
                            <label for="floatingInput"> { "Имя пользователя" } </label>
                        </div>
                        <div class="form-floating">
                            <input type="password" class="form-control" id="floatingPassword" placeholder="Password" value={ password_node_ref.cast::<HtmlInputElement>().map(|e| e.value()).unwrap_or("9uQFF1Lh".to_string()) } ref={ password_node_ref } />
                            <label for="floatingPassword"> { "Пароль" } </label>
                        </div>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" ref={ close_node_ref }> { "Закрыть" } </button>
                        <button type="button" class="btn btn-primary" { onclick } ref={ enter_node_ref }>
                            {
                                if let LoginState::InProgress(_) = (*login_state).clone() {
                                    html! {
                                        <>
                                            <div class="spinner-border spinner-border-sm" role="status">
                                                <span class="visually-hidden"> { "Loading..." } </span>
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