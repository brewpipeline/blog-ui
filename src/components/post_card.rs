use yew::prelude::*;
use yew_router::components::Link;

use crate::content::Post;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub post: Option<Post>
}

pub struct PostCard;

impl Component for PostCard {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Some(post) = &ctx.props().post else {
            return html! {
                <div class="card mb-3">
                    <img style="background-color:#868e96;height:180px;" class="bd-placeholder-img card-img-top" role="img" />
                    <div class="card-body">
                        <h5 class="card-title placeholder-glow">
                            <span class="placeholder col-6"></span>
                        </h5>
                        <p class="card-text placeholder-glow">
                            <span class="placeholder col-7"></span>
                            <span class="placeholder col-4"></span>
                            <span class="placeholder col-4"></span>
                            <span class="placeholder col-6"></span>
                            <span class="placeholder col-8"></span>
                        </p>
                    </div>
                    <div class="card-footer">
                        <div class="row align-items-center">
                            <div class="col">
                            <span class="placeholder col-4"></span>
                            </div>
                            <div class="col text-end">
                                <span> { "Автор" } </span>
                            </div>
                        </div>
                    </div>
                </div>
            }
        };
        html! {
            <div class="card mb-3">
                <Link<Route> classes={classes!("text-decoration-none")} to={Route::Post { id: post.id }}>
                    <img src={ post.image_url() } style="background-color:#868e96;height:180px;" class="bd-placeholder-img card-img-top" role="img" />
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
                        <div class="col text-end">
                            <Link<Route> classes={classes!("title", "is-block", "col-6")} to={Route::Author { id: post.user_id }}>
                                { "Автор" }
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
