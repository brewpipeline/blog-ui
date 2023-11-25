use yew::prelude::*;

use crate::components::svg_image::*;

use crate::content::SubscribeAuthorIdParams;
use crate::content::Tokened;
use crate::content::API;
use crate::utils::*;

#[function_component(SubscribeButton)]
pub fn subscribe_button() -> Html {
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let author = logged_user_context.author().cloned();

    let in_progress = use_state_eq(|| false);
    let is_subscribed = use_state_eq(|| false);
    {
        let is_subscribed = is_subscribed.clone();
        use_effect_with(author.clone(), move |author| {
            is_subscribed.set(
                author
                    .as_ref()
                    .map(|a| a.notification_subscribed == Some(1))
                    .unwrap_or(false),
            );
        })
    }

    let author = author.clone();
    let logged_user_context = logged_user_context.clone();
    let in_progress = in_progress.clone();
    let is_subscribed = is_subscribed.clone();
    use_effect_with(in_progress, move |in_progress| {
        if logged_user_context.is_not_inited() || !**in_progress {
            return;
        }

        let (Some(author), Some(token)) = (author, (*logged_user_context).token().cloned()) else {
            return;
        };

        let in_progress = in_progress.clone();
        let is_subscribed = is_subscribed.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let subscribe_author_request = API::<()>::get(Tokened {
                token,
                params: SubscribeAuthorIdParams {
                    id: author.id,
                    subscribe: !*is_subscribed,
                },
            });
            match subscribe_author_request.await {
                Ok(subscribe_author_result) => match subscribe_author_result {
                    API::Success {
                        identifier: _,
                        description: _,
                        data: _,
                    } => {
                        is_subscribed.set(!*is_subscribed);
                        show_popover.set(true);
                    }
                    API::Failure {
                        identifier: _,
                        reason: _,
                    } => {}
                },
                Err(_) => {}
            }
            in_progress.set(false);
        });
    });

    let onclick = {
        let in_progress = in_progress.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();

            if *in_progress {
                return;
            }
            in_progress.set(true);
        })
    };

    if author.is_none() {
        html! {}
    } else {
        html! {
            <button
            type="button"
            class={ classes!("btn","btn-light", if author.is_none() { "invisible"} else {"visible"}) }
            disabled={ *in_progress }
            { onclick }
            >
            if *is_subscribed {
                <SubscribedImg />
            } else {
                <UnsubscribedImg />
            }
            </button>
        }
    }
}
