//! Crate Prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic wrapper tuple struct for newtype pattern
pub struct Wrapper<T>(pub T);

// Personal preference.
pub use std::format as f;
