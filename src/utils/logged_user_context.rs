use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_cookies::CookieOptions;
use yew::{Reducible, UseReducerHandle};

use crate::content::{AuthUser, LoginParams};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoggedUserState {
    None,
    InProgress(LoginParams),
    Error(String),
    Active(AuthUser),
}

impl LoggedUserState {
    pub fn action_available(&self) -> bool {
        match self {
            LoggedUserState::None | LoggedUserState::Error(_) => true,
            LoggedUserState::InProgress(_) | LoggedUserState::Active(_) => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoggedUser {
    pub state: LoggedUserState,
}

impl LoggedUser {
    #[cfg(target_arch = "wasm32")]
    fn load_auth_user() -> Option<AuthUser> {
        let cookie = wasm_cookies::get("AuthUser")?.ok()?;
        let auth_user: AuthUser = serde_json::from_str(cookie.as_str()).ok()?;
        Some(auth_user)
    }

    #[cfg(target_arch = "wasm32")]
    fn save_auth_user(auth_user: Option<&AuthUser>) -> Option<()> {
        if let Some(auth_user) = &auth_user {
            let auth_user_string = serde_json::to_string(auth_user).ok()?;
            wasm_cookies::set("AuthUser", &auth_user_string, &CookieOptions::default());
        } else {
            wasm_cookies::delete("AuthUser")
        }
        Some(())
    }
}

impl Default for LoggedUser {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        let auth_user = Self::load_auth_user();
        #[cfg(not(target_arch = "wasm32"))]
        let auth_user = None;
        Self {
            state: match auth_user {
                Some(auth_user) => LoggedUserState::Active(auth_user),
                None => LoggedUserState::None,
            },
        }
    }
}

impl Reducible for LoggedUser {
    type Action = LoggedUserState;
    fn reduce(self: Rc<Self>, new_state: LoggedUserState) -> Rc<Self> {
        #[cfg(target_arch = "wasm32")]
        match &new_state {
            LoggedUserState::None | LoggedUserState::InProgress(_) | LoggedUserState::Error(_) => {
                Self::save_auth_user(None);
            }
            LoggedUserState::Active(auth_user) => {
                Self::save_auth_user(Some(auth_user));
            }
        }
        LoggedUser { state: new_state }.into()
    }
}

pub type LoggedUserContext = UseReducerHandle<LoggedUser>;
