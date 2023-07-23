use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::warning::*;
use crate::content;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq)]
enum NewPostState {
    None,
    InProgress(content::NewPost),
    Error(String),
    Created(content::Post),
}

impl NewPostState {
    pub fn action_available(&self) -> bool {
        match self {
            NewPostState::None | NewPostState::Error(_) => true,
            NewPostState::InProgress(_) | NewPostState::Created(_) => false,
        }
    }
}

#[function_component(NewPost)]
pub fn new_post() -> Html {
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Новая публикация".to_string());

    let navigator = use_navigator().unwrap();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let state = use_state_eq(|| NewPostState::None);

    let title_node_ref = use_node_ref();
    let summary_node_ref = use_node_ref();
    let content_node_ref = use_node_ref();
    let tags_node_ref = use_node_ref();

    {
        let navigator = navigator.clone();
        let logged_user_context = logged_user_context.clone();
        let state = state.clone();
        use_effect_with(state, move |state| match (**state).clone() {
            NewPostState::None | NewPostState::Error(_) => {}
            NewPostState::InProgress(new_post) => {
                let LoggedUserState::Active { token } = logged_user_context.state.clone() else {
                    return
                };
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match content::API::<content::NewPostContainer>::get(content::TokenParam {
                        token,
                        data: new_post,
                    })
                    .await
                    {
                        Ok(new_post_result) => match new_post_result {
                            content::API::Success {
                                identifier: _,
                                description: _,
                                data: content::NewPostContainer { created_post: post },
                            } => {
                                state.set(NewPostState::Created(post.clone()));
                            }
                            content::API::Failure { identifier, reason } => {
                                state.set(NewPostState::Error(reason.unwrap_or(identifier)));
                            }
                        },
                        Err(err) => {
                            state.set(NewPostState::Error(err.to_string()));
                        }
                    }
                });
            }
            NewPostState::Created(post) => navigator.push_with_state(
                &Route::Post {
                    slug: post.slug.clone(),
                },
                post,
            ),
        })
    }

    let onclick = {
        let state = state.clone();
        let title_node_ref = title_node_ref.clone();
        let summary_node_ref = summary_node_ref.clone();
        let content_node_ref = content_node_ref.clone();
        let tags_node_ref = tags_node_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let title = title_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let summary = summary_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let content = content_node_ref
                .cast::<HtmlInputElement>()
                .map(|v| v.value())
                .filter(|s| !s.is_empty());
            let tags = tags_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .value()
                .split(',')
                .map(|t| t.trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect();
            state.set(NewPostState::InProgress(content::NewPost {
                title: title.clone(),
                slug: title, // KONCH
                published: 1,
                summary,
                content,
                tags,
            }))
        })
    };

    html! {
        <>
            if let LoggedUserState::Active { token: _ } = logged_user_context.state.clone() {
                <form /*class="was-validated"*/>
                    <h5 class="mb-3">
                        { "Новая публикация" }
                    </h5>

                    if let NewPostState::Error(message) = (*state).clone() {
                        <div class="alert alert-danger d-flex align-items-center" role="alert">
                            { "Ошибка добавления публикации: " }
                            { message }
                        </div>
                    }

                    <div class="mb-4 border rounded-3 d-flex align-items-center justify-content-center p-3 py-6" style="font-size: 10em" role="img">
                        <svg style="width: 1em; height: 1em;" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-file-post" viewBox="0 0 16 16">
                            <path d="M4 3.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 0 1h-5a.5.5 0 0 1-.5-.5zm0 2a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 .5.5v8a.5.5 0 0 1-.5.5h-7a.5.5 0 0 1-.5-.5v-8z"></path>
                            <path d="M2 2a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2zm10-1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1z"></path>
                        </svg>
                    </div>

                    <div class="mb-3">
                        <label for="validationTitle1" class="form-label"> { "Заголовок" } </label>
                        <input type="text" class="form-control" id="validationTitle1" placeholder="Что-то захватывающее внимание..." required=true ref={ title_node_ref } />
                        <div class="invalid-feedback">
                            { "Пожалуйста, введите заголовок публикации, это обязательное поле!" }
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="validationTextarea1" class="form-label"> { "Короткая версия" } </label>
                        <textarea class="form-control" id="validationTextarea1" placeholder="Что-то короткое, но важное!" required=true ref={ summary_node_ref }></textarea>
                        <div class="invalid-feedback">
                            { "Пожалуйста, введите короткую версию публикации, это обязательное поле!" }
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="validationTextarea2" class="form-label"> { "Полная версия" } </label>
                        <textarea class="form-control" id="validationTextarea2" placeholder="Что-то динное и скучн... веселое!" ref={ content_node_ref } ></textarea>
                    </div>

                    <div class="mb-3">
                        <label for="validationTitle2" class="form-label"> { "Теги (через `,`)" } </label>
                        <input type="text" class="form-control" id="validationTitle2" placeholder="Что-то напоминающее о..." ref={ tags_node_ref } />
                    </div>

                    /*
                    <div class="form-check mb-3">
                        <input type="checkbox" class="form-check-input" id="validationFormCheck1" required=true />
                        <label class="form-check-label" for="validationFormCheck1"> { "Все проверено?" } </label>
                        <div class="invalid-feedback"> { "Убедитесь, все ли поля заполнены корректными значениями\\данными" } </div>
                    </div>
                    */

                    <div class="mb-3">
                        <button class="btn btn-light" type="submit" { onclick } disabled={ !state.action_available() }> { "Отправить" } </button>
                    </div>
                </form>
            } else {
                <Warning text="Создавать публикации можно только авторизованным авторам" />
            }
        </>
    }
}
