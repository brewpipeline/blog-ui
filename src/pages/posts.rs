use yew::prelude::*;

use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[function_component(Posts)]
pub fn posts() -> Html {
    html! {
        <>
            <Meta title="Публикации" />
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
                component={ |post| html! { <PostCard { post } is_full=false /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки публикаций!" /> } }
            >
                <Warning text="Нет публикаций." />
            </List<API<PostsContainer>, OptionTokened<PostsContainerParams>>>
        </>
    }
}
