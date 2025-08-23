//! Functions, traits, methods, and types to improve security *and* simplicity.

#![forbid(missing_docs)]
#![forbid(unused_must_use)]
#![forbid(clippy::all)]
#![forbid(clippy::nursery)]
#![forbid(clippy::pedantic)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![no_std]

// #[cfg(feature = "derive")]
// pub use briny_derive::{Pod, SafeMemory};

/// A general error for anything that goes wrong internally.
///
/// # Examples
///
/// Common examples include:
///
/// - Raw data is invalid
/// - Memory is unaligned
/// - Types have incorrect sizes
#[derive(Debug)]
pub struct BrinyError;

impl core::fmt::Display for BrinyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}
impl core::error::Error for BrinyError {}

impl SafeMemory for BrinyError {}
unsafe impl crate::raw::Pod for BrinyError {}

pub mod pack;
pub mod raw;
pub use raw::*;
pub mod valid;

/// A simple marker trait which tells the program that a type is safe to operate on in most cases.
///
/// For stricter requirements and better rewards, try `Pod`.
///
/// # Safety
///
/// Memory can't be guaranteed safe without explicit checks, hence the constant `_ASSERTIONS`.
// Procedurally deriving this trait verifies that it's safe, so it's reccomended to use this:
//
// ```rust
// use briny::SafeMemory;
//
// #[derive(SafeMemory)]
// #[repr(C)]
// pub struct Foo {
//     a: u16,
//     b: u32,
//     c: u64,
// }
// ```
///
// Rather than manually implementing it:
//
// ```rust
// pub struct Foo {
//     a: u16,
//     b: u32,
//     c: u64,
// }
//
// impl briny::SafeMemory for Foo {}
// ```
//
// Since `SafeMemory` reccomends predictable layout (like `#[repr(C)]`), the former is  Deriving the trait catches implementations and raises helpful compiler errors, allowing you to fix them.
pub trait SafeMemory {
    /// Optional `const` assertions to ensure complete safety.
    ///
    /// If they fail, a compiler error will be thrown.
    const _ASSERTIONS: () = ();
}

impl SafeMemory for u8 {}
impl SafeMemory for u16 {}
impl SafeMemory for u32 {}
impl SafeMemory for u64 {}
impl SafeMemory for usize {}
impl SafeMemory for u128 {}
impl SafeMemory for i8 {}
impl SafeMemory for i16 {}
impl SafeMemory for i32 {}
impl SafeMemory for i64 {}
impl SafeMemory for isize {}
impl SafeMemory for i128 {}
impl SafeMemory for f32 {}
impl SafeMemory for f64 {}
impl<T: SafeMemory, const N: usize> SafeMemory for [T; N] {}
impl<T: SafeMemory> SafeMemory for core::mem::MaybeUninit<T> {}

// #[cfg(feature = "derive")]
// pub use briny_derive::SafeMemory;

// #[cfg(feature = "derive")]
// pub use briny_derive::Pod;
