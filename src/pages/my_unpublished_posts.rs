use yew::prelude::*;

use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::simple_title_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[function_component(MyUnpublishedPosts)]
pub fn my_unpublished_posts() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let meta = html! {
        <Meta title="Мое неопубликованное" />
    };

    let not_auth_content = html! {
        <>
            { meta.clone() }
            <Warning text="Нужна авторизация для получения моего неопубликованного!" />
        </>
    };

    if logged_user_context.is_not_inited() {
        return not_auth_content;
    }

    let LoggedUserState::ActiveAndLoaded { token, author } = logged_user_context.state().clone()
    else {
        return not_auth_content;
    };
    html! {
        <>
            { meta }
            <SimpleTitleCard>
                { "Мое неопубликованное" }
            </SimpleTitleCard>
            <List<API<PostsContainer>, OptionTokened<UnpublishedPostsContainerAuthorParams>>
                r#type={
                    if !logged_user_context.is_not_inited() {
                        LoadType::Params(OptionTokened {
                            token: Some(token),
                            params: UnpublishedPostsContainerAuthorParams { author_id: author.id }
                        })
                    } else {
                        LoadType::OnlyAppCacheIfApplicable
                    }
                }
                route_to_page={ Route::UnpublishedPosts }
                component={ |post| html! { <PostCard { post } is_full=false /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки моего неопубликованного!" /> } }
            >
                <Warning text="Нет моего неопубликованного." />
            </List<API<PostsContainer>, OptionTokened<UnpublishedPostsContainerAuthorParams>>>
        </>
    }
}
