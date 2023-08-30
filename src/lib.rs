#![allow(dead_code, unused_variables, unused_mut)]

mod app;
mod components;
mod content;
mod pages;
mod route;
mod utils;

#[cfg(feature = "client")]
#[macro_use]
extern crate async_trait;

pub use app::*;
pub use route::*;
pub use utils::AppContent;

pub const API_URL: &'static str = "http://127.0.0.1:3000";

pub const TITLE: &'static str = "BLOG";
pub const DESCRIPTION: &'static str = "BLOG DESCRIPTION";
pub const KEYWORDS: &'static str = "BLOG, KEYWORDS";
pub const ACCORDION_ITEMS: [(&'static str, &'static str); 3] = [
    (
        "О блоге",
        "<strong>Ты ошибка эволюции.</strong><br/>А блог этот про хороших людей в плохое время.",
    ),
    ("Контент 1", "Привет!"),
    ("Контент 2", "Пока!"),
];
