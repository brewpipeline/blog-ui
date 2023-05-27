use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::item::*;
use crate::content::{Post, User};

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PostCardProps {
    pub post: Option<Post>,
    #[prop_or_default]
    pub fetch_author: bool,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let PostCardProps { post, fetch_author } = props.clone();

    let route = use_route::<Route>().unwrap_or_default();

    let Some(post) = post else {
        return html! {
            <div class="card mb-3">
                <div style="height:180px;width:100%;" class="img-block bd-placeholder-img card-img-top" role="img" />
                <div class="card-body">
                    <h5 class="card-title placeholder-glow">
                        <span class="placeholder col-6"></span>
                    </h5>
                    <p class="card-text placeholder-glow">
                        <span class="placeholder col-7"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-6"></span> { " " }
                        <span class="placeholder col-8"></span> { " " }
                    </p>
                </div>
                <div class="card-footer">
                    <div class="row align-items-center">
                        <div class="col">
                            <span class="placeholder col-4"></span>
                        </div>
                        if fetch_author {
                            <div class="col text-end">
                                <span class="placeholder col-4"></span>
                            </div>
                        }
                    </div>
                </div>
            </div>
        }
    };
    let main_content = html! {
        <>
            <div style={ format!("height:180px;width:100%;--image-url:url({});", post.image_url()) } class="img-block bd-placeholder-img card-img-top" role="img" />
            <div class="card-body">
                <h5 class="card-title">{ &post.title }</h5>
                <p class="card-text">{ &post.body }</p>
            </div>
        </>
    };
    let card_route = Route::Post { id: post.id };
    html! {
        <div class="card mb-3">
            if route != card_route {
                <Link<Route> classes={classes!("text-decoration-none")} to={ card_route }>
                    { main_content }
                </Link<Route>>
            } else {
                { main_content }
            }
            <div class="card-footer">
                <div class="row align-items-center">
                    <div class="col">
                        { post.tags.join(", ") }
                    </div>
                    if fetch_author {
                        <div class="col text-end">
                            <Item<User> item_id={ post.user_id } component={ move |user: Option<User>| { html! {
                                if let Some(user) = user {
                                    <Link<Route> classes={classes!("title", "is-block", "col-6", "text-decoration-none")} to={Route::Author { id: user.id }}>
                                        { &user.username }
                                    </Link<Route>>
                                } else {
                                    <div class="col text-end">
                                        <span class="placeholder col-4"></span>
                                    </div>
                                }
                            } } } />
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
