#![allow(dead_code, unused_variables, unused_mut)]

mod app;
mod components;
mod content;
pub mod lang;
mod pages;
mod route;
mod utils;

#[cfg(feature = "client")]
#[macro_use]
extern crate async_trait;

pub use app::*;
pub use route::*;
pub use utils::AppContent;

#[cfg(all(feature = "client", feature = "yandex"))]
const YANDEX_CLIENT_ID: &'static str = std::env!("YANDEX_CLIENT_ID"); // ee156ec6ee994a748e724f604db8e305
#[cfg(feature = "client")]
const API_URL: &'static str = std::env!("API_URL"); // http://127.0.0.1:3000/api
#[cfg(feature = "telegram")]
const TELEGRAM_BOT_LOGIN: &'static str = std::env!("TELEGRAM_BOT_LOGIN"); // AnyBlogBot
const TITLE: &'static str = std::env!("TITLE"); // BLOG
const DESCRIPTION: &'static str = std::env!("DESCRIPTION"); // BLOG DESCRIPTION
const KEYWORDS: &'static str = std::env!("KEYWORDS"); // BLOG, KEYWORDS
const ACCORDION_JSON: &'static str = std::env!("ACCORDION_JSON"); // [{"title":"О блоге","body":"<strong>Ты ошибка эволюции.</strong><br/>А блог этот про хороших людей в плохое время."},{"title":"Контент","body":"Привет!"}]
