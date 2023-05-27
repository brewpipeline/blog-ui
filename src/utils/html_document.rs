use web_sys::{HtmlDocument, Element};
use wasm_bindgen::JsCast;
use gloo::utils::document;

const TITLE: &'static str = "BLOG";
const DESCRIPTION: &'static str = "BLOG DESCRIPTION";
const KEYWORDS: &'static str = "BLOG, KEYWORDS";

pub fn html_document() -> HtmlDocument {
    document().unchecked_into::<HtmlDocument>()
}

#[derive(Clone, Copy)]
pub enum MetaTag {
    Description,
    Keywords,
}

impl std::fmt::Display for MetaTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MetaTag::Description => write!(f, "description"),
            MetaTag::Keywords => write!(f, "keywords"),
        }
    }
}

fn meta_element(meta_type: MetaTag) -> Element {
    html_document()
        .query_selector(format!("meta[name=\"{name}\"]", name = meta_type.to_string()).as_str())
        .ok()
        .flatten()
        .unwrap()
}

/*
pub fn meta(meta_type: MetaTag) -> String {
    meta_element(meta_type)
        .get_attribute("content")
        .unwrap()
}
*/

pub fn set_meta(meta_type: MetaTag, value: String) {
    meta_element(meta_type)
        .set_attribute("content", value.as_str())
        .unwrap()
}

/*
pub fn set_prefix_meta(meta_type: MetaTag, value: String) {
    set_meta(meta_type, value + meta(meta_type).as_str())
}
*/

/*
pub fn title() -> String {
    html_document()
        .title()
}
*/

pub fn set_title(value: String) {
    html_document()
        .set_title(value.as_str())
}

/*
pub fn set_prefix_title(value: String) {
    set_title(value + title().as_str())
}
*/

pub fn set_prefix_default_title(value: String) {
    set_title(format!("{} - {}", value, TITLE))
}

pub fn reset_title_and_meta() {
    set_title(TITLE.to_string());
    set_meta(MetaTag::Description, DESCRIPTION.to_string());
    set_meta(MetaTag::Keywords, KEYWORDS.to_string());
}