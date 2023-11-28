use yew::prelude::*;

const EMPTY_IMAGE: &'static str =
    "data:image/gif;base64,R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==";

#[derive(PartialEq, Properties, Clone)]
pub struct OptionalImageProps {
    #[prop_or_default]
    pub alt: Option<String>,
    #[prop_or_default]
    pub image: Option<String>,
    #[prop_or_default]
    pub fallback_image: Option<String>,
    #[prop_or_default]
    pub without_empty: bool,
}

#[function_component(OptionalImage)]
pub fn optional_image(props: &OptionalImageProps) -> Html {
    let OptionalImageProps {
        alt,
        image,
        fallback_image,
        without_empty,
    } = props.clone();

    let image_ref = use_node_ref();

    #[cfg(feature = "client")]
    let error_handler = {
        let fallback_image = fallback_image.clone();
        let image_ref = image_ref.clone();
        Callback::from(move |_: Event| {
            let image_ref = image_ref.cast::<web_sys::HtmlImageElement>().unwrap();
            if !(image_ref.natural_width() <= 1
                && image_ref.natural_height() <= 1
                && image_ref.src() != EMPTY_IMAGE)
            {
                return;
            }
            if let Some(fallback_image) = fallback_image.clone().filter(|f| f != &image_ref.src()) {
                image_ref.set_src(fallback_image.as_str());
            } else if !without_empty {
                image_ref.set_src(EMPTY_IMAGE);
            }
        })
    };
    #[cfg(not(feature = "client"))]
    let error_handler = Callback::from(|_| {});

    html! {
        <div class="optional-image-container">
            <img
                { alt }
                ref={ image_ref }
                src={
                    image
                        .or(fallback_image)
                        .unwrap_or(EMPTY_IMAGE.to_string())
                }
                onload={ error_handler.clone() }
                onerror={ error_handler }
                loading="lazy"
            />
        </div>
    }
}
