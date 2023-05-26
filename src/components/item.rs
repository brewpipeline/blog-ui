use yew::prelude::*;

use crate::hash_map_context::*;
use crate::get::*;

pub struct ExternalItemParams {
    pub id: u64
}

pub trait ExternalItem: Clone + PartialEq + RequestableItem<ExternalItemParams> {}

#[derive(PartialEq, Properties)]
pub struct ItemProps<I>
where
    I: ExternalItem + 'static,
{
    pub item_id: u64,
    pub component: Callback<Option<I>, Html>,
    #[prop_or_default]
    pub ignore_cache: bool
}

#[function_component(Item)]
pub fn item<I>(props: &ItemProps<I>) -> Html
where
    I: ExternalItem + KeyedItem<Key = u64> + 'static,
{
    let items_cache = use_context::<HashMapContext<u64, I>>();

    let item_id = props.item_id;
    let cached_item = if let (Some(items_cache), false) = (&items_cache, props.ignore_cache) {
        items_cache.0.get(&item_id).cloned()
    } else {
        None
    };

    let item = use_state_eq(|| cached_item);
    {
        let item = item.clone();
        use_effect_with_deps(move |_| {
            if (*item) != None {
                return
            }
            item.set(None);
            let item = item.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_item = I::get(ExternalItemParams { id: item_id }).await.unwrap();
                if let Some(items_cache) = items_cache {
                    items_cache.dispatch(ReducibleHashMapAction::Single(fetched_item.clone()))
                }
                item.set(Some(fetched_item));
            });
        }, ());
    }

    props.component.emit((*item).clone())
}