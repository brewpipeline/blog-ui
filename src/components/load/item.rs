use crate::utils::*;
use yew::prelude::*;
#[derive(PartialEq, Properties, Clone)]
pub struct ItemProps<C, P = ()>
where
    C: ExternalResultContainer + RequestableItem<P> + Clone + PartialEq + 'static,
    C::Inner: ExternalItemContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalItemContainer>::Item: ExternalCodable + Clone + PartialEq + 'static,
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
    C::Inner: ExternalItemContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalItemContainer>::Item: ExternalCodable + Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    let ItemProps {
        r#type,
        use_caches,
        component,
        error_component,
    } = props.clone();

    let item_result = use_load_and_map::<C, P, _, <C::Inner as ExternalItemContainer>::Item>(
        r#type,
        |i| i.item(),
        use_caches,
    );

    let Some(item_result) = (*item_result).clone() else {
        return component.emit(None);
    };
    match item_result {
        Ok(item) => component.emit(Some(item)),
        Err(err) => error_component.emit(err),
    }
}
