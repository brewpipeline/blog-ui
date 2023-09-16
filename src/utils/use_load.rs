use yew::prelude::*;
use yew_router::prelude::*;

use crate::utils::*;

#[derive(Clone, PartialEq)]
pub enum LoadType<P: Clone + PartialEq> {
    OnlyAppCacheIfApplicable,
    Params(P),
}

impl<P: Clone + PartialEq> LoadType<P> {
    pub fn map_params<NP: Clone + PartialEq, F: FnOnce(P) -> NP>(self, map_fn: F) -> LoadType<NP> {
        match self {
            LoadType::OnlyAppCacheIfApplicable => LoadType::OnlyAppCacheIfApplicable,
            LoadType::Params(params) => LoadType::Params(map_fn(params)),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum LoadError<E: Clone + PartialEq> {
    Net(String),
    Content(E),
}

#[hook]
pub fn use_load_and_map<C, P, F, I>(
    r#type: LoadType<P>,
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
    let last_used_route_cache = use_state_eq::<Option<I>, _>(|| None);
    let container_inner_result = {
        let app_content_context = app_content_container.clone();
        use_state_eq(|| {
            if !use_caches || (*app_content_container).is_used {
                return None;
            }
            let Some(page_container_inner) = (*app_content_container)
                .app_content
                .clone()
                .map(|a| I::decode(a))
                .flatten()
            else {
                return None;
            };
            Some(Ok(page_container_inner))
        })
    };
    #[cfg(feature = "client")]
    {
        let container_inner_result = container_inner_result.clone();
        use_effect_with(r#type, move |r#type| {
            let LoadType::Params(params) = r#type else {
                return;
            };
            let is_app_content_used = app_content_container.is_used;
            if use_caches {
                app_content_container.dispatch(AppContentContainerAction::MarkAsUsed);
            }
            if use_caches && !is_app_content_used && *container_inner_result != None {
                return;
            }
            if use_caches {
                if let Some(route_container_inner) = location.state::<I>().map(|i| (*i).clone()) {
                    let is_same_cache = last_used_route_cache
                        .as_ref()
                        .map(|l| l == &route_container_inner)
                        .unwrap_or(false);
                    last_used_route_cache.set(Some(route_container_inner.clone()));
                    if !is_same_cache {
                        app_content_container.dispatch(AppContentContainerAction::NewContent(
                            route_container_inner.encode(),
                        ));
                        container_inner_result.set(Some(Ok(route_container_inner)));
                        return;
                    }
                } else {
                    last_used_route_cache.set(None);
                }
            }
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
        });
    }
    container_inner_result
}

#[hook]
pub fn use_load<C, P>(
    r#type: LoadType<P>,
    use_caches: bool,
) -> UseStateHandle<Option<Result<C::Inner, LoadError<C::Error>>>>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: ExternalCodable + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    use_load_and_map::<C, P, _, C::Inner>(r#type, |i| i, use_caches)
}
