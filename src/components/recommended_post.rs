use yew::prelude::*;

use crate::components::item::*;
use crate::components::recommended_post_card::*;
use crate::components::simple_title_card::*;
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
        <>
            <SimpleTitleCard>
                { "Рекомендация" }
            </SimpleTitleCard>
            <Item<content::API<content::PostContainer>, content::PostRecommendationParams>
                r#type={ LoadType::Params(content::PostRecommendationParams { id }) }
                use_caches=false
                component={ |post: Option<content::Post>| {
                    post
                        .map(|post| html! { <RecommendedPostCard post={ Some(post) } /> })
                        .unwrap_or_default()
                } }
                error_component={ |_| Html::default() }
            />
        </>
    }
}
