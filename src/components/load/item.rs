use yew::prelude::*;

use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ItemProps<C, P = ()>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: ExternalCodable + ExternalItemContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalItemContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    pub r#type: LoadType<P>,
    #[prop_or_default]
    pub use_caches: bool,
    pub component: Callback<Option<<C::Inner as ExternalItemContainer>::Item>, Html>,
    pub error_component: Callback<LoadError<C::Error>, Html>,
}

#[function_component(Item)]
pub fn item<C, P = ()>(props: &ItemProps<C, P>) -> Html
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: ExternalCodable + ExternalItemContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalItemContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    let ItemProps {
        r#type,
        use_caches,
        component,
        error_component,
    } = props.clone();

    let item_result_container = use_load::<C, P>(r#type, use_caches);

    let Some(item_result_container) = (*item_result_container).clone() else {
        return component.emit(None);
    };
    match item_result_container {
        Ok(item_container) => component.emit(Some(item_container.item())),
        Err(err) => error_component.emit(err),
    }
}
