//! Core primitives, byte casting, and safe wrappers for handling raw bytes.

#[cfg(feature = "alloc")]
pub mod alloc;

pub mod cast;

mod nonzero_copy;

pub use nonzero_copy::MaybeNullCopy;

pub mod nonzero;

pub use nonzero::MaybeNull;
