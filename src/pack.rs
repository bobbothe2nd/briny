//! Trust-aware traits and buffer wrappers for secure binary serialization.
//!
//! This module provides:
//! - [`Pack`] and [`Unpack`]: traits for encoding and decoding structured types
//! - [`PackRef`] and [`UnpackBuf`]: slice wrappers used during serialization
//!
//! All decoded data is *validated at the boundary* and returned as [`TrustedData<T>`].
//!
//! # Trust model
//! - Packing assumes the data is valid and trusted
//! - Unpacking always performs validation before returning a usable value

use crate::trust::{ValidationError, TrustedData};

/// Trait for types that can be serialized into a binary format.
///
/// Implementations must ensure the resulting encoding is correct and canonical.
///
/// # Errors
/// Returns [`ValidationError`] if encoding fails (e.g., insufficient space or inconsistent data).
pub trait Pack {
    fn pack(&self, out: PackRef<'_>) -> Result<(), ValidationError>;
}

/// Trait for types that can be deserialized from raw bytes *and validated*.
///
/// This ensures that all unpacked values are trusted and structurally sound.
///
/// # Returns
/// A [`TrustedData<T>`] on success, or [`ValidationError`] if the bytes are invalid.
pub trait Unpack: Sized {
    fn unpack_and_validate(input: UnpackBuf<'_>) -> Result<TrustedData<'_, Self>, ValidationError>;
}

/// A mutable slice wrapper used to write serialized binary data.
///
/// Used as the output buffer for [`Pack`] implementations.
#[derive(Debug)]
pub struct PackRef<'a> {
    buf: &'a mut [u8],
}

impl<'a> PackRef<'a> {
    /// Create a new [`PackRef`] from a mutable byte slice.
    #[must_use]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf }
    }

    /// Get a mutable reference to the inner buffer.
    pub fn ref_mut(&mut self) -> &mut [u8] {
        self.buf
    }

    /// Length of the writable buffer (in bytes).
    #[must_use]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

/// An immutable slice wrapper used to read and validate structured data.
///
/// Used as the input buffer for [`Unpack`] implementations.
#[derive(Debug)]
pub struct UnpackBuf<'a> {
    buf: &'a [u8],
}

impl<'a> UnpackBuf<'a> {
    /// Create a new [`UnpackBuf`] from a read-only byte slice.
    #[must_use]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }

    /// Get a reference to the inner byte slice.
    pub fn as_slice(&self) -> &[u8] {
        self.buf
    }

    /// Length of the readable buffer (in bytes).
    #[must_use]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}
