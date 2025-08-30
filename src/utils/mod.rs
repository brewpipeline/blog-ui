pub mod app_content_context;
pub mod author_slug_formatter;
pub mod date;
pub mod external;
#[cfg(feature = "client")]
pub mod get;
pub mod head;
pub mod image_url_formatter;
pub mod logged_user_context;
pub mod map_in_pattern;
#[cfg(feature = "client")]
pub mod scroll;
pub mod text;
pub mod use_load;
#[cfg(feature = "client")]
pub mod use_raf_state_eq;
#[cfg(feature = "client")]
pub mod use_window_width;

pub use app_content_context::*;
pub use author_slug_formatter::*;
pub use external::*;
#[cfg(feature = "client")]
pub use get::*;
pub use image_url_formatter::*;
pub use logged_user_context::*;
pub use map_in_pattern::*;
#[cfg(all(feature = "client", feature = "chatgpt"))]
pub use scroll::*;
pub use text::*;
pub use use_load::*;
#[cfg(feature = "client")]
pub use use_raf_state_eq::*;
#[cfg(feature = "client")]
pub use use_window_width::*;

#[cfg(not(feature = "client"))]
pub trait RequestableItem<P> {}

#[cfg(not(feature = "client"))]
impl<T, P> RequestableItem<P> for T {}
