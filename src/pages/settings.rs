use blog_generic::entities::LoginTelegramQuestion;
#[cfg(feature = "client")]
use gloo::events::EventListener;
use noneifempty::NoneIfEmpty;
#[cfg(all(feature = "client", any(feature = "yandex", feature = "telegram")))]
use wasm_bindgen::JsCast;
#[cfg(all(feature = "client", any(feature = "yandex", feature = "telegram")))]
use web_sys::CustomEvent;
#[cfg(feature = "client")]
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew_alt_html::*;

use crate::components::meta::*;
#[cfg(feature = "telegram")]
use crate::components::telegram_button::*;
use crate::components::warning::*;
#[cfg(feature = "client")]
use crate::content::*;
use crate::utils::*;

#[derive(Clone, PartialEq)]
enum ActiveSection {
    None,
    Social,
    Custom,
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let in_progress = use_state(|| false);

    let main_active_section = use_state(|| ActiveSection::None);

    let main_section_error = use_state::<Option<Result<String, String>>, _>(|| None);

    let settings_node_ref = use_node_ref();

    let slug_node_ref = use_node_ref();
    let first_name_node_ref = use_node_ref();
    let last_name_node_ref = use_node_ref();
    let image_url_node_ref = use_node_ref();

    #[cfg(feature = "client")]
    {
        let logged_user_context = logged_user_context.clone();
        let main_active_section = main_active_section.clone();
        use_effect_with(logged_user_context, move |logged_user_context| {
            if logged_user_context.is_not_inited() {
                return;
            }
            main_active_section.set(
                if let LoggedUserState::ActiveAndLoaded { token, author } =
                    logged_user_context.state().clone()
                {
                    if author.override_social_data == 0 {
                        ActiveSection::Social
                    } else {
                        ActiveSection::Custom
                    }
                } else {
                    ActiveSection::None
                },
            )
        });
    }

