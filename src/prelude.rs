//! Crate prelude

// Re-export the crate Error.
pub use crate::error::Error;

// Re-export Metadata so callers can build in-memory entries.
pub use crate::Metadata;

// Alias Result to be the crate Result.
pub type Result<T> = core::result::Result<T, Error>;
