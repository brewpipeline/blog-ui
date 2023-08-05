use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct AuthorProps {
    pub slug: String,
}

#[function_component(Author)]
pub fn author(props: &AuthorProps) -> Html {
    let AuthorProps { slug } = props.clone();
    let app_meta = use_context::<AppMetaContext>().unwrap();
    html! {
        <Item<content::API<content::AuthorContainer>, content::AuthorSlugParams>
            params={ content::AuthorSlugParams { slug: slug.clone() } }
            use_caches=true
            component={ move |author: Option<content::Author>| {
                app_meta.dispatch([AppMetaAction::Title("Автор".to_string())].into());
                if let Some(author) = &author {
                    app_meta.dispatch([AppMetaAction::Title(format!("{} - Автор", author.base.slug.clone()))].into());
                }
                html! { <AuthorCard { author } link_to=false /> }
            } }
            error_component={ |_| html! { <Warning text="Ошибка загрузки автора" /> } }
        />
    }
}
