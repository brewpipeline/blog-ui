#[cfg(feature = "client")]
use gloo::timers::callback::Timeout;
#[cfg(feature = "client")]
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::comment_card::*;
use crate::components::list::*;
use crate::components::simple_title_card::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq)]
enum CommentsState {
    NotReadyToSend,
    ReadyToSend,
    InProgress,
}

#[derive(PartialEq, Properties, Clone)]
pub struct CommentsProps {
    pub post: content::Post,
}

#[function_component(Comments)]
pub fn comments(props: &CommentsProps) -> Html {
    let CommentsProps { post } = props.clone();

    let state = use_state_eq(|| CommentsState::NotReadyToSend);

    let field_ref = use_node_ref();
    let button_ref = use_node_ref();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let request_index = use_state_eq::<u64, _>(|| 0);
    #[cfg(feature = "client")]
    {
        let request_index = request_index.clone();
        use_effect_with(request_index.clone(), |_| {
            let timeout = Timeout::new(30000, move || {
                request_index.set(*request_index + 1);
            });
            move || drop(timeout)
        });
    }

    #[cfg(feature = "client")]
    {
        let state = state.clone();
        let field_ref = field_ref.clone();
        let logged_user_context = logged_user_context.clone();
        let request_index = request_index.clone();
        use_effect_with(state, move |state| {
            if logged_user_context.is_not_inited() || **state != CommentsState::InProgress {
                return;
            }

            let Some(token) = (*logged_user_context).token().cloned() else {
                return;
            };

            let field = field_ref.cast::<HtmlInputElement>().unwrap();
            let state = state.clone();
            let request_index = request_index.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let create_comment_request = content::API::<()>::get(content::Tokened {
                    token,
                    params: content::CreateCommentParams {
                        comment: content::CommonComment {
                            post_id: post.id,
                            content: field.value(),
                        },
                    },
                });
                match create_comment_request.await {
                    Ok(delete_post_result) => match delete_post_result {
                        content::API::Success {
                            identifier: _,
                            description: _,
                            data: _,
                        } => {
                            state.set(CommentsState::NotReadyToSend);
                            field.set_value("");
                            request_index.set(*request_index + 1);
                            return;
                        }
                        content::API::Failure {
                            identifier: _,
                            reason: _,
                        } => {}
                    },
                    Err(_) => {}
                }
                state.set(CommentsState::ReadyToSend);
            });
        });
    }

    #[cfg(feature = "client")]
    let onclick = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if *state != CommentsState::ReadyToSend {
                return;
            }
            state.set(CommentsState::InProgress);
        })
    };
    #[cfg(not(feature = "client"))]
    let onclick = Callback::from(|_| {});

    #[cfg(feature = "client")]
    let oninput = {
        let state = state.clone();
        let field_ref = field_ref.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();

            if *state == CommentsState::InProgress {
                return;
            }

            let text = field_ref.cast::<HtmlInputElement>().unwrap().value();
            state.set(if text.is_empty() || text.len() > 500 {
                CommentsState::NotReadyToSend
            } else {
                CommentsState::ReadyToSend
            });
        })
    };
    #[cfg(not(feature = "client"))]
    let oninput = Callback::from(|_| {});

    html! {
        <>
            <SimpleTitleCard>
                { "Комментарии" }
            </SimpleTitleCard>
            <List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParams>
                r#type={ LoadType::Params(content::CommentsContainerPostIdParams {
                    post_id: post.id,
                    request_index: *request_index
                }) }
                items_per_page={ 50 }
                route_to_page={ Route::Post { slug: post.slug, id: post.id } }
                component={ |comment| html! { <CommentCard { comment } /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки комментариев!" /> } }
            >
                <Warning text="Нет комментариев." />
            </List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParams>>
            if !logged_user_context.is_not_inited() {
                if logged_user_context.author().map(|a| a.blocked == 0).unwrap_or(false) {
                    <div class="mb-3">
                        <textarea
                            class="form-control"
                            rows="3"
                            placeholder="Комментарий..."
                            ref={ field_ref }
                            { oninput }
                            disabled={ *state == CommentsState::InProgress }
                        ></textarea>
                    </div>
                    <div class="mb-3 d-grid gap-2">
                        <button
                            class="btn btn-light"
                            type="submit"
                            { onclick }
                            ref={ button_ref }
                            disabled={ *state != CommentsState::ReadyToSend }
                        >
                            { "Отправить" }
                        </button>
                    </div>
                }
            }
        </>
    }
}
