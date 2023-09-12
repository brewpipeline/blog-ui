use yew::prelude::*;

use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content::*;

use crate::Route;

#[function_component(Posts)]
pub fn posts() -> Html {
    html! {
        <>
            <Meta title="Публикации" />
            <List<API<PostsContainer>>
                params={ () }
                route_to_page={ Route::Posts }
                component={ |post| html! { <PostCard { post } is_full=false /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки публикаций!" /> } }
            >
                <Warning text="Нет публикаций." />
            </List<API<PostsContainer>>>
        </>
    }
}
