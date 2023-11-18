use noneifempty::*;
use yew::prelude::*;

use crate::utils::head::*;

#[derive(PartialEq, Properties, Clone)]
pub struct MetaProps {
    #[prop_or_default]
    pub r#type: String,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub description: String,
    #[prop_or_default]
    pub keywords: String,
    #[prop_or_default]
    pub image: String,
    #[prop_or_default]
    pub image_width: String,
    #[prop_or_default]
    pub image_height: String,
    #[prop_or_default]
    pub noindex: bool,
}

#[function_component(Meta)]
pub fn meta(props: &MetaProps) -> Html {
    let MetaProps {
        r#type,
        title,
        description,
        keywords,
        image,
        image_width,
        image_height,
        noindex,
    } = props.clone();
    let r#type = r#type.none_if_empty().unwrap_or("website".to_owned());
    let short_title = title.clone();
    let title = title
        .none_if_empty()
        .map(|t| format!("{} - {}", t, crate::TITLE))
        .unwrap_or(crate::TITLE.to_owned());
    let description = description
        .none_if_empty()
        .unwrap_or(crate::DESCRIPTION.to_owned());
    let keywords = keywords
        .none_if_empty()
        .unwrap_or(crate::KEYWORDS.to_owned());
    let robots = if noindex { "noindex" } else { "all" }.to_owned();
    set_meta(MetaTag::OpenGraph(OpenGraph::Type), &r#type);
    set_title(&title);
    set_meta(MetaTag::OpenGraph(OpenGraph::Title), &short_title);
    set_meta(MetaTag::Description, &description);
    set_meta(MetaTag::OpenGraph(OpenGraph::Description), &description);
    set_meta(MetaTag::Keywords, &keywords);
    set_meta(MetaTag::OpenGraph(OpenGraph::Image), &image);
    set_meta(MetaTag::OpenGraph(OpenGraph::ImageWidth), &image_width);
    set_meta(MetaTag::OpenGraph(OpenGraph::ImageHeight), &image_height);
    set_meta(MetaTag::Robots, &robots);
    set_meta(
        MetaTag::OpenGraph(OpenGraph::SiteName),
        &crate::TITLE.to_owned(),
    );
    html! {
        <>
            <script data-page-content="type" type="text/plain"> { r#type } </script>
            <script data-page-content="title" type="text/plain"> { title.clone() } </script>
            <script data-page-content="description" type="text/plain"> { description } </script>
            <script data-page-content="keywords" type="text/plain"> { keywords } </script>
            <script data-page-content="image" type="text/plain"> { image } </script>
            <script data-page-content="image_width" type="text/plain"> { image_width } </script>
            <script data-page-content="image_height" type="text/plain"> { image_height } </script>
            <script data-page-content="robots" type="text/plain"> { robots } </script>
            <script data-page-content="site_name" type="text/plain"> { crate::TITLE } </script>
        </>
    }
}
