use yew::prelude::*;
use gloo_net::http::Request;

use crate::components::author_card;
use crate::content;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

#[function_component(Author)]
pub fn author(props: &Props) -> Html {
    let user_id = props.id;
    let user = use_state(|| None);
    {
        let user = user.clone();
        use_effect_with((), move |_| {
            let user = user.clone();
            user.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let user_url = content::User::url(user_id);
                let fetched_user: content::User = Request::get(user_url.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                user.set(Some(fetched_user));
            });
            || ()
        });
    }

    if let Some(user) = (*user).clone() {
        html! {
            <author_card::AuthorCard content={ Some(author_card::AuthorCardContent { 
                user,
            }) } />
        }
    } else {
        html! {
            <author_card::AuthorCard content={ None } />
        }
    }
}