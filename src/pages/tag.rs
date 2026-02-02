use yew::prelude::*;

use crate::components::item::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::simple_title_card::*;
use crate::components::warning::*;
use crate::content;
use crate::lang;
use crate::utils::*;

use crate::Route;

#[derive(PartialEq, Properties, Clone)]
pub struct TagProps {
    pub slug: String,
    pub id: u64,
}

#[function_component(Tag)]
pub fn post(props: &TagProps) -> Html {
    let TagProps { slug, id } = props.clone();
    html! {
        <Item<content::API<content::TagContainer>, content::TagIdParams>
            r#type={ LoadType::Params(content::TagIdParams { id }) }
            use_caches=true
            component={ move |tag: Option<content::Tag>| {
                if tag.as_ref().map(|t| t.id != id || t.slug != slug).unwrap_or(false) {
                    return html! {
                        <>
                            <Meta title={ lang::TAG_LINK_BROKEN_TITLE } noindex=true />
                            <Warning text={ lang::TAG_LINK_BROKEN_TEXT } />
                        </>
                    }
                }
                html! {
                    <>
                        if let Some(tag) = tag.as_ref() {
                            <Meta
                                title={ lang::tag_meta_title(&tag.title) }
                            />
                        } else {
                            <Meta title={ lang::TAG_TITLE } noindex=true />
                        }

                        <SimpleTitleCard>
                            { lang::TAG_PREFIX }
                            if let Some(tag) = &tag {
                                { &tag.title }
                            } else {
                                <span class="placeholder col-3 bg-secondary"></span>
                            }
                        </SimpleTitleCard>

                        if let Some(tag) = tag {
                            <List<content::API<content::PostsContainer>, content::OptionTokened<content::PostsContainerParams>>
                                r#type={ LoadType::Params(content::OptionTokened {
                                    token: None,
                                    params: content::PostsContainerParams {
                                        publish_type: content::PublishType::Published,
                                        search_query: None,
                                        author_id: None,
                                        tag_id: Some(tag.id)
                                    }
                                }) }
                                route_to_page={ Route::Tag { slug: tag.slug, id: tag.id } }
                                component={ |post| html! { <PostCard { post } is_full=false /> } }
                                error_component={ |_| html! { <Warning text={ lang::TAG_POSTS_ERROR } /> } }
                            >
                                <Warning text={ lang::TAG_POSTS_EMPTY } />
                            </List<content::API<content::PostsContainer>, content::OptionTokened<content::PostsContainerParams>>>
                        }
                    </>
                }
            } }
            error_component={ |_| html! {
                <>
                    <Meta title={ lang::TAG_ERROR_TITLE } noindex=true />
                    <Warning text={ lang::TAG_ERROR_TEXT } />
                </>
            } }
        />
    }
}
