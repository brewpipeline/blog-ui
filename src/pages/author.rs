use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::html_document;

#[derive(PartialEq, Properties, Clone)]
pub struct AuthorProps {
    pub slug: String,
}

#[function_component(Author)]
pub fn author(props: &AuthorProps) -> Html {
    let AuthorProps { slug } = props.clone();
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Автор".to_string());
    html! {
        <Item<content::API<content::AuthorContainer>, content::AuthorSlugParams>
            params={ content::AuthorSlugParams { slug: slug.clone() } }
            component={ |author: Option<content::Author>| {
                if let Some(author) = &author {
                    html_document::reset_title_and_meta();
                    html_document::set_prefix_default_title(format!("{} - Автор", author.base.slug.clone()));
                }
                html! { <AuthorCard { author } link_to=false /> }
            } }
            error_component={ |_| html! { <Warning text="Ошибка загрузки автора" /> } }
        />
    }
}
