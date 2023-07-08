use yew::prelude::*;

use crate::utils::get::*;

pub trait ExternalItem<P: PartialEq + Clone>: Clone + PartialEq + RequestableItem<P> {}

#[derive(PartialEq, Properties, Clone)]
pub struct ItemProps<I, P = ()>
where
    I: ExternalItem<P> + 'static,
    P: PartialEq + Clone + 'static,
{
    pub params: P,
    pub component: Callback<Option<I>, Html>,
}

#[function_component(Item)]
pub fn item<I, P = ()>(props: &ItemProps<I, P>) -> Html
where
    I: ExternalItem<P> + 'static,
    P: PartialEq + Clone + 'static,
{
    let ItemProps { params, component } = props.clone();

    let item = use_state_eq(|| None);
    {
        let item = item.clone();
        use_effect_with_deps(
            move |params| {
                item.set(None);
                let item = item.clone();
                let params = params.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match I::get(params).await {
                        Ok(fetched_item) => {
                            item.set(Some(fetched_item));
                        }
                        Err(err) => {
                            // TODO
                            web_sys::console::log_1(&err.to_string().as_str().into());
                        }
                    }
                });
            },
            params,
        );
    }

    component.emit((*item).clone())
}
