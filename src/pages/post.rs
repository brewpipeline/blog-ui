use yew::prelude::*;

use crate::components::comments::*;
use crate::components::item::*;
use crate::components::meta::*;
use crate::components::post_card::*;
use crate::components::warning::*;
use crate::content;
use crate::utils::*;

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
                let is_post_invalid = post
                    .as_ref()
                    .map(|p| p.id != id || p.slug != slug)
                    .unwrap_or(false);
                if is_post_invalid {
                    return html! {
                        <>
                            <Meta title="Ссылка на публикацию повреждена" noindex=true />
                            <Warning text="Ссылка на публикацию повреждена!" />
                        </>
                    }
                }
                let meta = if let Some(p) = post.as_ref() {
                    html! {
                        <Meta
                            r#type="article"
                            title={ format!("{} - Публикация", p.title.clone()) }
                            description={ p.summary.clone() }
                            keywords={ p.joined_tags_string(", ") }
                            image={ p.image_url.clone().unwrap_or_default() }
                            noindex={ p.publish_type != content::PublishType::Published }
                        />
                    }
                } else {
                    html! { <Meta title="Публикация" noindex=true /> }
                };
                let comments = if let Some(p) = post.clone() {
                    html! { <Comments post={ p } /> }
                } else {
                    html! { <></> }
                };
                html! {
                    <>
                        { meta }
                        <PostCard post={ post.clone() } is_full=true />
                        { comments }
                    </>
                }
            } }
            error_component={ |_| html! {
                <>
                    <Meta title="Ошибка загрузки публикации" noindex=true />
                    <Warning text="Ошибка загрузки публикации!" />
                </>
            } }
        />
    }
}
