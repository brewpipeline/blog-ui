use yew::prelude::*;

use crate::components::comment_card::*;
use crate::components::item::*;
use crate::components::list::*;
use crate::components::post_card::*;
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
    let app_meta = use_context::<AppMetaContext>().unwrap();
    html! {
        <Item<content::API<content::PostContainer>, content::PostIdParams>
            params={ content::PostIdParams { id } }
            use_caches=true
            component={ move |post: Option<content::Post>| {
                app_meta.dispatch([AppMetaAction::Title("Публикация".to_string())].into());
                if let Some(post) = &post {
                    if post.id != id || post.slug != slug {
                        return html! { <Warning text="Ссылка на публикацию повреждена" /> }
                    }
                    app_meta.dispatch([
                        AppMetaAction::Title(format!("{} - Публикация", post.title.clone())),
                        AppMetaAction::Description(post.summary.clone()),
                        AppMetaAction::Keywords(post.tags_string())
                    ].into());
                }
                html! {
                    <>
                        <PostCard post={ post.clone() } is_full=true link_to=false />
                        if let Some(post) = post {
                            <List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParams>
                                params={ content::CommentsContainerPostIdParams { post_id: post.id } }
                                items_per_page={ 100 }
                                route_to_page={ Route::Post { slug: post.slug, id: post.id } }
                                component={ |comment| html! { <CommentCard { comment } /> } }
                                error_component={ |_| html! { <Warning text="Ошибка загрузки комментариев" /> } }
                            >
                                <Warning text="Нет комментариев" />
                            </List<content::API<content::CommentsContainer>, content::CommentsContainerPostIdParams>>
                        }
                    </>
                }
            } }
            error_component={ |_| html! { <Warning text="Ошибка загрузки публикации" /> } }
        />
    }
}
