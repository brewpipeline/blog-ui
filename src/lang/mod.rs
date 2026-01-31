#[cfg(all(feature = "lang_ru", feature = "lang_en"))]
compile_error!("Features lang_ru and lang_en are mutually exclusive");

#[cfg(not(any(feature = "lang_ru", feature = "lang_en")))]
compile_error!("One language feature must be enabled: lang_ru or lang_en");

#[cfg(feature = "lang_ru")]
mod ru;
#[cfg(feature = "lang_ru")]
pub use ru::*;

#[cfg(feature = "lang_en")]
mod en;
#[cfg(feature = "lang_en")]
pub use en::*;
