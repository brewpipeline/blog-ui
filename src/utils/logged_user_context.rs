use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

use crate::content::*;

#[cfg(all(feature = "client", target_arch = "wasm32"))]
fn load_token() -> Option<String> {
    Some(wasm_cookies::get("Token")?.ok()?)
}

#[cfg(all(feature = "client", target_arch = "wasm32"))]
fn save_token(token: Option<&String>) -> Option<()> {
    let config = wasm_cookies::CookieOptions::default().with_path("/");
    #[cfg(not(debug_assertions))]
    let config = config
        .secure()
        .with_same_site(wasm_cookies::SameSite::Strict);
    if let Some(token) = &token {
        wasm_cookies::set(
            "Token",
            &token,
            &config.expires_after(std::time::Duration::new(60 * 60 * 24 * 30, 0)),
        );
    } else {
        wasm_cookies::set("Token", &"", &config.expires_at_timestamp(0));
    }
    Some(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum LoggedUserInnerState {
    NotInited,
    Common(LoggedUserState),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InProgressStateType {
    Default(LoginQuestion),
    #[cfg(feature = "yandex")]
    Yandex(LoginYandexQuestion),
    #[cfg(feature = "telegram")]
    Telegram(LoginTelegramQuestion),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoggedUserState {
    LoggedOut,
    InProgress(InProgressStateType),
    Error(String),
    Active { token: String },
    ActiveAndLoaded { token: String, author: Author },
}

impl LoggedUserState {
    pub fn token(&self) -> Option<&String> {
        match self {
            LoggedUserState::LoggedOut
            | LoggedUserState::Error(_)
            | LoggedUserState::InProgress(_) => None,
            LoggedUserState::Active { token }
            | LoggedUserState::ActiveAndLoaded { token, author: _ } => Some(token),
        }
    }
    pub fn author(&self) -> Option<&Author> {
        match self {
            LoggedUserState::LoggedOut
            | LoggedUserState::Error(_)
            | LoggedUserState::InProgress(_)
            | LoggedUserState::Active { token: _ } => None,
            LoggedUserState::ActiveAndLoaded { token: _, author } => Some(author),
        }
    }
    pub fn action_available(&self) -> bool {
        match self {
            LoggedUserState::LoggedOut | LoggedUserState::Error(_) => true,
            LoggedUserState::InProgress(_)
            | LoggedUserState::Active { token: _ }
            | LoggedUserState::ActiveAndLoaded {
                token: _,
                author: _,
            } => false,
        }
    }
    pub fn load() -> Self {
        #[cfg(all(feature = "client", target_arch = "wasm32"))]
        match load_token() {
            Some(token) => LoggedUserState::Active { token },
            None => LoggedUserState::LoggedOut,
        }
        #[cfg(any(not(feature = "client"), not(target_arch = "wasm32")))]
        return LoggedUserState::LoggedOut;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoggedUser {
    inner_state: LoggedUserInnerState,
}

impl Default for LoggedUser {
    fn default() -> Self {
        Self {
            inner_state: LoggedUserInnerState::NotInited,
        }
    }
}

impl LoggedUser {
    pub fn is_not_inited(&self) -> bool {
        match &self.inner_state {
            LoggedUserInnerState::NotInited => true,
            LoggedUserInnerState::Common(_) => false,
        }
    }
    pub fn state(&self) -> &LoggedUserState {
        match &self.inner_state {
            LoggedUserInnerState::NotInited => {
                unreachable!("The method should not be used for not inited state!")
            }
            LoggedUserInnerState::Common(state) => state,
        }
    }
    pub fn token(&self) -> Option<&String> {
        self.state().token()
    }
    pub fn author(&self) -> Option<&Author> {
        self.state().author()
    }
    pub fn action_available(&self) -> bool {
        self.state().action_available()
    }
}

impl Reducible for LoggedUser {
    type Action = LoggedUserState;
    fn reduce(self: Rc<Self>, new_state: LoggedUserState) -> Rc<Self> {
        #[cfg(all(feature = "client", target_arch = "wasm32"))]
        save_token(new_state.token());
        LoggedUser {
            inner_state: LoggedUserInnerState::Common(new_state),
        }
        .into()
    }
}

pub type LoggedUserContext = UseReducerHandle<LoggedUser>;
