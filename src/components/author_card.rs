use yew::prelude::*;
use yew_router::prelude::*;

use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct AuthorCardProps {
    pub author: Option<Author>,
    pub link_to: bool,
}

#[function_component(AuthorCard)]
pub fn author_card(props: &AuthorCardProps) -> Html {
    let AuthorCardProps { author, link_to } = props.clone();

    let main_content = html! {
        <div class="row g-0">
            <div class="col-4">
                <div
                    style={
                        format!(
                            "height:220px;width:100%;--image-url:url({});",
                            author
                                .as_ref()
                                .map(|a| a.image_url())
                                .unwrap_or_default()
                        )
                    }
                    class="img-block img-fluid rounded-start" role="img"
                />
            </div>
            <div class="col">
                <div class="card-body">
                    <h5 class="card-title placeholder-glow">
                        if let Some(author) = author.as_ref() {
                            {
                                format!(
                                    "{} {}",
                                    not_empty(author.first_name.clone())
                                        .unwrap_or("(Имя не указано)".to_owned()),
                                    not_empty(author.last_name.clone())
                                        .unwrap_or("(Фамилия не указанa)".to_owned())
                                )
                            }
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span> { " " }
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </h5>
                    <p class="card-text placeholder-glow">
                        <small class="text-body-secondary">
                            if let Some(slug) = author.as_ref().map(|a| a.slug.clone()) {
                                { slug }
                            } else {
                                <span class="placeholder col-2 bg-secondary"></span>
                            }
                        </small>
                    </p>
                    <p class="card-text placeholder-glow">
                        if let Some(author) = author.as_ref() {
                            {
                                not_empty(author.email.clone())
                                    .unwrap_or("(e-mail не указан)".to_owned())
                            }
                        } else {
                            <span class="placeholder col-4 bg-secondary"></span>
                        }
                    </p>
                </div>
            </div>
        </div>
    };
    html! {
        <div class="card mb-3">
            if let (Some(author), true) = (author.as_ref(), link_to) {
                <Link<Route, (), Author>
                    classes="text-decoration-none"
                    to={ Route::Author { slug: author.slug.clone() } }
                    state={ Some(author.clone()) }
                >
                    { main_content }
                </Link<Route, (), Author>>
            } else {
                { main_content }
            }
        </div>
    }
}
