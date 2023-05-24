use serde::Deserialize;
use yew::prelude::*;
use gloo_net::http::Request;

use crate::hash_map_context::*;

pub trait ExternalItem: Clone + PartialEq + for<'a> Deserialize<'a> {
    fn url(id: u64) -> String;
}

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
        use_effect_with((), move |_| {
            let item = item.clone();
            if (*item) == None {
                item.set(None);
                wasm_bindgen_futures::spawn_local(async move {
                    let item_url = I::url(item_id);
                    let fetched_item: I = Request::get(item_url.as_str())
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    if let Some(items_cache) = items_cache {
                        items_cache.dispatch(ReducibleHashMapAction::Single(fetched_item.clone()))
                    }
                    item.set(Some(fetched_item));
                });
            }
            || ()
        });
    }

    props.component.emit((*item).clone())
}