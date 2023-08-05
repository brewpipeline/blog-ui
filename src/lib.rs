#![allow(dead_code, unused_variables, unused_mut)]

mod app;
mod components;
mod content;
mod pages;
mod utils;

#[cfg(feature = "client")]
#[macro_use]
extern crate async_trait;

pub use app::*;
pub use utils::AppContent;

pub const TITLE: &'static str = "BLOG";
pub const DESCRIPTION: &'static str = "BLOG DESCRIPTION";
pub const KEYWORDS: &'static str = "BLOG, KEYWORDS";
pub const API_URL: &'static str = "http://127.0.0.1:3000";
