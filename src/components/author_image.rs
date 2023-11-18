use yew::prelude::*;

use crate::components::optional_image::*;
use crate::content::*;
use crate::utils::*;

#[derive(PartialEq, Properties, Clone)]
pub struct AuthorImageProps {
    pub author: Option<Author>,
}

#[function_component(AuthorImage)]
pub fn author_image(props: &AuthorImageProps) -> Html {
    let AuthorImageProps { author } = props.clone();

    html! {
        <OptionalImage
            alt={ author.as_ref().map(|a| a.slug.clone()) }
            image={ author.as_ref().map(|a| a.image_url.clone()).flatten() }
            fallback_image={ author.as_ref().map(|a| profile_image(&a.slug)) }
        />
    }
}
