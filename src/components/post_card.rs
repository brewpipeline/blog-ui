use yew::prelude::*;
use yew_router::components::Link;

use crate::components::item::Item;
use crate::content::{Post, User};

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PostCardProps {
    pub post: Option<Post>,
    #[prop_or_default]
    pub fetch_author: bool,
}

pub struct PostCard;

impl Component for PostCard {
    type Message = ();
    type Properties = PostCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Some(post) = &ctx.props().post else {
            return html! {
                <div class="card mb-3">
                    <img style="height:180px;" class="bd-placeholder-img card-img-top" role="img" />
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
                            {
                                if ctx.props().fetch_author {
                                    html! {
                                        <div class="col text-end">
                                            <span class="placeholder col-4"></span>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                </div>
            }
        };
        html! {
            <div class="card mb-3">
                <Link<Route> classes={classes!("text-decoration-none")} to={Route::Post { id: post.id }}>
                    <img src={ post.image_url() } style="height:180px;" class="bd-placeholder-img card-img-top" role="img" />
                    <div class="card-body">
                        <h5 class="card-title">{ &post.title }</h5>
                        <p class="card-text">{ &post.body }</p>
                    </div>
                </Link<Route>>
                <div class="card-footer">
                    <div class="row align-items-center">
                        <div class="col">
                            { post.tags.join(", ") }
                        </div>
                        {
                            if ctx.props().fetch_author {
                                let post_author_id = post.user_id;
                                html! {
                                    <div class="col text-end">
                                        <Item<User> id={post_author_id} component={ move |user: Option<User>| {
                                            if let Some(user) = user {
                                                html! {
                                                    <Link<Route> classes={classes!("title", "is-block", "col-6")} to={Route::Author { id: user.id }}>
                                                        { &user.username }
                                                    </Link<Route>>
                                                }
                                            } else {
                                                html! {
                                                    <div class="col text-end">
                                                        <span class="placeholder col-4"></span>
                                                    </div>
                                                }
                                            }
                                        } } />
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }
}
