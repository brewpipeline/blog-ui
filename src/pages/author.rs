use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::item::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::simple_title_card::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::*;

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
            r#type={ LoadType::Params(content::AuthorSlugParams { slug: slug.clone() }) }
            use_caches=true
            component={ |author: Option<content::Author>| html! {
                <>
                    if let Some(author) = &author {
                        <Meta
                            r#type="profile"
                            title={ format!("{} - Автор", author_slug_formatter(&author)) }
                            description={ author.status.clone().unwrap_or_default() }
                            keywords=""
                            image={ author.image_url.clone().unwrap_or_default() }
                        />
                    } else {
                        <Meta title="Автор" noindex=true />
                    }
                    <AuthorCard author={ author.clone() } link_to=false />
                    if let Some(author) = &author {
                        <SimpleTitleCard>
                            { "Публикации автора " }
                        </SimpleTitleCard>
                        <List<content::API<content::PostsContainer>, content::OptionTokened<content::PostsContainerParams>>
                            r#type={ LoadType::Params(content::OptionTokened {
                                token: None,
                                params: content::PostsContainerParams {
                                    publish_type: content::PublishType::Published,
                                    search_query: None,
                                    author_id: Some(author.id),
                                    tag_id: None
                                }
                            }) }
                            route_to_page={ Route::Author { slug: author.slug.clone() } }
                            component={ |post| html! { <PostCard { post } is_full=false /> } }
                            error_component={ |_| html! { <Warning text="Ошибка загрузки публикаций автора!" /> } }
                        >
                            <Warning text="У автора нет публикаций." />
                        </List<content::API<content::PostsContainer>, content::OptionTokened<content::PostsContainerParams>>>
                    }
                </>
            } }
            error_component={ |_| html! {
                <>
                    <Meta title="Ошибка загрузки автора" noindex=true />
                    <Warning text="Ошибка загрузки автора!" />
                </>
            } }
        />
    }
}
