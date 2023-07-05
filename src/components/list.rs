use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::load::*;
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

    html! {
        <Suspense fallback={
            (0..items_per_page).map(|_| {
                component.emit(None)
            }).collect::<Html>()
        }>
            <Load<C, ExternalListContainerParams<P>>
                params={ ExternalListContainerParams {
                    params,
                    limit: items_per_page,
                    skip: offset,
                } }
                component={ move |c: Result<C, String>| {
                    let Ok(list_container) = c else {
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
                                    route_to_page={ route_to_page.clone() }
                                />
                            }
                        } else {
                            { children.clone() }
                        }
                    }
                } }
            />
        </Suspense>
    }
}
