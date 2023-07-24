mod app;
mod components;
mod content;
mod pages;
mod utils;

#[macro_use]
extern crate async_trait;

pub use app::*;

pub const TITLE: &'static str = "BLOG";
pub const DESCRIPTION: &'static str = "BLOG DESCRIPTION";
pub const KEYWORDS: &'static str = "BLOG, KEYWORDS";
pub const API_URL: &'static str = "http://127.0.0.1:3000";
