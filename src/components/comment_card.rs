use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::author_image::*;
use crate::content;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq)]
enum CommentCardState {
    None,
    DeleteInProgress,
    Deleted,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct CommentCardProps {
    pub comment: Option<content::Comment>,
}

#[function_component(CommentCard)]
pub fn comment_card(props: &CommentCardProps) -> Html {
    let CommentCardProps { comment } = props.clone();

    let state = use_state_eq(|| CommentCardState::None);

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    #[cfg(feature = "client")]
    {
        let comment = comment.clone();
        let state = state.clone();
        let logged_user_context = logged_user_context.clone();
        use_effect_with(state, move |state| {
            if logged_user_context.is_not_inited() || **state != CommentCardState::DeleteInProgress
            {
                return;
            }

            let (Some(comment), Some(token)) = (comment, (*logged_user_context).token().cloned())
            else {
                return;
            };

            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let create_comment_request = content::API::<()>::get(content::Tokened {
                    token,
                    params: content::DeleteCommentParams {
                        comment_id: comment.id,
                    },
                });
                match create_comment_request.await {
                    Ok(create_comment_result) => match create_comment_result {
                        content::API::Success {
                            identifier: _,
                            description: _,
                            data: _,
                        } => {
                            state.set(CommentCardState::Deleted);
                            return;
                        }
                        content::API::Failure {
                            identifier: _,
                            reason: _,
                        } => {}
                    },
                    Err(_) => {}
                }
                state.set(CommentCardState::None);
            });
        });
    }

    let onclick = {
        let state = state.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if *state != CommentCardState::None {
                return;
            }
            state.set(CommentCardState::DeleteInProgress);
        })
    };

    html! {
        <div class="card mb-3">
            <div class="card-header placeholder-glow">
                <div class="row g-0 align-items-center">
                    <div class="d-flex col-6 align-items-center justify-content-start" style="height: 24px;">
                        <div class="img-block rounded me-1" style="height:24px;width:24px;overflow:hidden;">
                            <AuthorImage author={ comment.as_ref().map(|c| c.author.clone()) } />
                        </div>
                        if let Some(comment) = &comment {
                            <Link<Route, (), content::Author>
                                classes={ classes!("text-decoration-none") }
                                to={ Route::Author { slug: comment.author.slug.clone() } }
                                state={ Some(comment.author.clone()) }
                            >
                                <strong>
                                    { author_slug_formatter(&comment.author) }
                                </strong>
                            </Link<Route, (), content::Author>>
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </div>
                    <div class="d-flex col-6 align-items-center justify-content-end" style="height: 24px;">
                        if let Some(comment) = &comment {
                            <small> { date::format(comment.created_at) } </small>
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </div>
                </div>
            </div>
            <div class="card-body">
                <p class="card-text placeholder-glow">
                    if let Some(comment) = &comment {
                        if let Some(content) = &comment.content {
                            { render_text_with_newlines(content) }
                            if !logged_user_context.is_not_inited() {
                                if (*logged_user_context)
                                    .author()
                                    .map(|a| (a.editor == 1 || a.id == comment.author.id) && a.blocked == 0)
                                    .unwrap_or(false)
                                {
                                    { " " }
                                    {
                                        match *state {
                                            CommentCardState::None => html! {
                                                <a href="#" { onclick }>
                                                    <i title="Удалить комментарий" class="bi bi-trash"></i>
                                                </a>
                                            },
                                            CommentCardState::DeleteInProgress => html! {
                                                <i style="color:#8c8c8c;"> { "Удаление..." } </i>
                                            },
                                            CommentCardState::Deleted => html! {
                                                <i style="color:#8c8c8c;"> { "Удален!" } </i>
                                            },
                                        }
                                    }
                                }
                            }
                        } else {
                            <i style="color:#8c8c8c;"> { "Комментарий удален." } </i>
                        }
                    } else {
                        <span class="placeholder col-3 bg-secondary"></span> { " " }
                        <span class="placeholder col-4 bg-secondary"></span> { " " }
                        <span class="placeholder col-2 bg-secondary"></span> { " " }
                        <span class="placeholder col-2 bg-secondary"></span> { " " }
                        <span class="placeholder col-4 bg-secondary"></span> { " " }
                        <span class="placeholder col-2 bg-secondary"></span> { " " }
                        <span class="placeholder col-2 bg-secondary"></span> { " " }
                    }
                </p>
            </div>
        </div>
    }
}
