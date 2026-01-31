use yew::prelude::*;

use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::simple_title_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::lang;
use crate::utils::*;

use crate::Route;

#[function_component(MyUnpublishedPosts)]
pub fn my_unpublished_posts() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let meta = html! {
        <Meta title={ lang::MY_UNPUB_TITLE } noindex=true />
    };

    let not_auth_content = html! {
        <>
            { meta.clone() }
            <Warning text={ lang::MY_UNPUB_AUTH_REQUIRED } />
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
                { Html::from(lang::MY_UNPUB_TITLE) }
            </SimpleTitleCard>
            <List<API<PostsContainer>, OptionTokened<PostsContainerParams>>
                r#type={ LoadType::Params(OptionTokened {
                    token: Some(token),
                    params: PostsContainerParams {
                        publish_type: PublishType::Unpublished,
                        search_query: None,
                        author_id: Some(author.id),
                        tag_id: None
                    }
                }) }
                route_to_page={ Route::UnpublishedPosts }
                component={ |post| html! { <PostCard { post } is_full=false /> } }
                error_component={ |_| html! { <Warning text={ lang::MY_UNPUB_ERROR } /> } }
            >
                <Warning text={ lang::MY_UNPUB_EMPTY } />
            </List<API<PostsContainer>, OptionTokened<PostsContainerParams>>>
        </>
    }
}
