use yew::prelude::*;

use crate::utils::get::*;

pub struct ExternalItemParams {
    pub id: u64,
}

pub trait ExternalItem: Clone + PartialEq + RequestableItem<ExternalItemParams> {}

#[derive(PartialEq, Properties)]
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
    let item_id = props.item_id;

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

    props.component.emit((*item).clone())
}
