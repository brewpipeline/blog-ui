use yew::prelude::*;

use crate::components::item::*;
use crate::components::post_card::*;
use crate::content::*;
use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct PostRecommendationProps {
    pub id: u64,
}

#[function_component(PostRecommendation)]
pub fn post_recommendation(props: &PostRecommendationProps) -> Html {
    let PostRecommendationProps { id } = props.clone();
    let logged_user_context =
        use_context::<LoggedUserContext>().expect("no logged user context found");
    let is_editor = logged_user_context
        .author()
        .map(|a| a.editor == 1)
        .unwrap_or(false);

    let is_recommended = use_state_eq(|| false);
    {
        let is_recommended = is_recommended.clone();
        let token = logged_user_context.token().cloned();
        use_effect_with((id, is_editor), move |(id, is_editor)| {
            if !*is_editor {
                return;
            }
            let token = token.clone();
            let id = *id;
            wasm_bindgen_futures::spawn_local(async move {
                let result = API::<PostWithRecommendedContainer>::get(OptionTokened {
                    token,
                    params: PostParams { id },
                })
                .await;
                if let Ok(API::Success {
                    data: PostWithRecommendedContainer { post },
                    ..
                }) = result
                {
                    is_recommended.set(post.recommended == 1);
                }
            });
        });
    }

    let in_progress = use_state_eq(|| false);

    let action_button = if is_editor {
        let onclick = {
            let in_progress = in_progress.clone();
            let is_recommended = is_recommended.clone();
            let logged_user_context = logged_user_context.clone();
            let id = id;
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                if *in_progress {
                    return;
                }
                let Some(token) = logged_user_context.token().cloned() else {
                    return;
                };
                in_progress.set(true);
                let recommend = !*is_recommended;
                let is_recommended = is_recommended.clone();
                let in_progress = in_progress.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let res = API::<()>::get(Tokened {
                        token,
                        params: PostPoolParams { id, add: recommend },
                    })
                    .await;
                    if let Ok(API::Success { .. }) = res {
                        is_recommended.set(recommend);
                    }
                    in_progress.set(false);
                });
            })
        };
        html! {
            <div class="d-grid mt-2">
                <button type="button" class="btn btn-light" {onclick} disabled={*in_progress}>
                    if *is_recommended {
                        { "Удалить из рекомендаций" }
                    } else {
                        { "Добавить в рекомендации" }
                    }
                </button>
            </div>
        }
    } else {
        html! { <></> }
    };

    html! {
        <>
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
            { action_button }
        </>
    }
}
