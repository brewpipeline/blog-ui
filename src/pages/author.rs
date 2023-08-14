use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
use crate::components::meta::*;
use crate::components::warning::*;
use crate::content;

#[derive(PartialEq, Properties, Clone)]
pub struct AuthorProps {
    pub slug: String,
}

#[function_component(Author)]
pub fn author(props: &AuthorProps) -> Html {
    let AuthorProps { slug } = props.clone();
    html! {
        <Item<content::API<content::AuthorContainer>, content::AuthorSlugParams>
            params={ content::AuthorSlugParams { slug: slug.clone() } }
            use_caches=true
            component={ move |author: Option<content::Author>| {
                html! {
                    <>
                        if let Some(author) = &author {
                            <Meta title={ format!("{} - Автор", author.slug.clone()) } />
                        } else {
                            <Meta title="Автор" />
                        }
                        <AuthorCard { author } link_to=false />
                    </>
                }
            } }
            error_component={ |_| html! {
                <>
                    <Meta title="Ошибка загрузки автора" />
                    <Warning text="Ошибка загрузки автора!" />
                </>
            } }
        />
    }
}
