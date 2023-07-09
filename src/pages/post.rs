use yew::prelude::*;

use crate::components::comment_card::*;
use crate::components::item::*;
use crate::components::list::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::html_document;

use crate::Route;

#[derive(PartialEq, Properties, Clone)]
pub struct PostProps {
    pub slug: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let PostProps { slug } = props.clone();
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Публикация".to_string());
    html! {
        <>
            <Item<content::API<content::PostContainer>, content::PostSlugParam>
                params={ content::PostSlugParam { slug: slug.clone() } }
                component={ |post: Option<content::Post>| {
                    if let Some(post) = &post {
                        html_document::reset_title_and_meta();
                        html_document::set_prefix_default_title(format!("{} - Публикация", post.title.clone()));
                        html_document::set_meta(html_document::MetaTag::Description, post.summary.clone());
                        html_document::set_meta(html_document::MetaTag::Keywords, post.tags.clone().into_iter().map(|v| v.title).collect::<Vec<String>>().join(", "));
                    }
                    html! { <PostCard { post } link_to=false /> }
                } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки публикации" /> } }
            />
            <List<content::CommentsContainer, content::CommentsContainerPostIdParam>
                params={ content::CommentsContainerPostIdParam { post_id: 1 /* TODO */ } }
                items_per_page={ 100 }
                route_to_page={ Route::Post { slug: slug.clone() } }
                component={ |comment| html! { <CommentCard { comment } /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки комментариев" /> } }
            >
                <Warning text="Нет комментариев" />
            </List<content::CommentsContainer, content::CommentsContainerPostIdParam>>
        </>
    }
}
