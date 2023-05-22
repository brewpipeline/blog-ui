use std::marker::PhantomData;

use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;

use crate::components::pagination::*;

use crate::Route;

pub trait ExternalListContainer: Clone + PartialEq + for<'a> Deserialize<'a> {
    fn url(limit: u64, skip: u64) -> String;
    type Item: PartialEq;
    fn items(self) -> Vec<Self::Item>;
    fn total(&self) -> u64;
    fn skip(&self) -> u64;
    fn limit(&self) -> u64;
}

#[derive(Properties, PartialEq)]
pub struct ListProps<C>
where
    C: ExternalListContainer + 'static,
{
    #[prop_or(10)]
    pub items_per_page: u64,
    pub route_to_page: Route,
    #[prop_or_default]
    _p_c: PhantomData<C>,
    pub component: Callback<Option<C::Item>, Html>,
}

#[function_component(List)]
pub fn list<C>(props: &ListProps<C>) -> Html
where
    C: ExternalListContainer + 'static,
{
    let page = use_location()
        .unwrap()
        .query::<PageQuery>()
        .map(|it| it.page)
        .unwrap_or(1);
    let limit = props.items_per_page;
    let skip = (page - 1) * limit;
    let route_to_page = props.route_to_page.clone();
    
    let list_container = use_state_eq(|| None);
    {
        let list_container = list_container.clone();
        use_effect_with(page, move |_| {
            let list_container = list_container.clone();
            list_container.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let list_container_url = C::url(limit, skip);
                let fetched_list_container: C = Request::get(list_container_url.as_str())
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                list_container.set(Some(fetched_list_container));
            });
            || ()
        });
    }

    let Some(list_container) = (*list_container).clone() else {
        return (0..limit).map(|_| {
            props.component.emit(Option::None)
        }).collect::<Html>()
    };
    let total_pages = (list_container.total() as f64 / limit as f64).ceil() as u64;
    html! {
        <>
            {
                list_container.items().into_iter().map(|item| {
                    props.component.emit(Option::Some(item))
                }).collect::<Html>()
            }
            <Pagination
                { page }
                { total_pages }
                { route_to_page }
            />
        </>
    }
}