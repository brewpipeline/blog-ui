use yew::prelude::*;
use gloo_net::http::Request;

use crate::components::post_card;
use crate::content;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let post_id = props.id;
    let post = use_state_eq(|| None);
    {
        let post = post.clone();
        use_effect_with((), move |_| {
            let post = post.clone();
            post.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let post_url = content::Post::url(post_id);
                let fetched_post: content::Post = Request::get(post_url.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                post.set(Some(fetched_post));
            });
            || ()
        });
    }

    html! {
        <post_card::PostCard post={ (*post).clone() } />
    }
}