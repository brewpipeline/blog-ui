use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct CommentCardProps {
    pub comment: Option<Comment>,
}

#[function_component(CommentCard)]
pub fn comment_card(props: &CommentCardProps) -> Html {
    let CommentCardProps { comment } = props.clone();
    html! {
        <div class="card mb-3">
            <div class="card-header placeholder-glow">
                <div class="row align-items-center">
                    <div class="d-flex col align-items-center justify-content-start" style="height: 24px;">
                        <img
                            width="24"
                            height="24"
                            src={
                                comment
                                    .as_ref()
                                    .map(|c| c.author.image_url())
                                    .unwrap_or_default()
                            }
                            alt={
                                comment
                                    .as_ref()
                                    .map(|c| c.author.slug.clone())
                            }
                            class="img-block rounded me-1"
                        />
                        if let Some(comment) = &comment {
                            <Link<Route, (), Author>
                                classes={ classes!("text-decoration-none") }
                                to={ Route::Author { slug: comment.author.slug.clone() } }
                                state={ Some(comment.author.clone()) }
                            >
                                <strong>
                                    { &comment.author.slug }
                                </strong>
                            </Link<Route, (), Author>>
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </div>
                    <div class="d-flex col align-items-center justify-content-end" style="height: 24px;">
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
                        { &comment.content }
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
