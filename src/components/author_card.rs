use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::User;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct AuthorCardProps {
    pub user: Option<User>,
}

#[function_component(AuthorCard)]
pub fn author_card(props: &AuthorCardProps) -> Html {
    let AuthorCardProps { user } = props.clone();

    let route = use_route::<Route>().unwrap_or_default();

    let Some(user) = user else {
        return html! {
            <div class="card mb-3">
                <div class="row g-0">
                    <div class="col-4">
                        <div style="height:220px;width:100%;" class="img-block img-fluid rounded-start" role="img" />
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
    let main_content = html! {
        <div class="row g-0">
            <div class="col-4">
                <div style={ format!("height:220px;width:100%;--image-url:url({});", user.image_url.clone()) } class="img-block img-fluid rounded-start" role="img" />
            </div>
            <div class="col">
                <div class="card-body">
                    <h5 class="card-title">{ format!("{} {}", user.first_name, user.last_name) }</h5>
                    <p class="card-text"><small class="text-body-secondary">{ &user.username }</small></p>
                    <p class="card-text">{ &user.email }</p>
                </div>
            </div>
        </div>
    };
    let card_route = Route::Author { id: user.id };
    html! {
        <div class="card mb-3">
            if route != card_route {
                <Link<Route> classes={classes!("text-decoration-none")} to={ card_route }>
                    { main_content }
                </Link<Route>>
            } else {
                { main_content }
            }
        </div>
    }
}
