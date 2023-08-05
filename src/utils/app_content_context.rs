use serde::{Deserialize, Serialize};
use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppContentContainer {
    pub is_used: bool,
    pub app_content: Option<AppContent>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppContent {
    pub r#type: String,
    pub value: String,
}

impl AppContent {
    pub fn json_decode<D: for<'de> Deserialize<'de>>(&self) -> Option<D> {
        if self.r#type != "application/json" {
            return None;
        }
        serde_json::from_str(self.value.as_str()).ok()
    }
    pub fn json_encode<D: Serialize>(data: &D) -> Option<AppContent> {
        serde_json::to_string(data).ok().map(|s| AppContent {
            r#type: "application/json".to_string(),
            value: s,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AppContentContainerAction {
    MarkAsUsed,
    NewContent(Option<AppContent>),
}

impl Reducible for AppContentContainer {
    type Action = AppContentContainerAction;
    fn reduce(self: Rc<Self>, new_state: AppContentContainerAction) -> Rc<Self> {
        let mut container = (*self).clone();
        match new_state {
            AppContentContainerAction::MarkAsUsed => container.is_used = true,
            AppContentContainerAction::NewContent(app_content) => {
                container.app_content = app_content
            }
        }
        container.into()
    }
}

pub type AppContentContext = UseReducerHandle<AppContentContainer>;
