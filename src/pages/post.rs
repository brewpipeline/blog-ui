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
    pub id: u64,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let PostProps { slug, id } = props.clone();
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Публикация".to_string());
    html! {
        <Item<content::API<content::PostContainer>, content::PostIdParam>
            params={ content::PostIdParam { id } }
            component={ move |post: Option<content::Post>| {
                if let Some(post) = &post {
                    if post.id != id || post.slug != slug {
                        return html! { <Warning text="Ссылка на публикацию повреждена" /> }
                    }
                    html_document::reset_title_and_meta();
                    html_document::set_prefix_default_title(
                        format!("{} - Публикация", post.title.clone())
                    );
                    html_document::set_meta(
                        html_document::MetaTag::Description,
                        post.summary.clone()
                    );
                    html_document::set_meta(
                        html_document::MetaTag::Keywords,
                        post
                            .tags
                            .clone()
                            .into_iter()
                            .map(|v| v.title)
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                }
                html! {
                    <>
                        <PostCard post={ post.clone() } is_full=true link_to=false />
                        if let Some(post) = post {
                            <List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParam>
                                params={ content::CommentsContainerPostIdParam { post_id: post.id } }
                                items_per_page={ 100 }
                                route_to_page={ Route::Post { slug: post.slug, id: post.id } }
                                component={ |comment| html! { <CommentCard { comment } /> } }
                                error_component={ |_| html! { <Warning text="Ошибка загрузки комментариев" /> } }
                            >
                                <Warning text="Нет комментариев" />
                            </List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParam>>
                        }
                    </>
                }
            } }
            error_component={ |_| html! { <Warning text="Ошибка загрузки публикации" /> } }
        />
    }
}
