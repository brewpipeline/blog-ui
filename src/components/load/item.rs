use super::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ItemProps<C, P = ()>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: ExternalItemContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalItemContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    pub params: P,
    pub component: Callback<Option<<C::Inner as ExternalItemContainer>::Item>, Html>,
    pub error_component: Callback<ExternalError<C::Error>, Html>,
}

#[function_component(Item)]
pub fn item<C, P = ()>(props: &ItemProps<C, P>) -> Html
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: ExternalItemContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalItemContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    let ItemProps {
        params,
        component,
        error_component,
    } = props.clone();

    let location = use_location().unwrap();

    let item_result: UseStateHandle<
        Option<
            Result<
                <C::Inner as ExternalItemContainer>::Item,
                ExternalError<C::Error>,
            >,
        >,
    > = use_state_eq(|| None);
    #[cfg(feature = "client")]
    {
        let location = location.clone();
        let item_result = item_result.clone();
        use_effect_with(params, move |params| {
            if let Some(cached_item) = location
                .state::<<C::Inner as ExternalItemContainer>::Item>()
                .map(|i| (*i).clone())
            {
                item_result.set(Some(Ok(cached_item)));
                return;
            } else {
                item_result.set(None);
            }

            let params = params.clone();
            let item_result = item_result.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_item_result = C::get(params)
                    .await
                    .map_err(|err| ExternalError::Net(err.to_string()))
                    .and_then(|r| {
                        r.result()
                            .map(|i| i.item())
                            .map_err(|e| ExternalError::Content(e))
                    });
                item_result.set(Some(fetched_item_result));
            });
        });
    }

    let Some(item_result) = (*item_result).clone() else {
        return component.emit(None)
    };
    match item_result {
        Ok(item) => component.emit(Some(item)),
        Err(err) => error_component.emit(err),
    }
}
