use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pagination::*;
use crate::utils::get::*;

use crate::Route;

#[derive(Clone, PartialEq)]
pub struct ExternalListContainerParams<P: Clone + PartialEq> {
    pub params: P,
    pub limit: u64,
    pub skip: u64,
}

#[async_trait(?Send)]
pub trait ExternalListContainer: Clone + PartialEq {
    type Item: Clone + PartialEq;
    fn items(self) -> Vec<Self::Item>;
    fn total(&self) -> u64;
    fn skip(&self) -> u64;
    fn limit(&self) -> u64;
}

#[derive(Properties, PartialEq, Clone)]
pub struct ListProps<C, P = ()>
where
    C: ExternalListContainer + RequestableItem<ExternalListContainerParams<P>> + 'static,
    P: Clone + PartialEq + 'static,
{
    pub params: P,
    #[prop_or(10)]
    pub items_per_page: u64,
    pub route_to_page: Route,
    pub component: Callback<Option<C::Item>, Html>,
    pub children: Children,
}

#[function_component(List)]
pub fn list<C, P = ()>(props: &ListProps<C, P>) -> Html
where
    C: ExternalListContainer + RequestableItem<ExternalListContainerParams<P>> + 'static,
    P: Clone + PartialEq + 'static,
{
    let ListProps {
        params,
        items_per_page,
        route_to_page,
        component,
        children,
    } = props.clone();

    let location = use_location().unwrap();
    let page = location.query::<PageQuery>().map(|it| it.page).unwrap_or(1);
    let offset = (page - 1) * items_per_page;

    let list_container = use_state_eq(|| None);
    {
        let list_container = list_container.clone();
        use_effect_with_deps(
            move |(params, items_per_page, offset)| {
                list_container.set(None);
                let list_container = list_container.clone();
                let params = params.clone();
                let items_per_page = *items_per_page;
                let offset = *offset;
                wasm_bindgen_futures::spawn_local(async move {
                    let Ok(fetched_list_container) = C::get(ExternalListContainerParams {
                        params,
                        limit: items_per_page,
                        skip: offset,
                    })
                    .await else {
                        return
                    };
                    list_container.set(Some(fetched_list_container));
                });
                || ()
            },
            (params, items_per_page, offset),
        );
    }

    let Some(list_container) = (*list_container).clone() else {
        return (0..items_per_page).map(|_| {
            component.emit(None)
        }).collect::<Html>()
    };
    let total_pages = (list_container.total() as f64 / items_per_page as f64).ceil() as u64;
    let items = list_container.items();
    html! {
        if items.len() > 0 {
            {
                items.into_iter().map(|item| {
                    component.emit(Some(item))
                }).collect::<Html>()
            }
            if total_pages > 1 {
                <Pagination
                    { page }
                    { total_pages }
                    { route_to_page }
                />
            }
        } else {
            { children.clone() }
        }
    }
}
