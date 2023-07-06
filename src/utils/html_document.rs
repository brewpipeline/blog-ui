#[cfg(target_arch = "wasm32")]
use gloo::utils::document;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Element, HtmlDocument};

const TITLE: &'static str = "BLOG";
const DESCRIPTION: &'static str = "BLOG DESCRIPTION";
const KEYWORDS: &'static str = "BLOG, KEYWORDS";

#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
fn meta_element(meta_type: MetaTag) -> Element {
    html_document()
        .query_selector(format!("meta[name=\"{name}\"]", name = meta_type.to_string()).as_str())
        .ok()
        .flatten()
        .unwrap()
}

pub fn set_meta(meta_type: MetaTag, value: String) {
    #[cfg(target_arch = "wasm32")]
    meta_element(meta_type)
        .set_attribute("content", value.as_str())
        .unwrap()
}

pub fn set_title(value: String) {
    #[cfg(target_arch = "wasm32")]
    html_document().set_title(value.as_str())
}

pub fn set_prefix_default_title(value: String) {
    #[cfg(target_arch = "wasm32")]
    set_title(format!("{} - {}", value, TITLE))
}

pub fn reset_title_and_meta() {
    set_title(TITLE.to_string());
    set_meta(MetaTag::Description, DESCRIPTION.to_string());
    set_meta(MetaTag::Keywords, KEYWORDS.to_string());
}
