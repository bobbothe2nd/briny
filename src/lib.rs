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

pub mod align;
pub mod raw;
pub mod traits;

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
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct BrinyError {
    code: u8,
}

impl core::ops::BitOr<Self> for BrinyError {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl BrinyError {
    const RESERVED_CODE: u8 = 0b0000_0000;

    const SIZE_BOUND_FAILURE_CODE: u8 = 0b0000_0001;

    const UNALIGNED_ACCESS_CODE: u8 = 0b0000_0010;

    /// A reserved code `0` that does not work as a regular error.
    pub const RESERVED: Self = Self::new(Self::RESERVED_CODE);

    /// An error indicating that two sizes are incompatible.
    pub const SIZE_BOUND_FAILURE: Self = Self::new(Self::SIZE_BOUND_FAILURE_CODE);

    /// An error indicating that an unaligned access is imminent.
    pub const UNALIGNED_ACCESS: Self = Self::new(Self::UNALIGNED_ACCESS_CODE);

    /// Constructs a new error from a code.
    #[inline]
    const fn new(code: u8) -> Self {
        Self { code }
    }

    /// Adds the two errors into a combination of multiple error codes.
    #[inline]
    #[must_use]
    pub const fn add(self, rhs: Self) -> Self {
        Self {
            code: (self.code | rhs.code),
        }
    }

    /// Checks if the error is even an error.
    ///
    /// This returns false if and only if `self` IS [`Self::RESERVED`].
    #[inline]
    #[must_use]
    pub const fn is_err(self) -> bool {
        self.code != 0
    }

    /// Checks if the error includes an unaligned access code.
    #[inline]
    #[must_use]
    pub const fn is_unaligned_access(self) -> bool {
        (self.code & Self::UNALIGNED_ACCESS_CODE) != 0
    }

    /// Checks if the error includes an size bound failure code.
    #[inline]
    #[must_use]
    pub const fn is_size_bound_failure(self) -> bool {
        (self.code & Self::SIZE_BOUND_FAILURE_CODE) != 0
    }
}

impl core::fmt::Display for BrinyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "BrinyError:code={}", self.code)
    }
}
impl core::error::Error for BrinyError {}

unsafe impl crate::traits::RawConvert for BrinyError {}
unsafe impl crate::traits::StableLayout for BrinyError {}
unsafe impl crate::traits::Pod for BrinyError {}
