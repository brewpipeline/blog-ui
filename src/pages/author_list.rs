use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;

use crate::components::author_card::*;
use crate::components::pagination::*;
use crate::content::UsersContainer;

use crate::Route;

const ITEMS_PER_PAGE: u64 = 10;

#[function_component(AuthorList)]
pub fn author_list() -> Html {
    let page = use_location()
        .unwrap()
        .query::<PageQuery>()
        .map(|it| it.page)
        .unwrap_or(1);
    
    let users_container = use_state(|| None);
    {
        let users_container = users_container.clone();
        use_effect_with(page, move |_| {
            let users_container = users_container.clone();
            users_container.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let limit = ITEMS_PER_PAGE;
                let skip = (page - 1) * ITEMS_PER_PAGE;
                let users_url = UsersContainer::url(limit, skip);
                let fetched_users_container: UsersContainer = Request::get(users_url.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                users_container.set(Some(fetched_users_container));
            });
            || ()
        });
    }

    let Some(users_container) = (*users_container).clone() else {
        return (0..ITEMS_PER_PAGE).map(|_| {
            html! {
                <AuthorCard content={ None } />
            }
        }).collect::<Html>()
    };
    html! {
        <>
            {
                users_container.users.into_iter().map(|user| {
                    html! {
                        <AuthorCard content={ Some(AuthorCardContent { 
                            user, 
                        }) } />
                    }
                }).collect::<Html>()
            }
            <Pagination
                {page}
                total_pages={(users_container.total as f64 / ITEMS_PER_PAGE as f64).ceil() as u64}
                route_to_page={Route::Authors}
            />
        </>
    }
}