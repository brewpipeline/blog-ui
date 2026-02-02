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

#[function_component(HiddenPosts)]
pub fn hidden_posts() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let meta = html! {
        <Meta title={ lang::HIDDEN_TITLE } noindex=true />
    };

    let not_auth_content = html! {
        <>
            { meta.clone() }
            <Warning text={ lang::HIDDEN_AUTH_REQUIRED } />
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
                <Warning text={ lang::HIDDEN_EDITORS_ONLY } />
            </>
        };
    }

    html! {
        <>
            { meta }
            <SimpleTitleCard>
                { Html::from(lang::HIDDEN_TITLE) }
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
                error_component={ |_| html! { <Warning text={ lang::HIDDEN_ERROR } /> } }
            >
                <Warning text={ lang::HIDDEN_EMPTY } />
            </List<API<PostsContainer>, OptionTokened<PostsContainerParams>>>
        </>
    }
}
