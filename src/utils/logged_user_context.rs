use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_cookies::CookieOptions;
use yew::{Reducible, UseReducerHandle};

use crate::content::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoggedUserState {
    None,
    InProgress(AuthParams),
    Error(String),
    Active { token: String },
    ActiveAndLoaded { token: String, author: Author },
}

impl LoggedUserState {
    pub fn token(&self) -> Option<&String> {
        match self {
            LoggedUserState::None | LoggedUserState::Error(_) | LoggedUserState::InProgress(_) => {
                None
            }
            LoggedUserState::Active { token }
            | LoggedUserState::ActiveAndLoaded { token, author: _ } => Some(token),
        }
    }
    pub fn action_available(&self) -> bool {
        match self {
            LoggedUserState::None | LoggedUserState::Error(_) => true,
            LoggedUserState::InProgress(_)
            | LoggedUserState::Active { token: _ }
            | LoggedUserState::ActiveAndLoaded {
                token: _,
                author: _,
            } => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoggedUser {
    pub state: LoggedUserState,
}

impl LoggedUser {
    #[cfg(target_arch = "wasm32")]
    fn load_token() -> Option<String> {
        Some(wasm_cookies::get("Token")?.ok()?)
    }

    #[cfg(target_arch = "wasm32")]
    fn save_token(token: Option<&String>) -> Option<()> {
        if let Some(token) = &token {
            wasm_cookies::set("Token", &token, &CookieOptions::default());
        } else {
            wasm_cookies::delete("Token")
        }
        Some(())
    }
}

impl Default for LoggedUser {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let token = Self::load_token();
        #[cfg(not(target_arch = "wasm32"))]
        let token = None;
        Self {
            state: match token {
                Some(token) => LoggedUserState::Active { token },
                None => LoggedUserState::None,
            },
        }
    }
}

impl Reducible for LoggedUser {
    type Action = LoggedUserState;
    fn reduce(self: Rc<Self>, new_state: LoggedUserState) -> Rc<Self> {
        #[cfg(target_arch = "wasm32")]
        Self::save_token(new_state.token());
        LoggedUser { state: new_state }.into()
    }
}

pub type LoggedUserContext = UseReducerHandle<LoggedUser>;
