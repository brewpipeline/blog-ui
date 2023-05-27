use yew::prelude::*;

use crate::components::comment_card::*;
use crate::components::item::*;
use crate::components::list::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::html_document;

use crate::Route;

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub post_id: u64,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title("Публикация".to_string());
    html! {
        <>
            <Item<content::Post>
                item_id={ props.post_id }
                component={ |post: Option<content::Post>| {
                    if let Some(post) = &post {
                        html_document::reset_title_and_meta();
                        html_document::set_prefix_default_title(format!("{} - Публикация", post.title.clone()));
                        html_document::set_meta(html_document::MetaTag::Description, post.body.clone());
                        html_document::set_meta(html_document::MetaTag::Keywords, post.tags.join(", "));
                    }
                    html! { <PostCard { post } fetch_author={ true } /> }
                } }
            />
            <List<content::CommentsContainer, content::CommentsContainerPostIdParam>
                params={ content::CommentsContainerPostIdParam { post_id: props.post_id } }
                items_per_page={ 100 }
                route_to_page={ Route::Post { id: props.post_id } }
                component={ |comment| html! { <CommentCard { comment } /> } }
            >
                <Warning text="Нет комментариев" />
            </List<content::CommentsContainer, content::CommentsContainerPostIdParam>>
        </>
    }
}
