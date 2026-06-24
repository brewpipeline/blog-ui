use yew::prelude::*;

use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::lang;
use crate::utils::*;

use crate::Route;

#[function_component(Posts)]
pub fn posts() -> Html {
    html! {
        <Meta title={ lang::POSTS_TITLE } />
        <List<API<PostsContainer>, OptionTokened<PostsContainerParams>>
            r#type={ LoadType::Params(OptionTokened {
                token: None,
                params: PostsContainerParams {
                    publish_type: PublishType::Published,
                    search_query: None,
                    author_id: None,
                    tag_id: None
                }
            }) }
            use_caches=true
            route_to_page={ Route::Posts }
            component={ |(i, post)| html! { <PostCard { post } is_full=false priority={ i < 2 } /> } }
            error_component={ |_| html! {
                <Meta title={ lang::POSTS_ERROR_TITLE } noindex=true />
                <Warning text={ lang::POSTS_ERROR_TEXT } />
            } }
        >
            <Meta title={ lang::POSTS_EMPTY_TITLE } noindex=true />
            <Warning text={ lang::POSTS_EMPTY_TEXT } />
        </List<API<PostsContainer>, OptionTokened<PostsContainerParams>>>
    }
}
