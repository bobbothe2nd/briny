//! Functions, traits, methods, and types to improve memory integrity.

#![forbid(missing_docs)]
#![forbid(unused_must_use)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::unwrap_used)]
#![allow(clippy::inline_always)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod align;
pub mod bitpattern;
pub mod raw;
pub mod traits;

pub(crate) mod private {
    pub trait Private {}
}

/// Internally check if the type is private to `briny`.
#[doc(hidden)]
#[inline(always)]
pub const fn if_private<T: private::Private>(_val: *const T) {}

/// A general error for anything that goes wrong internally.
///
/// # Examples
///
/// Common examples include:
///
/// - Raw data is invalid
/// - Memory is unaligned
/// - Types have incorrect sizes
///
/// Compile errors are generally preferred.
///
/// To find out what specifically happened, match the code with each constant
/// descriptor.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum BrinyError {
    /// types of unequal sizes
    SizeBoundFailure,

    /// attempted creation of unaligned types
    UnalignedAccess,
}

impl private::Private for BrinyError {}

impl core::fmt::Display for BrinyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        match self {
            Self::SizeBoundFailure => f.write_str("assert size bound failed"),
            Self::UnalignedAccess => f.write_str("assert alignment failed"),
        }
    }
}
impl core::error::Error for BrinyError {}

unsafe impl crate::traits::StableLayout for BrinyError {}
