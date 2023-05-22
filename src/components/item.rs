use serde::Deserialize;
use yew::prelude::*;
use gloo_net::http::Request;

pub trait ExternalItem: Clone + PartialEq + for<'a> Deserialize<'a> {
    fn url(id: u64) -> String;
}

#[derive(PartialEq, Properties)]
pub struct ItemProps<I>
where
    I: ExternalItem + 'static,
{
    pub id: u64,
    pub component: Callback<Option<I>, Html>,
}

#[function_component(Item)]
pub fn item<I>(props: &ItemProps<I>) -> Html
where
    I: ExternalItem + 'static,
{
    let id = props.id;
    let item = use_state_eq(|| None);
    {
        let item = item.clone();
        use_effect_with((), move |_| {
            let item = item.clone();
            item.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let item_url = I::url(id);
                let fetched_item: I = Request::get(item_url.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                item.set(Some(fetched_item));
            });
            || ()
        });
    }
    props.component.emit((*item).clone())
}