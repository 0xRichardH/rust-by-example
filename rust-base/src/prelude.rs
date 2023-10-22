/// Re-export of the `Error` type
pub use crate::error::Error;

/// An alias for the `Result` type
pub type Result<T> = core::result::Result<T, Error>;

/// Generic wrapper
/// for external types to type From/TryFrom conversions
pub struct W<T>(pub T);
