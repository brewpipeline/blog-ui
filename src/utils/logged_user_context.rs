use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

use crate::content::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoggedUserState {
    None,
    InProgress(LoginQuestion),
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
    #[cfg(all(feature = "client", target_arch = "wasm32"))]
    fn load_token() -> Option<String> {
        Some(wasm_cookies::get("Token")?.ok()?)
    }

    #[cfg(all(feature = "client", target_arch = "wasm32"))]
    fn save_token(token: Option<&String>) -> Option<()> {
        if let Some(token) = &token {
            wasm_cookies::set("Token", &token, &wasm_cookies::CookieOptions::default());
        } else {
            wasm_cookies::delete("Token")
        }
        Some(())
    }
}

impl Default for LoggedUser {
    fn default() -> Self {
        #[cfg(all(feature = "client", target_arch = "wasm32"))]
        return Self {
            state: match Self::load_token() {
                Some(token) => LoggedUserState::Active { token },
                None => LoggedUserState::None,
            },
        };
        #[cfg(any(not(feature = "client"), not(target_arch = "wasm32")))]
        return Self {
            state: LoggedUserState::None,
        };
    }
}

impl Reducible for LoggedUser {
    type Action = LoggedUserState;
    fn reduce(self: Rc<Self>, new_state: LoggedUserState) -> Rc<Self> {
        #[cfg(all(feature = "client", target_arch = "wasm32"))]
        Self::save_token(new_state.token());
        LoggedUser { state: new_state }.into()
    }
}

pub type LoggedUserContext = UseReducerHandle<LoggedUser>;
