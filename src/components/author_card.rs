use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::User;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct AuthorCardProps {
    pub user: Option<User>,
    pub link_to: bool,
}

#[function_component(AuthorCard)]
pub fn author_card(props: &AuthorCardProps) -> Html {
    let AuthorCardProps { user, link_to } = props.clone();

    let main_content = html! {
        <div class="row g-0">
            <div class="col-4">
                <div style={ format!("height:220px;width:100%;--image-url:url({});", user.as_ref().map(|p| p.image_url.clone()).unwrap_or_default()) } class="img-block img-fluid rounded-start" role="img" />
            </div>
            <div class="col">
                <div class="card-body">
                    <h5 class="card-title placeholder-glow">
                        if let Some(user) = user.as_ref() {
                            { format!("{} {}", user.first_name, user.last_name) }
                        } else {
                            <span class="placeholder col-3"></span> { " " }
                            <span class="placeholder col-3"></span>
                        }
                    </h5>
                    <p class="card-text placeholder-glow">
                        <small class="text-body-secondary">
                            if let Some(username) = user.as_ref().map(|u| u.username.clone()) {
                                { username }
                            } else {
                                <span class="placeholder col-2"></span>
                            }
                        </small>
                    </p>
                    <p class="card-text placeholder-glow">
                        if let Some(email) = user.as_ref().map(|u| u.email.clone()) {
                            { email }
                        } else {
                            <span class="placeholder col-4"></span>
                        }
                    </p>
                </div>
            </div>
        </div>
    };
    html! {
        <div class="card mb-3">
            if let (Some(user), true) = (user.as_ref(), link_to) {
                <Link<Route> classes="text-decoration-none" to={Route::Author { id: user.id }}>
                    { main_content }
                </Link<Route>>
            } else {
                { main_content }
            }
        </div>
    }
}
