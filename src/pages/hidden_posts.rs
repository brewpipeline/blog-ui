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

    let meta = html! {
        <Meta title="Скрытое" noindex=true />
    };

    let not_auth_content = html! {
        <>
            { meta.clone() }
            <Warning text="Нужна авторизация для просмотра скрытого!" />
        </>
    };

    if logged_user_context.is_not_inited() {
        return not_auth_content;
    }

    let LoggedUserState::ActiveAndLoaded { token, author } = logged_user_context.state().clone()
    else {
        return not_auth_content;
    };

    if author.editor != 1 {
        return html! {
            <>
                { meta.clone() }
                <Warning text="Просмотр скрытого доступен только редакторам!" />
            </>
        };
    }

    html! {
        <>
            { meta }
            <SimpleTitleCard>
                { "Скрытое" }
            </SimpleTitleCard>
            <List<API<PostsContainer>, OptionTokened<PostsContainerParams>>
                r#type={ LoadType::Params(OptionTokened {
                    token: Some(token),
                    params: PostsContainerParams {
                        publish_type: PublishType::Hidden,
                        search_query: None,
                        author_id: None,
                        tag_id: None
                    }
                }) }
                route_to_page={ Route::UnpublishedPosts }
                component={ |post| html! { <PostCard { post } is_full=false /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки скрытого!" /> } }
            >
                <Warning text="Нет скрытого." />
            </List<API<PostsContainer>, OptionTokened<PostsContainerParams>>>
        </>
    }
}