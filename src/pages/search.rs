use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::search_field::*;
use crate::components::warning::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(PartialEq, Properties, Clone)]
pub struct SearchProps {
    pub mode: SearchMode,
}

#[function_component(Search)]
pub fn search(props: &SearchProps) -> Html {
    let SearchProps { mode } = props.clone();
    html! {
        <>
            <Meta title={ mode.title() } noindex=true />
            <div class="mb-3 d-block d-lg-none">
                <SearchField />
            </div>
            {
                match mode {
                    SearchMode::Posts { query } => html! {
                        if let Some(query) = not_empty(query) {
                            <List<API<PostsContainer>, PostsContainerSearchParams>
                                r#type={ LoadType::Params(PostsContainerSearchParams { query: query.clone() }) }
                                route_to_page={ Route::PostsSearch { query: query.clone() } }
                                component={ |post| html! { <PostCard { post } is_full=false /> } }
                                error_component={ |_| html! { <Warning text="Ошибка загрузки результатов поиска публикаций!" /> } }
                            >
                                <Warning text="Публикаций не найдено!" />
                            </List<API<PostsContainer>, PostsContainerSearchParams>>
                        } else {
                            <Warning text="Начните ввод для поиска публикаций..." />
                        }
                    },
                    SearchMode::Authors { query } => html! {
                        if let Some(query) = not_empty(query) {
                            <List<API<AuthorsContainer>, AuthorsContainerSearchParams>
                                r#type={ LoadType::Params(AuthorsContainerSearchParams { query: query.clone() }) }
                                route_to_page={ Route::AuthorsSearch { query: query.clone() } }
                                component={ |author| html! { <AuthorCard { author } link_to=true /> } }
                                error_component={ |_| html! { <Warning text="Ошибка загрузки результатов поиска авторов!" /> } }
                            >
                                <Warning text="Авторов не найдено!" />
                            </List<API<AuthorsContainer>, AuthorsContainerSearchParams>>
                        } else {
                            <Warning text="Начните ввод для поиска авторов..." />
                        }
                    },
                }
            }
        </>

    }
}
