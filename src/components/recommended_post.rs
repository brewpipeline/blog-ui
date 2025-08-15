use yew::prelude::*;

use crate::components::item::*;
use crate::components::post_card::*;
use crate::content;
use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct RecommendedPostProps {
    pub id: u64,
}

#[function_component(RecommendedPost)]
pub fn recommended_post(props: &RecommendedPostProps) -> Html {
    let RecommendedPostProps { id } = *props;
    html! {
        <Item<content::API<content::PostContainer>, content::PostRecommendationParams>
            r#type={ LoadType::Params(content::PostRecommendationParams { id }) }
            use_caches=false
            component={ |post: Option<content::Post>| {
                if let Some(post) = post {
                    html! { <PostCard post={ Some(post) } is_full=false /> }
                } else {
                    html! {}
                }
            } }
            error_component={ |_| html! {} }
        />
    }
}