    #[cfg(all(feature = "client", feature = "telegram"))]
    {
        let logged_user_context = logged_user_context.clone();
        let in_progress = in_progress.clone();
        let main_section_error = main_section_error.clone();
        let settings_node_ref = settings_node_ref.clone();
        use_effect_with(logged_user_context, move |logged_user_context| {
            let settings_element = settings_node_ref.cast::<HtmlElement>().unwrap();

            let data_listener = {
                let logged_user_context = logged_user_context.clone();
                EventListener::new(&settings_element, "telegram.reauth.data", move |e| {
                    if *in_progress || logged_user_context.is_not_inited() {
                        return;
                    }
                    main_section_error.set(None);
                    in_progress.set(true);
                    let e = e.dyn_ref::<CustomEvent>().unwrap();
                    let Some(login_telegram_question) = e
                        .detail()
                        .as_string()
                        .map(|j| serde_json::from_str::<LoginTelegramQuestion>(j.as_str()).ok())
                        .flatten()
                    else {
                        main_section_error
                            .set(Some(Err("incorrect data from telegram".to_string())));
                        in_progress.set(false);
                        return;
                    };
                    let Some(token) = logged_user_context.state().token().cloned() else {
                        main_section_error.set(Some(Err("currently not logged in".to_string())));
                        in_progress.set(false);
                        return;
                    };
                    let logged_user_context = logged_user_context.clone();
                    let in_progress = in_progress.clone();
                    let main_section_error = main_section_error.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match API::<()>::get(Tokened {
                            token,
                            params: AuthorResetOverrideSocialData,
                        })
                        .await
                        {
                            Ok(result) => match result {
                                API::Success {
                                    identifier: _,
                                    description: _,
                                    data: _,
                                } => {
                                    logged_user_context.dispatch(LoggedUserState::InProgress(
                                        InProgressStateType::Telegram(login_telegram_question),
                                    ));
                                    main_section_error
                                        .set(Some(Ok("telegram data applied".to_string())));
                                }
                                API::Failure { identifier, reason } => {
                                    main_section_error.set(Some(Err(reason.unwrap_or(identifier))));
                                }
                            },
                            Err(err) => {
                                main_section_error.set(Some(Err(err.to_string())));
                            }
                        }
                        in_progress.set(false);
                    });
                })
            };
            move || drop(data_listener)
        });
    }

    #[cfg(feature = "client")]
    let onclick = {
        let logged_user_context = logged_user_context.clone();
        let in_progress = in_progress.clone();
        let main_section_error = main_section_error.clone();
        let slug_node_ref = slug_node_ref.clone();
        let first_name_node_ref = first_name_node_ref.clone();
        let last_name_node_ref = last_name_node_ref.clone();
        let image_url_node_ref = image_url_node_ref.clone();
        Callback::from(move |_event: MouseEvent| {
            if *in_progress || logged_user_context.is_not_inited() {
                return;
            }
            main_section_error.set(None);
            in_progress.set(true);
            let slug = slug_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let first_name = first_name_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .none_if_empty();
            let last_name = last_name_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .none_if_empty();
            let image_url = image_url_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .none_if_empty();
            let Some(token) = logged_user_context.state().token().cloned() else {
                main_section_error.set(Some(Err("currently not logged in".to_string())));
                in_progress.set(false);
                return;
            };
            let logged_user_context = logged_user_context.clone();
            let in_progress = in_progress.clone();
            let main_section_error = main_section_error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match API::<()>::get(Tokened {
                    token: token.clone(),
                    params: UpdateMinimalAuthor {
                        update_minimal_author: CommonMinimalAuthor {
                            slug,
                            first_name,
                            last_name,
                            image_url,
                        },
                    },
                })
                .await
                {
                    Ok(result) => match result {
                        API::Success {
                            identifier: _,
                            description: _,
                            data: _,
                        } => {
                            logged_user_context.dispatch(LoggedUserState::Active { token });
                            main_section_error.set(Some(Ok("custom data applied".to_string())));
                        }
                        API::Failure { identifier, reason } => {
                            main_section_error.set(Some(Err(reason.unwrap_or(identifier))));
                        }
                    },
                    Err(err) => {
                        main_section_error.set(Some(Err(err.to_string())));
                    }
                }
                in_progress.set(false);
            })
        })
    };
    #[cfg(not(feature = "client"))]
    let onclick = Callback::from(|_| {});

    #[cfg(feature = "telegram")]
    let telegram_button = ah! {
        <TelegramButton onauth="document.getElementById('settingsPage').dispatchEvent(new CustomEvent('telegram.reauth.data', {detail: JSON.stringify(user)}))" />
    };
    #[cfg(not(feature = "telegram"))]
    let telegram_button = ah! {};

    ah! {
        <Meta title={ "Настройки" } noindex=true />
        <div id="settingsPage" ref={ settings_node_ref }>
            if !logged_user_context.is_not_inited() && !logged_user_context.state().action_available() {
                <div class="card mb-3">
                    <div class="card-body">
                        <h5 class="card-title placeholder-glow mb-3">
                            "Настройки"
                        </h5>
                        <div class="col-12 col-lg-10 col-xl-8">
                            <h6 class="card-title placeholder-glow mb-3">
                                "Основные данные профиля"
                            </h6>
                            if let Some(message) = main_section_error.as_ref() {
                                match message {
                                    Ok(ok_message) => {
                                        <div class="alert alert-success d-flex align-items-center" role="alert">
                                            { "Данные успешно обновлены: " }
                                            { ok_message }
                                        </div>
                                    },
                                    Err(err_message) => {
                                        <div class="alert alert-danger d-flex align-items-center" role="alert">
                                            { "Ошибка обновления данных: " }
                                            { err_message }
                                        </div>
                                    }
                                }
                            }
                            <div class="mb-3">
                                <div class="form-check mb-2">
                                    <input
                                        class="form-check-input"
                                        type="radio"
                                        name="flexRadioDefault"
                                        id="flexRadioDefault1"
                                        disabled=true
                                        checked={ *main_active_section == ActiveSection::Social }
                                        onclick={
                                            let main_active_section = main_active_section.clone();
                                            Callback::from(move |_: MouseEvent| main_active_section.set(ActiveSection::Social))
                                        }
                                    />
                                    <label class="form-check-label mb-2" for="flexRadioDefault1">
                                        "Использовать данные Telegram (используйте кнопку ниже, чтобы выбрать этот пункт)"
                                    </label>
                                    <div class="mb-2">
                                        <div style={ if !*in_progress { "" } else { "pointer-events: none;" } }>
                                            { telegram_button }
                                        </div>
                                        <div class="form-text">"Также используйте кнопку для синхронизации данныx."</div>
                                    </div>
                                </div>
                            </div>
                            <div class="mb-3">
                                <div class="form-check mb-2">
                                    <input
                                        class="form-check-input"
                                        type="radio"
                                        name="flexRadioDefault"
                                        id="flexRadioDefault2"
                                        disabled={ *main_active_section == ActiveSection::None|| *in_progress }
                                        checked={ *main_active_section == ActiveSection::Custom }
                                        onclick={
                                            let main_active_section = main_active_section.clone();
                                            Callback::from(move |_: MouseEvent| main_active_section.set(ActiveSection::Custom))
                                        }
                                    />
                                    <label class="form-check-label mb-2" for="flexRadioDefault2">
                                        "Использовать пользовательские данные"
                                    </label>
                                    <div class="form-floating mb-2">
                                        <input
                                            ref={ slug_node_ref }
                                            type="text"
                                            class="form-control"
                                            id="floatingInput1"
                                            placeholder="Имя профиля (уникальное)"
                                            disabled={ *main_active_section != ActiveSection::Custom|| *in_progress }
                                        />
                                        <label for="floatingInput1">"Имя аккаунта (уникальное)"</label>
                                    </div>
                                    <div class="form-floating mb-2">
                                        <input
                                            ref={ image_url_node_ref }
                                            type="text"
                                            class="form-control"
                                            id="floatingInput2"
                                            placeholder="Изображение профиля (ссылка)"
                                            disabled={ *main_active_section != ActiveSection::Custom || *in_progress }
                                        />
                                        <label for="floatingInput2">"Изображение профиля (ссылка)"</label>
                                    </div>
                                    <div class="form-floating mb-2">
                                        <input
                                            ref={ first_name_node_ref }
                                            type="text"
                                            class="form-control"
                                            id="floatingInput3"
                                            placeholder="Имя"
                                            disabled={ *main_active_section != ActiveSection::Custom|| *in_progress }
                                        />
                                        <label for="floatingInput3">"Имя"</label>
                                    </div>
                                    <div class="form-floating mb-2">
                                        <input
                                            ref={ last_name_node_ref }
                                            type="text"
                                            class="form-control"
                                            id="floatingInput4"
                                            placeholder="Фамилия"
                                            disabled={ *main_active_section != ActiveSection::Custom|| *in_progress }
                                        />
                                        <label for="floatingInput4">"Фамилия"</label>
                                    </div>
                                    <button
                                        type="button"
                                        class="btn btn-primary"
                                        { onclick }
                                        disabled={ *main_active_section != ActiveSection::Custom || *in_progress }
                                    >
                                        { "Сохранить" }
                                        if *in_progress {
                                            { " " }
                                            <div class="spinner-border spinner-border-sm" role="status">
                                                <span class="visually-hidden"> { "Загрузка..." } </span>
                                            </div>
                                        }
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            } else {
                <Warning text="Настройки доступны только авторизованным авторам!" />
            }
        </div>
    }
}
