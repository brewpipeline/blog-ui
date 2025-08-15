use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::author_image::*;
use crate::components::delayed_component::*;
use crate::components::optional_image::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct RecommendedPostCardProps {
    pub post: Option<Post>,
}

#[function_component(RecommendedPostCard)]
pub fn recommended_post_card(props: &RecommendedPostCardProps) -> Html {
    let RecommendedPostCardProps { post } = props.clone();

    let image_block = if let Some(url) = post.as_ref().and_then(|p| p.image_url.clone()) {
        html! {
            <div class="img-block bd-placeholder-img" style="height:194px;width:100%;overflow:hidden;">
                <OptionalImage
                    alt={ post.as_ref().map(|p| p.title.clone()) }
                    image={ Some(image_url_formatter(ImageType::Medium, url)) }
                />
            </div>
        }
    } else {
        Html::default()
    };

    let main_content = html! {
        <>
            { image_block }
            <div class="card-body">
                <h5 class="card-title placeholder-glow">
                    <DelayedComponent<Option<Post>> component={ |post: Option<Post>| html! {
                        if let Some(post) = post {
                            { post.title }
                        } else {
                            <span class="placeholder col-6 bg-secondary"></span>
                        }
                    } } deps={ post.clone() } />
                </h5>
                <article class="card-text placeholder-glow">
                    {
                        if let Some(post) = post.as_ref() {
                            html! { post.summary.clone() }
                        } else {
                            html! {
                                <>
                                    <span class="placeholder col-7 bg-secondary"></span> { " " }
                                    <span class="placeholder col-4 bg-secondary"></span> { " " }
                                    <span class="placeholder col-4 bg-secondary"></span> { " " }
                                    <span class="placeholder col-6 bg-secondary"></span> { " " }
                                    <span class="placeholder col-6 bg-secondary"></span> { " " }
                                    <span class="placeholder col-5 bg-secondary"></span> { " " }
                                    <span class="placeholder col-5 bg-secondary"></span> { " " }
                                    <span class="placeholder col-3 bg-secondary"></span> { " " }
                                </>
                            }
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
                        classes="link-dark link-underline-opacity-25 link-underline-opacity-100-hover"
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

    html! {
        <div class="card mb-3">
            <div class="card-header placeholder-glow border-0">
                <div class="row align-items-center">
                    <div class="d-flex col-12 align-items-center justify-content-start" style="height:24px;">
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
                </div>
            </div>
            if let Some(post) = post.as_ref() {
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
                    <div class="d-flex col-12 align-items-center justify-content-start" style="height: 24px;">
                        { tags_content }
                    </div>
                </div>
            </div>
        </div>
    }
}
