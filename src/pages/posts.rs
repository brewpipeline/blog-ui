use yew::prelude::*;

use crate::components::list::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::head;

use crate::Route;

#[function_component(Posts)]
pub fn posts() -> Html {
    head::reset_title_and_meta();
    head::set_prefix_default_title("Публикации".to_string());
    html! {
        <List<API<PostsContainer>>
            params={ () }
            route_to_page={ Route::Posts }
            component={ |post| html! { <PostCard { post } is_full=false link_to=true /> } }
            error_component={ |_| html! { <Warning text="Ошибка загрузки публикаций" /> } }
        >
            <Warning text="Нет публикаций" />
        </List<API<PostsContainer>>>
    }
}
