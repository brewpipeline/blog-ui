use super::*;
use yew::prelude::*;

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

    let item_result = use_state_eq(|| None);
    {
        let item_result = item_result.clone();
        use_effect_with_deps(
            move |params| {
                item_result.set(None);
                let item_result = item_result.clone();
                let params = params.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match C::get(params).await {
                        Ok(fetched_item_result_container) => {
                            item_result.set(Some(
                                fetched_item_result_container
                                    .result()
                                    .map_err(|e| ExternalError::Content(e)),
                            ));
                        }
                        Err(err) => {
                            item_result.set(Some(Err(ExternalError::Net(err.to_string()))));
                        }
                    }
                });
            },
            params,
        );
    }

    let Some(item_result) = (*item_result).clone() else {
        return component.emit(None)
    };
    match item_result {
        Ok(item_container) => component.emit(Some(item_container.item())),
        Err(err) => error_component.emit(err),
    }
}
