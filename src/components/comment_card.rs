use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::Comment;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct CommentCardProps {
    pub comment: Option<Comment>,
}

#[function_component(CommentCard)]
pub fn comment_card(props: &CommentCardProps) -> Html {
    let CommentCardProps { comment } = props.clone();

    let Some(comment) = comment else {
        return html! {
            <div class="card mb-3">
                <div class="card-header d-flex placeholder-glow">
                    <svg class="bd-placeholder-img rounded me-2" width="20" height="20" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" preserveAspectRatio="xMidYMid slice" focusable="false"><rect width="100%" height="100%" fill="#007aff"></rect></svg>
                    <span class="me-auto placeholder col-3"></span>
                    // <span class="placeholder col-3"></span>
                </div>
                <div class="card-body">
                    <p class="card-text placeholder-glow">
                        <span class="placeholder col-3"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                        <span class="placeholder col-2"></span> { " " }
                    </p>
                </div>
            </div>
        }
    };
    html! {
        <div class="card mb-3">
            <div class="card-header d-flex">
                <svg class="bd-placeholder-img rounded me-2" width="20" height="20" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" preserveAspectRatio="xMidYMid slice" focusable="false"><rect width="100%" height="100%" fill="#007aff"></rect></svg>
                <Link<Route> classes={classes!("text-decoration-none", "me-auto")} to={Route::Author { id: comment.short_user.id }}>
                    <strong>{ &comment.short_user.username }</strong>
                </Link<Route>>
                // <small> { "11 mins ago" } </small>
            </div>
            <div class="card-body">
                <p class="card-text">{ &comment.body }</p>
            </div>
        </div>
    }
}