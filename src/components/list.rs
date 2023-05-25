use std::marker::PhantomData;
use gloo_net::Error;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::{Request, Response};

use crate::components::pagination::*;
use crate::hash_map_context::*;

use crate::Route;

#[async_trait(?Send)]
pub trait ExternalListContainer: Clone + PartialEq {
    type UrlProps: PartialEq + Default;
    fn request(url_props: &Self::UrlProps, limit: u64, skip: u64) -> Request;
    async fn response(response: Response) -> Result<Self, Error>;
    type Item: Clone + PartialEq + KeyedItem<Key = u64>;
    fn items(&self) -> Vec<Self::Item>;
    fn total(&self) -> u64;
    fn skip(&self) -> u64;
    fn limit(&self) -> u64;
}

#[derive(Properties, PartialEq)]
pub struct ListProps<C>
where
    C: ExternalListContainer + 'static,
{
    #[prop_or_default]
    pub url_props: C::UrlProps,
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
    let items_cache = use_context::<HashMapContext<u64, C::Item>>();

    let page = use_location()
        .unwrap()
        .query::<PageQuery>()
        .map(|it| it.page)
        .unwrap_or(1);
    let url_props = &props.url_props;
    let limit = props.items_per_page;
    let skip = (page - 1) * limit;
    let route_to_page = props.route_to_page.clone();
    
    let list_container_request = C::request(url_props, limit, skip);
    let list_container = use_state_eq(|| None);
    {
        let items_cache = items_cache.clone();
        let list_container = list_container.clone();
        use_effect_with(page, move |_| {
            let items_cache = items_cache.clone();
            let list_container = list_container.clone();
            list_container.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_list_container = C::response(list_container_request
                    .send()
                    .await
                    .unwrap()
                )
                    .await
                    .unwrap();
                if let Some(items_cache) = items_cache {
                    items_cache.dispatch(ReducibleHashMapAction::Batch(fetched_list_container.items()))
                }
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
            if total_pages > 1 {
                <Pagination
                    { page }
                    { total_pages }
                    { route_to_page }
                />
            }
        </>
    }
}