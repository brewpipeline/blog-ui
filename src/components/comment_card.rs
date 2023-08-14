use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::svg_image::*;
use crate::content::*;

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
            <div class="card-header d-flex placeholder-glow">
                <PlaceholderImg />
                if let Some(comment) = &comment {
                    <Link<Route, (), Author>
                        classes={ classes!("text-decoration-none", "me-auto") }
                        to={ Route::Author { slug: comment.author.slug.clone() } }
                        state={ Some(comment.author.clone()) }
                    >
                        <strong>
                            { &comment.author.slug }
                        </strong>
                    </Link<Route, (), Author>>
                } else {
                    <span class="me-auto placeholder col-3"></span>
                }
                // <small> { "11 mins ago" } </small>
                // OR
                // <span class="placeholder col-3"></span>
            </div>
            <div class="card-body">
                <p class="card-text placeholder-glow">
                    if let Some(comment) = &comment {
                        { &comment.content }
                    } else {
                        <span class="placeholder col-3"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                    }
                </p>
            </div>
        </div>
    }
}
