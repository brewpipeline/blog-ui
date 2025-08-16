use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::item::*;
use crate::components::recommended_post_card::*;
use crate::components::simple_title_card::*;
use crate::content;

use crate::utils::*;
use crate::Route;

#[function_component(RecommendedPost)]
pub fn recommended_post() -> Html {
    let route = use_route::<Route>().unwrap_or_default();

    match route {
        Route::Post { id, .. } => {
            html! {
                <Item<content::API<content::PostContainer>, content::PostRecommendationParams>
                    r#type={ LoadType::Params(content::PostRecommendationParams { id }) }
                    use_caches=false
                    component={ |post: Option<content::Post>| {
                        post
                            .map(|post| html! {
                                <>
                                    <SimpleTitleCard>
                                        { "Вам будет интересно" }
                                    </SimpleTitleCard>
                                    <RecommendedPostCard { post } />
                                </>
                            })
                            .unwrap_or_default()
                    } }
                    error_component={ |_| Html::default() }
                />
            }
        }
        _ => html! {},
    }
}
