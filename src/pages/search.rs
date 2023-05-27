use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::list::*;
use crate::components::post_card::*;
use crate::components::search_field::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::html_document;
use crate::utils::not_empty::*;

use crate::Route;

#[derive(PartialEq, Properties, Clone)]
pub struct SearchProps {
    pub mode: SearchMode,
}

#[function_component(Search)]
pub fn search(props: &SearchProps) -> Html {
    let SearchProps { mode } = props.clone();
    html_document::reset_title_and_meta();
    html_document::set_prefix_default_title(mode.title());
    html! {
        <>
            <div class="mb-3 d-block d-lg-none">
                <SearchField />
            </div>
            {
                match mode {
                    SearchMode::Posts { query } => html! {
                        if let Some(query) = not_empty(query) {
                            <List<content::PostsContainer, content::PostsContainerSearchParam>
                                params={ content::PostsContainerSearchParam { query: query.clone() } }
                                route_to_page={ Route::PostsSearch { query: query.clone() } }
                                component={ |post| html! { <PostCard { post } /> } }
                            >
                                <Warning text="Публикаций не найдено!" />
                            </List<content::PostsContainer, content::PostsContainerSearchParam>>
                        } else {
                            <Warning text="Начните ввод для поиска публикаций..." />
                        }
                    },
                    SearchMode::Authors { query } => html! {
                        if let Some(query) = not_empty(query) {
                            <List<content::UsersContainer, content::UsersContainerSearchParam>
                                params={ content::UsersContainerSearchParam { query: query.clone() } }
                                route_to_page={ Route::AuthorsSearch { query: query.clone() } }
                                component={ |user| html! { <AuthorCard { user } /> } }
                            >
                                <Warning text="Авторов не найдено!" />
                            </List<content::UsersContainer, content::UsersContainerSearchParam>>
                        } else {
                            <Warning text="Начните ввод для поиска авторов..." />
                        }
                    },
                }
            }
        </>

    }
}
