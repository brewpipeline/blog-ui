use yew::prelude::*;

use crate::components::item::*;
use crate::components::post_card::*;
use crate::content::*;
use crate::utils::LoadType;

#[derive(PartialEq, Properties, Clone)]
pub struct PostRecommendationProps {
    pub id: u64,
}

#[function_component(PostRecommendation)]
pub fn post_recommendation(props: &PostRecommendationProps) -> Html {
    let PostRecommendationProps { id } = props.clone();
    html! {
        <Item<API<PostRecommendationContainer>, PostRecommendationParams>
            r#type={ LoadType::Params(PostRecommendationParams { id }) }
            use_caches=true
            component={ |post: Option<Post>| {
                if let Some(post) = post {
                    html! { <PostCard post={ Some(post) } is_full=false /> }
                } else {
                    html! { <></> }
                }
            } }
            error_component={ |_| html! { <></> } }
        />
    }
}

