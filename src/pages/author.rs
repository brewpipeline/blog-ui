use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content;

use crate::Route;

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
            component={ |author: Option<content::Author>| {
                html! {
                    <>
                        if let Some(author) = &author {
                            <Meta title={ format!("{} - Автор", author.slug.clone()) } />
                        } else {
                            <Meta title="Автор" />
                        }
                        <AuthorCard author={ author.clone() } link_to=false />
                        if let Some(author) = &author {
                            <List<content::API<content::PostsContainer>, content::PostsContainerAuthorParam>
                                params={ content::PostsContainerAuthorParam { author_id: author.id } }
                                route_to_page={ Route::Author { slug: author.slug.clone() } }
                                component={ |post| html! { <PostCard { post } is_full=false link_to=true /> } }
                                error_component={ |_| html! { <Warning text="Ошибка загрузки публикаций автора!" /> } }
                            >
                                <Warning text="У автора нет публикаций." />
                            </List<content::API<content::PostsContainer>, content::PostsContainerAuthorParam>>
                        }
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
