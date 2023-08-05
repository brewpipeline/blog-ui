use yew::prelude::*;

use crate::components::list::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[function_component(Posts)]
pub fn posts() -> Html {
    let app_meta = use_context::<AppMetaContext>().unwrap();
    app_meta.dispatch([AppMetaAction::Title("Публикации".to_string())].into());
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
