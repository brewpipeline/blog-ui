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

    let alt = author.as_ref().map(|a| a.slug.clone());
    let image = author
        .as_ref()
        .map(|a| a.image_url.clone())
        .flatten()
        .map(|u| image_url_formatter(ImageType::Small, &u));
    let fallback_image = author
        .as_ref()
        .map(|a| format!("https://api.dicebear.com/7.x/thumbs/svg?seed={}", a.slug));

    html! {
        <OptionalImage
            { alt }
            { image }
            { fallback_image }
        />
    }
}
