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
    <C::Inner as ExternalItemContainer>::Item: Identifiable + Clone + PartialEq + 'static,
    P: Identifiable + Clone + PartialEq + 'static,
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
    <C::Inner as ExternalItemContainer>::Item:
        Identifiable<Id = P::Id> + Clone + PartialEq + 'static,
    P: Identifiable + Clone + PartialEq + 'static,
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
                <<C as ExternalResultContainer>::Inner as ExternalItemContainer>::Item,
                ExternalError<<C as ExternalResultContainer>::Error>,
            >,
        >,
    > = use_state_eq(|| None);
    {
        let location = location.clone();
        let item_result = item_result.clone();
        use_effect_with(params, move |params| {
            if let Some(cached_item) = location
                .state::<<C::Inner as ExternalItemContainer>::Item>()
                .map(|i| (*i).clone())
                .or_else(|| {
                    location
                        .state::<std::collections::HashMap<
                            P::Id,
                            <C::Inner as ExternalItemContainer>::Item,
                        >>()
                        .map(|i| (*i).get(params.id()).cloned())
                        .flatten()
                })
                .filter(|i| i.id() == params.id())
            {
                item_result.set(Some(Ok(cached_item)));
                return;
            } else {
                item_result.set(None);
            }

            let params = params.clone();
            let item_result = item_result.clone();
            #[cfg(feature = "client")]
            wasm_bindgen_futures::spawn_local(async move {
                match C::get(params).await {
                    Ok(item_result_container) => {
                        item_result.set(Some(
                            item_result_container
                                .result()
                                .map(|i| i.item())
                                .map_err(|e| ExternalError::Content(e)),
                        ));
                    }
                    Err(err) => {
                        item_result.set(Some(Err(ExternalError::Net(err.to_string()))));
                    }
                }
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
