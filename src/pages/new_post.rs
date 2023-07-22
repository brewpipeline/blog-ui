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
    InProgress {
        token: String,
        new_post: content::NewPost,
    },
    Error(String),
    Created(content::Post),
}

impl NewPostState {
    pub fn action_available(&self) -> bool {
        match self {
            NewPostState::None | NewPostState::Error(_) => true,
            NewPostState::InProgress {
                token: _,
                new_post: _,
            }
            | NewPostState::Created(_) => false,
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
        let state = state.clone();
        use_effect_with_deps(
            move |state| {
                let NewPostState::InProgress { token, new_post } = (**state).clone() else {
                    return
                };
                let navigator = navigator.clone();
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match content::API::<content::PostContainer>::get(content::TokenParam {
                        token,
                        data: new_post,
                    })
                    .await
                    {
                        Ok(auth_result) => match auth_result {
                            content::API::Success {
                                identifier: _,
                                description: _,
                                data: content::PostContainer { post },
                            } => {
                                let post_slug = post.slug.clone();
                                state.set(NewPostState::Created(post));
                                navigator.push(&Route::Post { slug: post_slug })
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
            },
            state,
        )
    }

    let onclick = {
        let logged_user_context = logged_user_context.clone();
        let state = state.clone();
        let title_node_ref = title_node_ref.clone();
        let summary_node_ref = summary_node_ref.clone();
        let content_node_ref = content_node_ref.clone();
        let tags_node_ref = tags_node_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let LoggedUserState::Active { token } = logged_user_context.state.clone() else {
                return
            };
            let title = title_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let summary = summary_node_ref.cast::<HtmlInputElement>().unwrap().value();
            let content = content_node_ref
                .cast::<HtmlInputElement>()
                .map(|v| v.value());
            let tags = tags_node_ref.cast::<HtmlInputElement>().unwrap().value();
            state.set(NewPostState::InProgress {
                token,
                new_post: content::NewPost {
                    title: title.clone(),
                    slug: title,
                    published: 1,
                    summary: summary,
                    content: content,
                    tags: tags.split(',').map(|t| t.trim().to_owned()).collect(),
                },
            })
        })
    };

    html! {
        <>
            if let LoggedUserState::Active { token: _ } = logged_user_context.state.clone() {
                <form class="was-validated">
                    <h5 class="mb-3">
                        { "Новая публикация" }
                    </h5>

                    <div class="mb-3">
                        <label for="validationTitle1" class="form-label"> { "Заголовок" } </label>
                        <input type="text" class="form-control" id="validationTitle1" placeholder="Что-то захватывающее внимание..." required=true ref={ title_node_ref } />
                        <div class="invalid-feedback">
                            { "Заголовок это обязательное поле!" }
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="validationTextarea1" class="form-label"> { "Короткая версия" } </label>
                        <textarea class="form-control" id="validationTextarea1" placeholder="Что-то короткое, но важное!" required=true ref={ summary_node_ref }></textarea>
                        <div class="invalid-feedback">
                            { "Пожалуйста введите короткую версию публикации, это обязательное поле!" }
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

                    <div class="form-check mb-3">
                        <input type="checkbox" class="form-check-input" id="validationFormCheck1" required=true />
                        <label class="form-check-label" for="validationFormCheck1"> { "Все проверено?" } </label>
                        <div class="invalid-feedback"> { "Проверьте или все заполнено и верно" } </div>
                    </div>

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
