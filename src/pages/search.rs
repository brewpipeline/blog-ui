use noneifempty::*;
use yew::prelude::*;

use crate::components::author_card::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::search_field::*;
use crate::components::warning::*;
use crate::content::*;
use crate::lang;
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
                <SearchField id="pageSearchField" />
            </div>
            {
                match mode {
                    SearchMode::Posts { query } => html! {
                        if let Some(query) = query.none_if_empty() {
                            <List<API<PostsContainer>, OptionTokened<PostsContainerParams>>
                                r#type={ LoadType::Params(OptionTokened {
                                    token: None,
                                    params: PostsContainerParams {
                                        publish_type: PublishType::Published,
                                        search_query: Some(query.clone()),
                                        author_id: None,
                                        tag_id: None
                                    }
                                }) }
                                route_to_page={ Route::PostsSearch { query: query.clone() } }
                                component={ |post| html! { <PostCard { post } is_full=false /> } }
                                error_component={ |_| html! { <Warning text={ lang::SEARCH_POSTS_ERROR } /> } }
                            >
                                <Warning text={ lang::SEARCH_POSTS_EMPTY } />
                            </List<API<PostsContainer>, OptionTokened<PostsContainerParams>>>
                        } else {
                            <Warning text={ lang::SEARCH_POSTS_HINT } />
                        }
                    },
                    SearchMode::Authors { query } => html! {
                        if let Some(query) = query.none_if_empty() {
                            <List<API<AuthorsContainer>, AuthorsContainerSearchParams>
                                r#type={ LoadType::Params(AuthorsContainerSearchParams { query: query.clone() }) }
                                route_to_page={ Route::AuthorsSearch { query: query.clone() } }
                                component={ |author| html! { <AuthorCard { author } link_to=true /> } }
                                error_component={ |_| html! { <Warning text={ lang::SEARCH_AUTHORS_ERROR } /> } }
                            >
                                <Warning text={ lang::SEARCH_AUTHORS_EMPTY } />
                            </List<API<AuthorsContainer>, AuthorsContainerSearchParams>>
                        } else {
                            <Warning text={ lang::SEARCH_AUTHORS_HINT } />
                        }
                    },
                }
            }
        </>

    }
}
