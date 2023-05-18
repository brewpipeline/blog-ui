use yew::prelude::*;
use gloo_net::http::Request;

use crate::components::post_card;
use crate::content;
use crate::generator::Generator;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u64,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let post_id = props.id;
    let post = use_state(|| None);
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
        {
            if let Some(post) = (*post).clone() {
                let image_url = Generator::from_seed(post.id).image_url((400, 100), post.tags.as_slice());
                html! {
                    <post_card::PostCard content={ Some(post_card::PostCardContent { 
                        post, 
                        image_url,
                    }) } />
                }
            } else {
                html! {
                    <post_card::PostCard content={ None } />
                }
            }
        }
    }
}