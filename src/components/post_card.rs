use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::svg_image::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct PostCardProps {
    pub post: Option<Post>,
    pub is_full: bool,
    pub link_to: bool,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let PostCardProps {
        post,
        is_full,
        link_to,
    } = props.clone();

    let logged_user_context = use_context::<LoggedUserContext>().unwrap();

    let main_content = html! {
        <>
            <div
                style={
                    format!(
                        "height:194px;width:100%;--image-url:url({});",
                        post
                            .as_ref()
                            .map(|p| p.image_url())
                            .unwrap_or_default()
                    )
                }
                class="img-block bd-placeholder-img card-img-top"
                role="img"
            />
            <div class="card-body">
                <h5 class="card-title placeholder-glow">
                    if let Some(title) = post.as_ref().map(|p| p.title.clone()) {
                        { title }
                    } else {
                        <span class="placeholder col-6"></span>
                    }
                </h5>
                <p class="card-text placeholder-glow">
                    if let Some(text) = post.as_ref().map(|post| {
                        if let (Some(content), true) = (post.content.clone(), is_full) {
                            Html::from_html_unchecked(AttrValue::from(content))
                        } else {
                            html! { post.summary.clone() }
                        }
                    }) {
                        { text }
                    } else {
                        <span class="placeholder col-7"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-4"></span> { " " }
                        <span class="placeholder col-6"></span> { " " }
                        <span class="placeholder col-6"></span> { " " }
                        <span class="placeholder col-5"></span> { " " }
                        <span class="placeholder col-5"></span> { " " }
                        <span class="placeholder col-3"></span> { " " }
                        if is_full {
                            <span class="placeholder col-7"></span> { " " }
                            <span class="placeholder col-4"></span> { " " }
                            <span class="placeholder col-4"></span> { " " }
                            <span class="placeholder col-6"></span> { " " }
                            <span class="placeholder col-6"></span> { " " }
                            <span class="placeholder col-5"></span> { " " }
                            <span class="placeholder col-5"></span> { " " }
                            <span class="placeholder col-3"></span> { " " }
                        }
                    }
                </p>
            </div>
        </>
    };
    html! {
        <div class="card mb-3">
            if let (Some(post), true) = (post.as_ref(), link_to) {
                <Link<Route, Post>
                    classes="text-decoration-none"
                    to={ Route::Post { slug: post.slug.clone(), id: post.id } }
                    state={ Some(post.clone()) }
                >
                    { main_content }
                </Link<Route, Post>>
            } else {
                { main_content }
            }
            <div class="card-footer">
                <div class="row align-items-center">
                    <div class="col placeholder-glow">
                        if let Some(tags) = post
                            .as_ref()
                            .map(|p| p.tags_string())
                        {
                            { tags }
                        } else {
                            <span class="placeholder col-6"></span>
                        }
                    </div>
                    <div class="col-6 text-end placeholder-glow">
                        if let Some(post) = post.as_ref() {
                            <Link<Route>
                                classes="title is-block col-6 text-decoration-none"
                                to={ Route::Author {
                                    slug: post.short_author.slug.clone()
                                } }
                            >
                                { &post.short_author.slug }
                            </Link<Route>>
                            if let LoggedUserState::ActiveAndLoaded { token: _, author } = logged_user_context.state.clone() {
                                if author.base.slug == post.short_author.slug {
                                    { " | " }
                                    <Link<Route, Post>
                                        classes="title is-block col-6 text-decoration-none"
                                        to={ Route::EditPost { id: post.id } }
                                        state={ Some(post.clone()) }
                                    >
                                        <PencilSquareImg />
                                    </Link<Route, Post>>
                                }
                            }
                        } else {
                            <span class="placeholder col-4"></span>
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
