use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
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
        <Item<content::API<content::AuthorContainer>, content::AuthorSlugParam>
            params={ content::AuthorSlugParam { slug } }
            component={ |api_author: Option<content::API<content::AuthorContainer>>| {
                /* TODO: think about remove API here */
                let author = api_author.map(|a| a.data()).flatten().map(|d| d.author);
                if let Some(author) = &author {
                    html_document::reset_title_and_meta();
                    html_document::set_prefix_default_title(format!("{} - Автор", author.slug.clone()));
                }
                html! { <AuthorCard { author } link_to=false /> }
            } }
        />
    }
}
