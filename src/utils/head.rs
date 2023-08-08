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

#[cfg(feature = "client")]
fn meta_element(meta_type: MetaTag) -> Element {
    html_document()
        .query_selector(format!("meta[name=\"{name}\"]", name = meta_type.to_string()).as_str())
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
