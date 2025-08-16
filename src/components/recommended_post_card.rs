use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::optional_image::*;
use crate::content::*;
use crate::utils::*;

use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct RecommendedPostCardProps {
    pub post: Post,
}

#[function_component(RecommendedPostCard)]
pub fn recommended_post_card(props: &RecommendedPostCardProps) -> Html {
    let RecommendedPostCardProps { post } = props.clone();

    let main_content = html! {
        <>
            <div 
                class="img-block bd-placeholder-img" 
                style="height:144px;width:100%;overflow:hidden;border-radius:calc(var(--bs-border-radius) - 1px) calc(var(--bs-border-radius) - 1px) 0 0"
            >
                <OptionalImage
                    alt={ post.title.clone() }
                    image={
                        post.image_url.clone().map(|u| image_url_formatter(ImageType::Medium, u))
                    }
                />
            </div>
            <div class="card-body">
                <h5 class="card-title placeholder-glow mb-0">
                    { post.title.clone() }
                </h5>
            </div>
        </>
    };

    html! {
        <div class="card mb-3">
            <Link<Route, (), Post>
                classes="text-decoration-none"
                to={ Route::Post { slug: post.slug.clone(), id: post.id } }
                state={ Some(post.clone()) }
            >
                { main_content }
            </Link<Route, (), Post>>
        </div>
    }
}
