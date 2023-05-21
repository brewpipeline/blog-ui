use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;

use crate::components::pagination::*;
use crate::components::post_card::*;
use crate::content::PostsContainer;

use crate::Route;

const ITEMS_PER_PAGE: u64 = 10;

#[function_component(PostList)]
pub fn post_list() -> Html {
    let page = use_location()
        .unwrap()
        .query::<PageQuery>()
        .map(|it| it.page)
        .unwrap_or(1);
    
    let posts_container = use_state_eq(|| None);
    {
        let posts_container = posts_container.clone();
        use_effect_with(page, move |_| {
            let posts_container = posts_container.clone();
            posts_container.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let limit = ITEMS_PER_PAGE;
                let skip = (page - 1) * ITEMS_PER_PAGE;
                let posts_url = PostsContainer::url(limit, skip);
                let fetched_posts_container: PostsContainer = Request::get(posts_url.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                posts_container.set(Some(fetched_posts_container));
            });
            || ()
        });
    }

    let Some(posts_container) = (*posts_container).clone() else {
        return (0..ITEMS_PER_PAGE).map(|_| {
            html! {
                <PostCard post={ None } />
            }
        }).collect::<Html>()
    };
    html! {
        <>
            {
                posts_container.posts.into_iter().map(|post| {
                    html! {
                        <PostCard post={ Some(post) } />
                    }
                }).collect::<Html>()
            }
            <Pagination
                {page}
                total_pages={(posts_container.total as f64 / ITEMS_PER_PAGE as f64).ceil() as u64}
                route_to_page={Route::Posts}
            />
        </>
    }
}