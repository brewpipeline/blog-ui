use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pagination::*;
use crate::utils::*;

use crate::Route;

pub trait PageProcessor {
    fn create_for_page(page: &u64) -> Self;
    fn limit(&self) -> u64;
    fn offset(&self) -> u64;
}

pub struct DefaultPageProcessor<const LIMIT: u64 = { blog_generic::ITEMS_PER_PAGE }> {
    page: u64,
}

impl<const LIMIT: u64> PageProcessor for DefaultPageProcessor<LIMIT> {
    fn create_for_page(page: &u64) -> Self {
        Self { page: *page }
    }
    fn limit(&self) -> u64 {
        LIMIT
    }
    fn offset(&self) -> u64 {
        blog_generic::offset_for_page::<LIMIT>(&self.page)
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ListProps<C, P = ()>
where
    C: ExternalResultContainer
        + RequestableItem<ExternalListContainerParams<P>>
        + Clone
        + PartialEq
        + 'static,
    C::Inner: ExternalCodable + ExternalListContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalListContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
{
    pub r#type: LoadType<P>,
    #[prop_or_default]
    pub use_caches: bool,
    pub route_to_page: Route,
    pub component: Callback<Option<<C::Inner as ExternalListContainer>::Item>, Html>,
    pub error_component: Callback<LoadError<C::Error>, Html>,
    pub children: Children,
}

#[function_component(List)]
pub fn list<C, P = (), PP = DefaultPageProcessor>(props: &ListProps<C, P>) -> Html
where
    C: ExternalResultContainer
        + RequestableItem<ExternalListContainerParams<P>>
        + Clone
        + PartialEq
        + 'static,
    C::Inner: ExternalCodable + ExternalListContainer + Clone + PartialEq + 'static,
    C::Error: Clone + PartialEq + 'static,
    <C::Inner as ExternalListContainer>::Item: Clone + PartialEq + 'static,
    P: Clone + PartialEq + 'static,
    PP: PageProcessor,
{
    let ListProps {
        r#type,
        use_caches,
        route_to_page,
        component,
        error_component,
        children,
    } = props.clone();

    let location = use_location().unwrap();
    let page = location.query::<PageQuery>().map(|it| it.page).unwrap_or(1);
    let page_processor = PP::create_for_page(&page);

    let list_result_container = use_load::<C, ExternalListContainerParams<P>>(
        r#type.map_params(|params| ExternalListContainerParams {
            params,
            limit: page_processor.limit(),
            skip: page_processor.offset(),
        }),
        use_caches,
    );

    let Some(list_result_container) = (*list_result_container).clone() else {
        return (0..page_processor.limit())
            .map(|_| component.emit(None))
            .collect::<Html>();
    };
    match list_result_container {
        Ok(list_container) => {
            let total_pages =
                (list_container.total() as f64 / page_processor.limit() as f64).ceil() as u64;
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
