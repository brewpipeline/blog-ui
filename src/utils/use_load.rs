use yew::prelude::*;
use yew_router::prelude::*;

use crate::utils::*;

#[derive(Clone, PartialEq)]
pub enum LoadError<E: Clone + PartialEq> {
    Net(String),
    Content(E),
}

#[hook]
pub fn use_load_and_map<C, P, F, I>(
    params: P,
    inner_map: F,
    use_caches: bool,
) -> UseStateHandle<Option<Result<I, LoadError<C::Error>>>>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
    F: FnOnce(C::Inner) -> I + 'static,
    I: ExternalCodable + Clone + PartialEq + 'static,
{
    let location = use_location().unwrap();
    let app_content_container = use_context::<AppContentContext>().unwrap();
    let container_inner_result = {
        let app_content_context = app_content_container.clone();
        use_state_eq(|| {
            if let (true, Some(page_container_inner), false) = (
                use_caches,
                (*app_content_container)
                    .app_content
                    .clone()
                    .map(|a| I::decode(a))
                    .flatten(),
                (*app_content_container).is_used,
            ) {
                Some(Ok(page_container_inner))
            } else {
                None
            }
        })
    };
    #[cfg(feature = "client")]
    {
        let container_inner_result = container_inner_result.clone();
        use_effect_with(params, move |params| {
            let is_app_content_used = app_content_container.is_used;
            app_content_container.dispatch(AppContentContainerAction::MarkAsUsed);
            if use_caches && !is_app_content_used && *container_inner_result != None {
                return;
            }
            if let (true, Some(route_container_inner)) =
                (use_caches, location.state::<I>().map(|i| (*i).clone()))
            {
                app_content_container.dispatch(AppContentContainerAction::NewContent(
                    route_container_inner.encode(),
                ));
                container_inner_result.set(Some(Ok(route_container_inner)));
            } else {
                if use_caches {
                    app_content_container.dispatch(AppContentContainerAction::NewContent(None));
                }
                container_inner_result.set(None);
                let app_content_container = app_content_container.clone();
                let params = params.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_container_inner_result = C::get(params)
                        .await
                        .map_err(|err| LoadError::Net(err.to_string()))
                        .and_then(|r| r.result().map(inner_map).map_err(|e| LoadError::Content(e)));
                    if use_caches {
                        app_content_container.dispatch(AppContentContainerAction::NewContent(
                            fetched_container_inner_result
                                .as_ref()
                                .map(|r| r.encode())
                                .ok()
                                .flatten(),
                        ));
                    }
                    container_inner_result.set(Some(fetched_container_inner_result));
                });
            }
        });
    }
    container_inner_result
}

#[hook]
pub fn use_load<C, P>(
    params: P,
    use_route_cache: bool,
) -> UseStateHandle<Option<Result<C::Inner, LoadError<C::Error>>>>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: ExternalCodable + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    use_load_and_map::<C, P, _, C::Inner>(params, |i| i, use_route_cache)
}
