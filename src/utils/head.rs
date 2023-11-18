#[cfg(feature = "client")]
use gloo::utils::document;
#[cfg(feature = "client")]
use wasm_bindgen::JsCast;
#[cfg(feature = "client")]
use web_sys::{Element, HtmlDocument};

#[cfg(feature = "client")]
pub fn html_document() -> HtmlDocument {
    document().unchecked_into::<HtmlDocument>()
}

#[derive(Clone, Copy)]
pub enum OpenGraph {
    Title,
    Description,
    Type,
    Image,
    ImageWidth,
    ImageHeight,
    SiteName,
}

impl std::fmt::Display for OpenGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpenGraph::Title => write!(f, "og:title"),
            OpenGraph::Description => write!(f, "og:description"),
            OpenGraph::Type => write!(f, "og:type"),
            OpenGraph::Image => write!(f, "og:image"),
            OpenGraph::ImageWidth => write!(f, "og:image:width"),
            OpenGraph::ImageHeight => write!(f, "og:image:height"),
            OpenGraph::SiteName => write!(f, "og:site_name"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MetaTag {
    Description,
    Keywords,
    OpenGraph(OpenGraph),
    Robots,
}

impl MetaTag {
    fn key(&self) -> &'static str {
        match self {
            MetaTag::Description | MetaTag::Keywords | MetaTag::Robots => "name",
            MetaTag::OpenGraph(_) => "property",
        }
    }
}

impl std::fmt::Display for MetaTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MetaTag::Description => write!(f, "description"),
            MetaTag::Keywords => write!(f, "keywords"),
            MetaTag::OpenGraph(open_graph) => open_graph.fmt(f),
            MetaTag::Robots => write!(f, "robots"),
        }
    }
}

#[cfg(feature = "client")]
fn meta_element(meta_type: MetaTag) -> Element {
    html_document()
        .query_selector(
            format!(
                "meta[{key}=\"{value}\"]",
                key = meta_type.key(),
                value = meta_type.to_string()
            )
            .as_str(),
        )
        .ok()
        .flatten()
        .unwrap()
}

pub fn set_meta(meta_type: MetaTag, value: &String) {
    #[cfg(feature = "client")]
    meta_element(meta_type)
        .set_attribute("content", value.as_str())
        .unwrap()
}

pub fn set_title(value: &String) {
    #[cfg(feature = "client")]
    html_document().set_title(value.as_str())
}
