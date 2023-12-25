use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

const ELLIPSIS: &str = "\u{02026}";
const LINKS_PER_SIDE: usize = 3;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct PageQuery {
    pub page: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PaginationProps {
    pub page: u64,
    pub total_pages: u64,
    pub route_to_page: Route,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    html! {
        <nav aria-label="Page navigation example">
            <ul class="pagination justify-content-center">
                { view_relnav_previous_button(props) }
                { view_links(props) }
                { view_relnav_next_button(props) }
            </ul>
        </nav>
    }
}

fn render_link(to_page: u64, props: &PaginationProps) -> Html {
    let PaginationProps {
        page,
        route_to_page,
        ..
    } = props.clone();

    let is_current_class = if to_page == page { "active" } else { "" };

    html! {
        <li class={ classes!("page-item", "text-center", is_current_class) }>
            <Link<Route, PageQuery, ()>
                classes={ classes!("page-link") }
                to={ route_to_page }
                query={ Some(PageQuery{page: to_page}) }
            >
                { to_page }
            </Link<Route, PageQuery, ()>>
        </li>
    }
}

fn render_links<P>(mut pages: P, len: usize, max_links: usize, props: &PaginationProps) -> Html
where
    P: Iterator<Item = u64> + DoubleEndedIterator,
{
    if len > max_links {
        let last_link = render_link(pages.next_back().unwrap(), props);
        // remove 1 for the ellipsis and 1 for the last link
        let links = pages
            .take(max_links - 2)
            .map(|page| render_link(page, props));
        html! {
            <>
                { for links }
                <li class="page-item text-center disabled"><span class="page-link">{ ELLIPSIS }</span></li>
                { last_link }
            </>
        }
    } else {
        html! { for pages.map(|page| render_link(page, props)) }
    }
}

fn view_links(props: &PaginationProps) -> Html {
    let PaginationProps {
        page, total_pages, ..
    } = *props;

    let pages_prev = page.checked_sub(1).unwrap_or_default() as usize;
    let pages_next = (total_pages - page) as usize;

    let links_left = LINKS_PER_SIDE.min(pages_prev)
        // if there are less than `LINKS_PER_SIDE` to the right, we add some more on the left.
        + LINKS_PER_SIDE.checked_sub(pages_next).unwrap_or_default();
    let links_right = 2 * LINKS_PER_SIDE - links_left;

    html! {
        <>
            { render_links(1..page, pages_prev, links_left, props) }
            { render_link(page, props) }
            { render_links(page + 1..=total_pages, pages_next, links_right, props) }
        </>
    }
}

fn view_relnav_previous_button(props: &PaginationProps) -> Html {
    let PaginationProps {
        page,
        route_to_page: to,
        ..
    } = props.clone();

    let is_current_class = if page == 1 { "disabled" } else { "" };

    html! {
        <li class={ classes!("page-item", "text-center", is_current_class) }>
            <Link<Route, PageQuery, ()>
                classes={ classes!("page-link") }
                query={ Some(PageQuery { page: page.checked_sub(1).unwrap_or_default() } ) }
                to={ to.clone() }
            >
                { "<" }
            </Link<Route, PageQuery, ()>>
        </li>
    }
}

fn view_relnav_next_button(props: &PaginationProps) -> Html {
    let PaginationProps {
        page,
        total_pages,
        route_to_page: to,
    } = props.clone();

    let is_current_class = if page == total_pages { "disabled" } else { "" };

    html! {
        <li class={ classes!("page-item", "text-center", is_current_class) }>
            <Link<Route, PageQuery, ()>
                classes={ classes!("page-link") }
                query={ Some(PageQuery{page: page + 1}) }
                { to }
            >
                { ">" }
            </Link<Route, PageQuery, ()>>
        </li>
    }
}
