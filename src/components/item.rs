use yew::prelude::*;

use crate::components::load::*;
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

    html! {
        <Suspense fallback={ component.emit(None) }>
            <Load<I, ExternalItemParams>
                params={ ExternalItemParams { id: item_id } }
                component={ move |i: Result<I, String>| {
                    let Ok(item) = i else {
                        return component.emit(None)
                    };
                    component.emit(Some(item))
                } }
            />
        </Suspense>
    }
}
