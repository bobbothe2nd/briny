//! Core primitives, byte casting, and safe wrappers for handling raw bytes.

#[cfg(feature = "alloc")]
pub mod alloc;

pub mod cast;

mod nonzero_copy;

pub use nonzero_copy::MaybeNullCopy;

pub mod nonzero;

pub use nonzero::MaybeNull;

use crate::traits::ImplicitPod;

/// Transparent wrapper for `ImplicitPod` to `ExplicitPod` traits
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PodCell<T: ImplicitPod>(pub T);

unsafe impl<T: ImplicitPod> stable_auto::Auto for PodCell<T> {
    type Fields = stable_auto::Leaf<T>;
}
