use yew::prelude::*;

use crate::components::optional_image::*;
use crate::content::*;

#[derive(PartialEq, Properties, Clone)]
pub struct AuthorImageProps {
    pub author: Option<Author>,
}

#[function_component(AuthorImage)]
pub fn author_image(props: &AuthorImageProps) -> Html {
    let AuthorImageProps { author } = props.clone();

    let alt = author.as_ref().map(|a| a.slug.clone());
    let image = author.as_ref().map(|a| a.image_url.clone()).flatten();
    let fallback_image = author
        .as_ref()
        .map(|a| format!("https://api.dicebear.com/7.x/bottts/svg?seed={}&baseColor=f2f2f2&eyes=dizzy,eva,frame1,frame2,happy,hearts,robocop,round,roundFrame01,roundFrame02,sensor,shade01", a.slug));

    html! {
        <OptionalImage
            { alt }
            { image }
            { fallback_image }
        />
    }
}
