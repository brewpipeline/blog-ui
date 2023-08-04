use super::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pagination::*;
use crate::utils::*;

use crate::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct ListProps<C, P = ()>
where
    C: ExternalResultContainer
        + RequestableItem<ExternalListContainerParams<P>>
        + Clone
        + PartialEq
        + 'static,
    C::Inner: ExternalListContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalListContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    pub params: P,
    #[prop_or(10)]
    pub items_per_page: u64,
    pub route_to_page: Route,
    pub component: Callback<Option<<C::Inner as ExternalListContainer>::Item>, Html>,
    pub error_component: Callback<ExternalError<C::Error>, Html>,
    pub children: Children,
}

#[function_component(List)]
pub fn list<C, P = ()>(props: &ListProps<C, P>) -> Html
where
    C: ExternalResultContainer
        + RequestableItem<ExternalListContainerParams<P>>
        + Clone
        + PartialEq
        + 'static,
    C::Inner: ExternalListContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalListContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    let ListProps {
        params,
        items_per_page,
        route_to_page,
        component,
        error_component,
        children,
    } = props.clone();

    let location = use_location().unwrap();
    let page = location.query::<PageQuery>().map(|it| it.page).unwrap_or(1);
    let offset = (page - 1) * items_per_page;

    let list_result_container: UseStateHandle<
        Option<
            Result<
                <C as ExternalResultContainer>::Inner,
                ExternalError<<C as ExternalResultContainer>::Error>,
            >,
        >,
    > = use_state_eq(|| None);
    #[cfg(feature = "client")]
    {
        let list_result_container = list_result_container.clone();
        use_effect_with(
            (params, items_per_page, offset),
            move |(params, items_per_page, offset)| {
                list_result_container.set(None);
                let list_result_container = list_result_container.clone();
                let params = params.clone();
                let items_per_page = *items_per_page;
                let offset = *offset;
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_list_result_container = C::get(ExternalListContainerParams {
                        params,
                        limit: items_per_page,
                        skip: offset,
                    })
                    .await
                    .map_err(|err| ExternalError::Net(err.to_string()))
                    .and_then(|r| r.result().map_err(|e| ExternalError::Content(e)));
                    list_result_container.set(Some(fetched_list_result_container));
                });
            },
        );
    }

    let Some(list_result_container) = (*list_result_container).clone() else {
        return (0..items_per_page).map(|_| {
            component.emit(None)
        }).collect::<Html>()
    };
    match list_result_container {
        Ok(list_container) => {
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
        Err(err) => error_component.emit(err),
    }
}
