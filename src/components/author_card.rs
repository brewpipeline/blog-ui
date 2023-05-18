use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::User;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthorCardContent {
    pub user: User,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub content: Option<AuthorCardContent>,
}

pub struct AuthorCard;

impl Component for AuthorCard {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Some(AuthorCardContent { user }) = &ctx.props().content else {
            return html! {
                <div class="card mb-3">
                    <div class="row g-0">
                        <div class="col-4">
                            <img style="background-color:#868e96;height:250px;width:100%;" class="img-fluid rounded-start" role="img" />
                        </div>
                        <div class="col">
                            <div class="card-body">
                                <h5 class="card-title placeholder-glow">
                                    <span class="placeholder col-3"></span>
                                    <span class="placeholder col-3"></span>
                                </h5>
                                <p class="card-text placeholder-glow"><small class="text-body-secondary"><span class="placeholder col-2"></span></small></p>
                                <p class="card-text placeholder-glow"><span class="placeholder col-3"></span></p>
                            </div>
                        </div>
                    </div>
                </div>
            }
        };
        html! {
            <div class="card mb-3">
                <Link<Route> classes={classes!("text-decoration-none")} to={Route::Author { id: user.id }}>
                    <div class="row g-0">
                        <div class="col-4">
                            <img src={ user.image_url.clone() } style="background-color:#868e96;height:250px;width:100%;" class="img-fluid rounded-start" role="img" />
                        </div>
                        <div class="col">
                            <div class="card-body">
                                <h5 class="card-title">{ format!("{} {}", user.first_name, user.last_name) }</h5>
                                <p class="card-text"><small class="text-body-secondary">{ &user.gender }</small></p>
                                <p class="card-text">{ &user.email }</p>
                            </div>
                        </div>
                    </div>
                </Link<Route>>
            </div>
        }
    }
}
