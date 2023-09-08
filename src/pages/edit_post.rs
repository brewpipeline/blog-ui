#[cfg(feature = "client")]
use gloo::utils::document;
#[cfg(feature = "client")]
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::delayed_component::*;
use crate::components::item::*;
use crate::components::meta::*;
use crate::components::svg_image::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::*;

#[cfg(feature = "client")]
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq)]
enum EditPostState {
    None,
    EditError(String),
    EditedInProgress(content::CommonPost),
    Edited(content::Post),
    DeleteError(String),
    DeleteInProgress,
    Deleted,
}

impl EditPostState {
    pub fn action_available(&self) -> bool {
        match self {
            EditPostState::None | EditPostState::EditError(_) | EditPostState::DeleteError(_) => {
                true
            }
            EditPostState::EditedInProgress(_)
            | EditPostState::Edited(_)
            | EditPostState::DeleteInProgress
            | EditPostState::Deleted => false,
        }
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct EditPostProps {
    pub id: Option<u64>,
}

#[function_component(EditPost)]
pub fn edit_post(props: &EditPostProps) -> Html {
    let EditPostProps { id } = props.clone();

    let meta = html! {
        <Meta title={
            if id == None {
                "Новая публикация"
            } else {
                "Редактирование публикации"
            }
        } />
    };

    let navigator = use_navigator().unwrap();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let state = use_state_eq(|| EditPostState::None);

    let image_node_ref = use_node_ref();
    let title_node_ref = use_node_ref();
    let summary_node_ref = use_node_ref();
    let content_node_ref = use_node_ref();
    let tags_node_ref = use_node_ref();
    let published_node_ref = use_node_ref();

    #[cfg(feature = "client")]
    {
        let navigator = navigator.clone();
        let logged_user_context = logged_user_context.clone();
        let state = state.clone();
        use_effect_with(state, move |state| match (**state).clone() {
            EditPostState::None
            | EditPostState::EditError(_)
            | EditPostState::DeleteError(_)
            | EditPostState::Deleted => {}
            EditPostState::EditedInProgress(common_post) => {
                let Some(token) = logged_user_context.state.token().cloned() else {
                    return;
                };
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let edit_post_request = if let Some(id) = id {
                        content::API::<content::PostContainer>::get(content::Tokened {
                            token,
                            params: content::UpdatePostParams {
                                id,
                                update_post: common_post,
                            },
                        })
                    } else {
                        content::API::<content::PostContainer>::get(content::Tokened {
                            token,
                            params: content::NewPostParams {
                                new_post: common_post,
                            },
                        })
                    };
                    match edit_post_request.await {
                        Ok(new_post_result) => match new_post_result {
                            content::API::Success {
                                identifier: _,
                                description: _,
                                data: content::PostContainer { post },
                            } => {
                                state.set(EditPostState::Edited(post.clone()));
                            }
                            content::API::Failure { identifier, reason } => {
                                state.set(EditPostState::EditError(reason.unwrap_or(identifier)));
                            }
                        },
                        Err(err) => {
                            state.set(EditPostState::EditError(err.to_string()));
                        }
                    }
                });
            }
            EditPostState::Edited(post) => navigator.push_with_state(
                &Route::Post {
                    slug: post.slug.clone(),
                    id: post.id,
                },
                post,
            ),
            EditPostState::DeleteInProgress => {
                let (Some(id), Some(token)) = (id, logged_user_context.state.token().cloned())
                else {
                    return;
                };
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let delete_post_request = content::API::<()>::get(content::Tokened {
                        token,
                        params: content::DeletePostParams { id },
                    });
                    match delete_post_request.await {
                        Ok(delete_post_result) => match delete_post_result {
                            content::API::Success {
                                identifier: _,
                                description: _,
                                data: _,
                            } => {
                                state.set(EditPostState::Deleted);
                            }
                            content::API::Failure { identifier, reason } => {
                                state.set(EditPostState::DeleteError(reason.unwrap_or(identifier)));
                            }
                        },
                        Err(err) => {
                            state.set(EditPostState::DeleteError(err.to_string()));
                        }
                    }
                });
            }
        })
    }

    let LoggedUserState::ActiveAndLoaded { token: _, author } = logged_user_context.state.clone()
    else {
        return html! {
            <>
                { meta }
                <Warning text={
                    if id == None {
                        "Создавать публикации можно только авторизованным авторам!"
                    } else {
                        "Редактировать публикации можно только авторизованным авторам!"
                    }
                } />
            </>
        };
    };

    if let EditPostState::Deleted = *state {
        return html! {
            <div class="alert alert-success d-flex align-items-center" role="alert">
                { "Публикация удалена!" }
            </div>
        };
    }

    #[cfg(feature = "client")]
    let save_onclick = {
        let state = state.clone();
        let image_node_ref = image_node_ref.clone();
        let title_node_ref = title_node_ref.clone();
        let summary_node_ref = summary_node_ref.clone();
        let content_node_ref = content_node_ref.clone();
        let tags_node_ref = tags_node_ref.clone();
        let published_node_ref = published_node_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            let image_url = image_node_ref
                .cast::<HtmlInputElement>()
                .map(|v| v.value())
                .filter(|s| !s.is_empty());
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
            let published = published_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .checked() as u8;
            state.set(EditPostState::EditedInProgress(content::CommonPost {
                title,
                published,
                summary,
                content,
                tags,
                image_url,
            }));
        })
    };
    #[cfg(not(feature = "client"))]
    let save_onclick = Callback::from(|_| {});

    let delete_onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(EditPostState::DeleteInProgress);
        })
    };

    let main_content = Callback::from(move |post: Option<content::Post>| {
        let post_image = post.as_ref().map(|p| p.image_url.clone()).flatten();
        let post_title = post.as_ref().map(|p| p.title.clone());
        let post_summary = post.as_ref().map(|p| p.summary.clone());
        let post_content = post.as_ref().map(|p| p.content.clone()).flatten();
        let post_tags = post.as_ref().map(|p| p.joined_tags_string(", "));
        #[cfg(feature = "client")]
        let post_image = image_node_ref
            .cast::<HtmlInputElement>()
            .map(|h| h.value())
            .or(post_image);
        #[cfg(feature = "client")]
        let post_title = title_node_ref
            .cast::<HtmlInputElement>()
            .map(|h| h.value())
            .or(post_title);
        #[cfg(feature = "client")]
        let post_summary = summary_node_ref
            .cast::<HtmlInputElement>()
            .map(|h| h.value())
            .or(post_summary);
        #[cfg(feature = "client")]
        let post_content = content_node_ref
            .cast::<HtmlInputElement>()
            .map(|h| h.value())
            .or(post_content);
        #[cfg(feature = "client")]
        let post_tags = tags_node_ref
            .cast::<HtmlInputElement>()
            .map(|h| h.value())
            .or(post_tags);
        html! {
            <>
                <form>
                    <h5 class="mb-3">
                        if let Some(post) = post.as_ref() {
                            { "Редактирование публикации: " }
                            { post.title.clone() }
                        } else {
                            { "Новая публикация" }
                        }
                    </h5>

                    <div
                        class="mb-4 border rounded-3 d-flex align-items-center justify-content-center p-3 py-6"
                        style="font-size: 10em"
                        role="img"
                    >
                        <FilePostImg />
                    </div>

                    <div class="mb-3">
                        <label for="validationImage1" class="form-label">
                            { "Изображение (Cсылка) (Опциональное)" }
                        </label>
                        <input
                            type="text"
                            class="form-control"
                            id="validationImage1"
                            placeholder="Что-то визуально приятное..."
                            value={ post_image }
                            ref={ image_node_ref.clone() }
                        />
                    </div>

                    <div class="mb-3">
                        <label for="validationTitle1" class="form-label">
                            { "Заголовок" }
                        </label>
                        <input
                            type="text"
                            class="form-control"
                            id="validationTitle1"
                            placeholder="Что-то захватывающее внимание..."
                            value={ post_title }
                            ref={ title_node_ref.clone() }
                        />
                        <div class="invalid-feedback">
                            { "Пожалуйста, введите заголовок публикации, это обязательное поле!" }
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="validationTextarea1" class="form-label">
                            { "Короткая версия" }
                        </label>
                        <textarea
                            class="form-control"
                            id="validationTextarea1"
                            placeholder="Что-то короткое, но важное!"
                            value={ post_summary }
                            ref={ summary_node_ref.clone() }
                        ></textarea>
                        <div class="invalid-feedback">
                            { "Пожалуйста, введите короткую версию публикации, это обязательное поле!" }
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="validationTextarea2" class="form-label">
                            { "Полная версия (Опциональное)" }
                        </label>
                        <textarea
                            class="form-control"
                            id="validationTextarea2"
                            placeholder="Что-то динное и скучн... веселое!"
                            value={ post_content }
                            ref={ content_node_ref.clone() }
                        ></textarea>
                    </div>

                    <div class="mb-3">
                        <label for="validationTitle2" class="form-label">
                            { "Теги (через `,`) (Опциональное)" }
                        </label>
                        <input
                            type="text"
                            class="form-control"
                            id="validationTitle2"
                            placeholder="Что-то напоминающее о..."
                            value={ post_tags }
                            ref={ tags_node_ref.clone() }
                        />
                    </div>

                    <div class="form-check mb-3">
                        <input
                            type="checkbox"
                            class="form-check-input"
                            id="validationFormCheck1"
                            ref={ published_node_ref.clone() }
                        />
                        <label class="form-check-label" for="validationFormCheck1">
                            { "Опубликовать" }
                        </label>
                    </div>

                    if let EditPostState::EditError(message) = (*state).clone() {
                        <div class="alert alert-danger d-flex align-items-center" role="alert">
                            if post == None {
                                { "Ошибка добавления публикации: " }
                            } else {
                                { "Ошибка редактирования публикации: " }
                            }
                            { message }
                        </div>
                    }

                    if let EditPostState::DeleteError(message) = (*state).clone() {
                        <div class="alert alert-danger d-flex align-items-center" role="alert">
                            { "Ошибка удаления публикации: " }
                            { message }
                        </div>
                    }

                    <div class="mb-3">
                        <button
                            class="btn btn-light"
                            type="submit"
                            onclick={ save_onclick.clone() }
                            disabled={ !state.action_available() }
                        >
                            { "Сохранить" }
                        </button>
                        if post != None {
                            <button
                                class="btn btn-danger float-end"
                                onclick={ delete_onclick.clone() }
                                disabled={ !state.action_available() }
                            >
                                { "Удалить" }
                            </button>
                        }
                    </div>
                </form>
                <script id="tinymceScript" src="https://cdn.tiny.cloud/1/o40kmyk68yh2km1wxwntfx69sodri6r5jrqfip730ivhfcb7/tinymce/6/tinymce.min.js" referrerpolicy="origin"></script>
                <DelayedComponent<()> component={ |_| {
                    #[cfg(feature = "client")]
                    {
                        let script: Element = document().create_element("script").unwrap();
                        script.set_attribute("type", "text/javascript").unwrap();
                        script.set_inner_html("
                            document.getElementById('tinymceScript').onload = function() {
                                tinymce.remove();
                                tinymce.init({
                                    selector: '#validationTextarea2',
                                    setup: function(editor) {
                                        editor.on('Change Keyup', function () {
                                            editor.save();
                                        });
                                    },
                                    language: 'ru',
                                    plugins: 'anchor autolink charmap codesample emoticons image link lists media searchreplace table visualblocks wordcount code codesample',
                                });
                            }
                        ");
                        Html::VRef(script.into())
                    }
                    #[cfg(not(feature = "client"))]
                    unreachable!()
                }} deps={ () } />
            </>
        }
    });

    html! {
        <>
            { meta }
            if let Some(id) = id {
                <Item<content::API<content::PostContainer>, content::PostParams>
                    params={ content::PostParams { id } }
                    use_caches=true
                    component={ move |post: Option<content::Post>| {
                        if let Some(post) = post {
                            if post.author.slug == author.slug {
                                main_content.emit(Some(post))
                            } else {
                                html! { <Warning text="Только автор может редактировать публикацию!" /> }
                            }
                        } else {
                            html! { <Warning text="Загрузка публикации для редактирования..." /> }
                        }
                    } }
                    error_component={ |_| html! { <Warning text="Ошибка загрузки публикации для редактирования!" /> } }
                />
            } else {
                { main_content.emit(None) }
            }
        </>
    }
}
