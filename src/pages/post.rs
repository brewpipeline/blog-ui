use yew::prelude::*;

use crate::components::comment_card::*;
use crate::components::item::*;
use crate::components::list::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::simple_title_card::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::*;

use crate::Route;

#[derive(PartialEq, Properties, Clone)]
pub struct PostProps {
    pub slug: String,
    pub id: u64,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let PostProps { slug, id } = props.clone();
    let logged_user_context = use_context::<LoggedUserContext>().unwrap();
    html! {
        <Item<content::API<content::PostContainer>, content::OptionTokened<content::PostParams>>
            r#type={
                if !logged_user_context.is_not_inited() {
                    LoadType::Params(content::OptionTokened {
                        token: logged_user_context.token().cloned(),
                        params: content::PostParams { id }
                    })
                } else {
                    LoadType::OnlyAppCacheIfApplicable
                }
            }
            use_caches=true
            component={ move |post: Option<content::Post>| {
                if let Some(post) = &post {
                    if post.id != id || post.slug != slug {
                        return html! {
                            <>
                                <Meta title="Ссылка на публикацию повреждена" />
                                <Warning text="Ссылка на публикацию повреждена!" />
                            </>
                        }
                    }
                }
                html! {
                    <>
                        if let Some(post) = post.as_ref() {
                            <Meta
                                title={ format!("{} - Публикация", post.title.clone()) }
                                description={ post.summary.clone() }
                                keywords={ post.joined_tags_string(", ") }
                            />
                        } else {
                            <Meta title="Публикация" />
                        }
                        <PostCard post={ post.clone() } is_full=true />
                        if let Some(post) = post {
                            <SimpleTitleCard>
                                { "Комментарии" }
                            </SimpleTitleCard>
                            <List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParams>
                                r#type={ LoadType::Params(content::CommentsContainerPostIdParams { post_id: post.id }) }
                                items_per_page={ 50 }
                                route_to_page={ Route::Post { slug: post.slug, id: post.id } }
                                component={ |comment| html! { <CommentCard { comment } /> } }
                                error_component={ |_| html! { <Warning text="Ошибка загрузки комментариев!" /> } }
                            >
                                <Warning text="Нет комментариев." />
                            </List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParams>>
                        }
                    </>
                }
            } }
            error_component={ |_| html! {
                <>
                    <Meta title="Ошибка загрузки публикации" />
                    <Warning text="Ошибка загрузки публикации!" />
                </>
            } }
        />
    }
}
