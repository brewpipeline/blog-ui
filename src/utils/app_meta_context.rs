use std::collections::HashSet;
use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

use super::head::{self, MetaTag};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppMeta {
    pub title: String,
    pub description: String,
    pub keywords: String,
}

impl Default for AppMeta {
    fn default() -> Self {
        Self {
            title: crate::TITLE.to_owned(),
            description: crate::DESCRIPTION.to_owned(),
            keywords: crate::KEYWORDS.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum AppMetaAction {
    Title(String),
    Description(String),
    Keywords(String),
}

impl Reducible for AppMeta {
    type Action = HashSet<AppMetaAction>;
    fn reduce(self: Rc<Self>, actions: HashSet<AppMetaAction>) -> Rc<Self> {
        head::set_title(&crate::TITLE.to_owned());
        head::set_meta(MetaTag::Description, &crate::DESCRIPTION.to_owned());
        head::set_meta(MetaTag::Keywords, &crate::KEYWORDS.to_owned());
        let mut app_meta = AppMeta::default();
        for action in actions {
            match action {
                AppMetaAction::Title(title) => {
                    let title = format!("{} - {}", title, crate::TITLE);
                    head::set_title(&title);
                    app_meta.title = title
                }
                AppMetaAction::Description(description) => {
                    head::set_meta(MetaTag::Description, &description);
                    app_meta.description = description
                }
                AppMetaAction::Keywords(keywords) => {
                    head::set_meta(MetaTag::Keywords, &keywords);
                    app_meta.keywords = keywords
                }
            }
        }
        app_meta.into()
    }
}

pub type AppMetaContext = UseReducerHandle<AppMeta>;
