use yew::prelude::*;

use crate::utils::get::*;

#[derive(PartialEq, Clone)]
pub struct ExternalItemParams {
    pub id: u64,
}

pub trait ExternalItem: Clone + PartialEq + RequestableItem<ExternalItemParams> {}

#[derive(PartialEq, Properties, Clone)]
pub struct ItemProps<I>
where
    I: ExternalItem + 'static,
{
    pub item_id: u64,
    pub component: Callback<Option<I>, Html>,
}

#[function_component(Item)]
pub fn item<I>(props: &ItemProps<I>) -> Html
where
    I: ExternalItem + 'static,
{
    let ItemProps { item_id, component } = props.clone();

    let item = use_state_eq(|| None);
    {
        let item = item.clone();
        use_effect_with_deps(
            move |item_id| {
                item.set(None);
                let item = item.clone();
                let item_id = *item_id;
                wasm_bindgen_futures::spawn_local(async move {
                    let Ok(fetched_item) = I::get(ExternalItemParams { id: item_id }).await else {
                        return
                    };
                    item.set(Some(fetched_item));
                });
            },
            item_id,
        );
    }

    component.emit((*item).clone())
}
