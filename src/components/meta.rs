use yew::prelude::*;

use crate::utils::{head::*, not_empty};

#[derive(PartialEq, Properties, Clone)]
pub struct MetaProps {
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub description: String,
    #[prop_or_default]
    pub keywords: String,
    #[prop_or_default]
    pub noindex: bool,
}

#[function_component(Meta)]
pub fn meta(props: &MetaProps) -> Html {
    let MetaProps {
        title,
        description,
        keywords,
        noindex,
    } = props.clone();
    let title = not_empty(Some(title))
        .map(|t| format!("{} - {}", t, crate::TITLE))
        .unwrap_or(crate::TITLE.to_owned());
    let description = not_empty(Some(description)).unwrap_or(crate::DESCRIPTION.to_owned());
    let keywords = not_empty(Some(keywords)).unwrap_or(crate::KEYWORDS.to_owned());
    let robots = if noindex { "noindex" } else { "all" }.to_string();
    set_title(&title);
    set_meta(MetaTag::Description, &description);
    set_meta(MetaTag::Keywords, &keywords);
    set_meta(MetaTag::Robots, &robots);
    html! {
        <>
            <script data-page-content="title" type="text/plain"> { title } </script>
            <script data-page-content="description" type="text/plain"> { description } </script>
            <script data-page-content="keywords" type="text/plain"> { keywords } </script>
            <script data-page-content="robots" type="text/plain"> { robots } </script>
        </>
    }
}
