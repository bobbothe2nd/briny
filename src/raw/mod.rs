//! Core primitives, byte casting, and (mostly) safe wrappers for handling raw bytes.
//!
//! # Safety
//!
//! This is the only module containing unsafe code, but it has a lot of it! The unsafe code in this module is for good reason though - It allows for casting between arbitrary types and making safe abstractions over unsafe ones.
//!
//! Traits like `Pod` provide useful methods to handle this data safely.

use crate::SafeMemory;

pub mod casting;
pub mod cell;
pub mod darc;
pub mod naarc;
pub mod ptr;

/// POD trait for *Plain Old Data*, allowing
///
/// # Safety
///
/// - `T` must have a stable layout (e.g. `#[repr(C)]`)
/// - All bit patterns of `T` must be valid.
/// - `T` must have no padding or tolerate uninitialized padding.
///
/// Violating these constraints is undefined behavior.
pub unsafe trait Pod: SafeMemory {
    /// Verifies that the given data is compatible with `Self`.
    #[must_use]
    fn is_valid_bitpattern(_data: &[u8]) -> bool {
        true // default: primitives, arrays, etc.
    }
}

unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for usize {}
unsafe impl Pod for u128 {}
unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for isize {}
unsafe impl Pod for i128 {}
unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}
unsafe impl<T: Copy + Pod, const N: usize> Pod for [T; N] {}
unsafe impl<T: Copy + Pod> Pod for core::mem::MaybeUninit<T> {}
