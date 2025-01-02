use yew::prelude::*;

use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::simple_title_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[function_component(HiddenPosts)]
pub fn hidden_posts() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();
    html! {
        <>
            <Meta title="Скрытое" noindex=true />
            <SimpleTitleCard>
                { "Скрытое" }
            </SimpleTitleCard>
            <List<API<PostsContainer>, OptionTokened<HiddenPostsContainerParams>>
                r#type={
                    if !logged_user_context.is_not_inited() {
                        LoadType::Params(OptionTokened {
                            token: logged_user_context.token().cloned(),
                            params: HiddenPostsContainerParams
                        })
                    } else {
                        LoadType::OnlyAppCacheIfApplicable
                    }
                }
                route_to_page={ Route::UnpublishedPosts }
                component={ |post| html! { <PostCard { post } is_full=false /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки скрытого!" /> } }
            >
                <Warning text="Нет скрытого." />
            </List<API<PostsContainer>, OptionTokened<HiddenPostsContainerParams>>>
        </>
    }
}
