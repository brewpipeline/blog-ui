use yew::prelude::*;

use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[function_component(UnpublishedPosts)]
pub fn unpublished_posts() -> Html {
    let meta = html! {
        <Meta title="Неопубликованное" />
    };

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let LoggedUserState::ActiveAndLoaded {
        token,
        author: Author { editor: 1, .. },
    } = logged_user_context.state.clone()
    else {
        return html! {
            <>
                { meta }
                <Warning text="Доступ к неопубликованному есть только у редакторов!" />
            </>
        };
    };

    html! {
        <>
            { meta }
            <List<API<PostsContainer>, Tokened<UnpublishedPostsContainerParams>>
                params={ Tokened::<UnpublishedPostsContainerParams> { token, params: UnpublishedPostsContainerParams } }
                route_to_page={ Route::UnpublishedPosts }
                component={ |post| html! { <PostCard { post } is_full=false /> } }
                error_component={ |_| html! { <Warning text="Ошибка загрузки неопубликованного!" /> } }
            >
                <Warning text="Нет неопубликованного." />
            </List<API<PostsContainer>, Tokened<UnpublishedPostsContainerParams>>>
        </>
    }
}
