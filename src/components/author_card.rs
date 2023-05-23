use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::User;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct AuthorCardProps {
    pub user: Option<User>,
}

pub struct AuthorCard;

impl Component for AuthorCard {
    type Message = ();
    type Properties = AuthorCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Some(user) = &ctx.props().user else {
            return html! {
                <div class="card mb-3">
                    <div class="row g-0">
                        <div class="col-4">
                            <img style="height:250px;width:100%;" class="img-fluid rounded-start" role="img" />
                        </div>
                        <div class="col">
                            <div class="card-body">
                                <h5 class="card-title placeholder-glow">
                                    <span class="placeholder col-3"></span> { " " }
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
                            <img src={ user.image_url.clone() } style="height:250px;width:100%;" class="img-fluid rounded-start" role="img" />
                        </div>
                        <div class="col">
                            <div class="card-body">
                                <h5 class="card-title">{ format!("{} {}", user.first_name, user.last_name) }</h5>
                                <p class="card-text"><small class="text-body-secondary">{ &user.username }</small></p>
                                <p class="card-text">{ &user.email }</p>
                            </div>
                        </div>
                    </div>
                </Link<Route>>
            </div>
        }
    }
}
