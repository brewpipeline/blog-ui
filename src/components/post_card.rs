use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::author_image::*;
use crate::components::delayed_component::*;
use crate::components::optional_image::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PostCardProps {
    pub post: Option<Post>,
    pub is_full: bool,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let PostCardProps { post, is_full } = props.clone();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();
    let recommended = use_state(|| post.as_ref().map(|p| p.recommended).unwrap_or(false));
    {
        let post = post.clone();
        let recommended = recommended.clone();
        use_effect_with(post, move |p| {
            if let Some(p) = p {
                recommended.set(p.recommended);
            }
        });
    }
    #[cfg(feature = "client")]
    let star_button = {
        if logged_user_context.is_not_inited() {
            html! {}
        } else {
            match (post.as_ref(), logged_user_context.author()) {
                (Some(post), Some(author))
                    if author.editor == 1 && author.blocked == 0 && is_full =>
                {
                    let recommended_state = recommended.clone();
                    let post_id = post.id;
                    let logged_user_context = logged_user_context.clone();
                    let onclick = {
                        let recommended_state = recommended_state.clone();
                        let logged_user_context = logged_user_context.clone();
                        Callback::from(move |e: web_sys::MouseEvent| {
                            e.prevent_default();
                            let Some(token) = logged_user_context.token().cloned() else {
                                return;
                            };
                            let recommended_state = recommended_state.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                let res = API::<()>::get(Tokened {
                                    token,
                                    params: UpdatePostRecommendedParams {
                                        id: post_id,
                                        value: !*recommended_state,
                                    },
                                })
                                .await;
                                if let Ok(API::Success { .. }) = res {
                                    recommended_state.set(!*recommended_state);
                                }
                            });
                        })
                    };
                    html! {
                        <>
                            { " " }
                            <a href="#" class="text-decoration-none" {onclick} title={ if *recommended { "Убрать из рекомендаций" } else { "Добавить в рекомендации" } }>
                                <i
                                    class={ classes!("bi", if *recommended { "bi-star-fill" } else { "bi-star" }) }
                                ></i>
                            </a>
                        </>
                    }
                }
                _ => html! {},
            }
        }
    };
    #[cfg(not(feature = "client"))]
    let star_button: Html = html! {};

    let edit_button = if logged_user_context.is_not_inited() {
        html! {}
    } else {
        match (post.as_ref(), logged_user_context.author()) {
            (Some(post), Some(author))
                if (author.id == post.author.id || author.editor == 1) && author.blocked == 0 =>
            {
                html! {
                    <>
                        { " " }
                        <Link<Route, (), Post>
                            classes="text-decoration-none"
                            to={ Route::EditPost { id: post.id } }
                            state={ Some(post.clone()) }
                        >
                            <i title="Редактировать публикацию" class="bi bi-pencil-square"></i>
                        </Link<Route, (), Post>>
                    </>
                }
            }
            _ => html! {},
        }
    };

    let main_content = html! {
        <>
            <div class="img-block bd-placeholder-img" style="height:194px;width:100%;overflow:hidden;">
                <OptionalImage
                    alt={ post.as_ref().map(|p| p.title.clone()) }
                    image={
                        post
                            .as_ref()
                            .map(|p| p.image_url.clone())
                            .flatten()
                            .map(|u| image_url_formatter(ImageType::Medium, u))
                    }
                />
            </div>
            <div class="card-body pb-0">
                <h4 class="card-title placeholder-glow">
                    if let Some(title) = post.as_ref().map(|p| p.title.clone()) {
                        { title }
                    } else {
                        <span class="placeholder col-6 bg-secondary"></span>
                    }
                </h4>
                <article class="card-text placeholder-glow">
                    if let Some(text) = post.as_ref().map(|post| {
                        if let (Some(content), true) = (post.content.clone(), is_full) {
                            let content = content.map_in_pattern(["<img", ">"], |i| {
                                i.map_in_pattern(["src=\"", "\""], |u| {
                                    image_url_formatter(ImageType::Medium, u)
                                })
                            });
                            Html::from_html_unchecked(AttrValue::from(content))
                        } else {
                            html! { post.summary.clone() }
                        }
                    }) {
                        { text }
                    } else {
                        <span class="placeholder col-7 bg-secondary"></span> { " " }
                        <span class="placeholder col-4 bg-secondary"></span> { " " }
                        <span class="placeholder col-4 bg-secondary"></span> { " " }
                        <span class="placeholder col-6 bg-secondary"></span> { " " }
                        <span class="placeholder col-6 bg-secondary"></span> { " " }
                        <span class="placeholder col-5 bg-secondary"></span> { " " }
                        <span class="placeholder col-5 bg-secondary"></span> { " " }
                        <span class="placeholder col-3 bg-secondary"></span> { " " }
                        if is_full {
                            <span class="placeholder col-7 bg-secondary"></span> { " " }
                            <span class="placeholder col-4 bg-secondary"></span> { " " }
                            <span class="placeholder col-4 bg-secondary"></span> { " " }
                            <span class="placeholder col-6 bg-secondary"></span> { " " }
                            <span class="placeholder col-6 bg-secondary"></span> { " " }
                            <span class="placeholder col-5 bg-secondary"></span> { " " }
                            <span class="placeholder col-5 bg-secondary"></span> { " " }
                            <span class="placeholder col-3 bg-secondary"></span> { " " }
                        }
                    }
                </article>
            </div>
        </>
    };
    let tags_content = {
        if let Some(post) = post.as_ref() {
            let mut tags_len = 0;
            let tags = post.tags.iter().filter(|t| {
                tags_len += t.slug.len() + 1;
                tags_len < 30
            }).map(|tag| { html! {
                <>
                    <Link<Route, (), Tag>
                        classes="link link-underline-opacity-25 link-underline-opacity-100-hover"
                        to={ Route::Tag { id: tag.id, slug: tag.slug.clone() } }
                        state={ Some(tag.clone()) }
                    >
                        { tag.title.clone() }
                    </Link<Route, (), Tag>>
                    { " " }
                </>
            } }).collect::<Html>();
            html! {
                <p class="mt-0 mb-0">{ tags }</p>
            }
        } else {
            html! {
                <span class="placeholder col-6 bg-secondary"></span>
            }
        }
    };
    let publish_type = post.as_ref().map(|p| p.publish_type.clone());
    html! {
        <div class="card mb-3">
            <div class="card-header placeholder-glow border-0">
                <div class="row align-items-center">
                    <div class="d-flex col-5 align-items-center justify-content-start" style="height:24px;">
                        <div class="img-block rounded me-1" style="height:24px;width:24px;overflow:hidden;">
                            <AuthorImage author={ post.as_ref().map(|p| p.author.clone()) } />
                        </div>
                        if let Some(post) = &post {
                            <Link<Route, (), Author>
                                classes="text-decoration-none"
                                to={ Route::Author { slug: post.author.slug.clone() } }
                                state={ Some(post.author.clone()) }
                            >
                                <strong>
                                    { author_slug_formatter(&post.author) }
                                </strong>
                            </Link<Route, (), Author>>
                        } else {
                            <span class="placeholder col-3 bg-secondary"></span>
                        }
                    </div>
                    <div class="d-flex col-7 align-items-center justify-content-end" style="height: 24px;">
                        <DelayedComponent<Option<Post>> component={ |post: Option<Post>| html! {
                            if let Some(post) = post {
                                <small> { date::format(post.created_at) } </small>
                            } else {
                                <span class="placeholder col-3 bg-secondary"></span>
                            }
                        } } deps={ post.clone() } />
                    </div>
                </div>
            </div>
            if let (Some(post), false) = (post.as_ref(), is_full) {
                <Link<Route, (), Post>
                    classes="text-decoration-none"
                    to={ Route::Post { slug: post.slug.clone(), id: post.id } }
                    state={ Some(post.clone()) }
                >
                    { main_content }
                </Link<Route, (), Post>>
            } else {
                { main_content }
            }
            <div class="card-footer placeholder-glow">
                <div class="row align-items-center">
                    <div class="d-flex col-10 align-items-center justify-content-start" style="height: 24px;">
                        { tags_content }
                    </div>
                    <div class="d-flex col-2 align-items-center justify-content-end" style="height: 24px;">
                        <p class="mt-0 mb-0">
                            if publish_type == Some(PublishType::Unpublished) {
                                <i title="Неопубликовано" class="bi bi-clipboard2-x-fill"></i>
                            }
                            if publish_type == Some(PublishType::Hidden) {
                                <i title="Скрыто" class="bi bi-eye-slash-fill"></i>
                            }
                            { star_button }
                            { edit_button }
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
