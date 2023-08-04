use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::head;

#[derive(PartialEq, Properties, Clone)]
pub struct AuthorProps {
    pub slug: String,
}

#[function_component(Author)]
pub fn author(props: &AuthorProps) -> Html {
    let AuthorProps { slug } = props.clone();
    head::reset_title_and_meta();
    head::set_prefix_default_title("Автор".to_string());
    html! {
        <Item<content::API<content::AuthorContainer>, content::AuthorSlugParams>
            params={ content::AuthorSlugParams { slug: slug.clone() } }
            use_route_cache=true
            component={ |author: Option<content::Author>| {
                if let Some(author) = &author {
                    head::reset_title_and_meta();
                    head::set_prefix_default_title(format!("{} - Автор", author.base.slug.clone()));
                }
                html! { <AuthorCard { author } link_to=false /> }
            } }
            error_component={ |_| html! { <Warning text="Ошибка загрузки автора" /> } }
        />
    }
}
