pub mod item;
pub mod list;

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
) -> UseStateHandle<Option<Result<I, LoadError<C::Error>>>>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
    F: FnOnce(C::Inner) -> I + 'static,
    I: Clone + PartialEq + 'static,
{
    let location = use_location().unwrap();
    let container_inner_result = use_state_eq(|| None);
    #[cfg(feature = "client")]
    {
        let container_inner_result = container_inner_result.clone();
        use_effect_with(params, move |params| {
            if let Some(cached_container_inner) = location.state::<I>().map(|i| (*i).clone()) {
                container_inner_result.set(Some(Ok(cached_container_inner)));
                return;
            } else {
                container_inner_result.set(None);
            }
            let params = params.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_container_inner_result = C::get(params)
                    .await
                    .map_err(|err| LoadError::Net(err.to_string()))
                    .and_then(|r| r.result().map(inner_map).map_err(|e| LoadError::Content(e)));
                container_inner_result.set(Some(fetched_container_inner_result));
            });
        });
    }
    container_inner_result
}

#[hook]
pub fn use_load<C, P>(params: P) -> UseStateHandle<Option<Result<C::Inner, LoadError<C::Error>>>>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    use_load_and_map::<C, P, _, C::Inner>(params, |i| i)
}
