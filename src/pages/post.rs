use yew::prelude::*;

use crate::components::item::*;
use crate::components::list::*;
use crate::components::post_card::*;
use crate::components::comment_card::*;
use crate::content;

use crate::Route;

#[derive(PartialEq, Properties)]
pub struct PostProps {
    pub post_id: u64,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    html! {
        <>
            <Item<content::Post> 
                item_id={ props.post_id } 
                component={ |post| html! { <PostCard { post } 
                fetch_author={ true } /> } } 
            />
            <List<content::CommentsContainer, content::CommentsContainerPostIdParam> 
                params={ content::CommentsContainerPostIdParam { post_id: props.post_id } } 
                items_per_page={ 100 } 
                route_to_page={ Route::Post { id: props.post_id } } 
                component={ |comment| html! { <CommentCard { comment } /> } } 
            />
        </>
    }
}