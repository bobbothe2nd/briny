//! Raw byte buffer abstraction for fixed-size binary data.
//!
//! This module provides:
//! - [`ByteBuf<T, N>`]: a generic wrapper around `[u8; N]` for safely handling raw bytes
//! - [`Raw<N>`]: a trait for parsing and serializing types to/from fixed-size byte arrays

use core::fmt::Debug;
use crate::trust::ValidationError;

/// A fixed-size byte buffer associated with a raw-parsable type `T`.
///
/// This wrapper enables safe and validated handling of raw binary data
/// that will eventually be interpreted as a well-defined type.
///
/// # Type Parameters
///
/// - `T`: A type implementing [`Raw<N>`] and optionally [`Validate`](crate::trust::Validate)
/// - `N`: The number of bytes in the buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ByteBuf<T, const N: usize> {
    buf: [u8; N],
    _phantom: core::marker::PhantomData<T>,
}

use core::str::FromStr;

impl<T, const N: usize> FromStr for ByteBuf<T, N>
where
    T: Raw<N> + Clone,
{
    type Err = ValidationError;

    /// Constructs a `ByteBuf<T, N>` from a UTF-8 `&str`.
    ///
    /// Fails if the string is longer than `N` bytes.
    ///
    /// Remaining bytes are zero-padded.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > N {
            return Err(ValidationError);
        }

        let mut buf = [0u8; N];
        buf[..s.len()].copy_from_slice(s.as_bytes());

        Ok(Self {
            buf,
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<T, const N: usize> ByteBuf<T, N> {
    /// Construct a new `ByteBuf<T, N>` from a raw `[u8; N]` array.
    pub fn new(buf: [u8; N]) -> Self {
        Self {
            buf,
            _phantom: core::marker::PhantomData,
        }
    }

    /// Returns a reference to the internal `[u8; N]` buffer.
    pub fn as_bytes(&self) -> &[u8; N] {
        &self.buf
    }

    /// Consumes the `ByteBuf` and returns the owned `[u8; N]` buffer.
    pub fn into_bytes(self) -> [u8; N] {
        self.buf
    }

    /// Returns the length of the byte buffer (always `N`).
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if the buffer contains all zeroes.
    pub fn is_empty(&self) -> bool {
        self.buf.iter().all(|&b| b == 0)
    }

    /// Attempts to parse the byte buffer into a `T` by calling [`Raw::from_bytes`].
    ///
    /// This may perform internal validation and can fail with [`ValidationError`].
    pub fn parse(self) -> Result<T, ValidationError>
    where
        T: Raw<N>,
    {
        T::from_bytes(self.buf)
    }
}

/// A trait for types that can be losslessly converted to/from a fixed-size byte buffer.
///
/// Typically used for binary-encoded data like protocol fields, fixed-length strings,
/// or hardware representations.
pub trait Raw<const N: usize>: Sized {
    /// Attempt to parse a fixed-size buffer into `Self`.
    ///
    /// Should return [`ValidationError`] if the byte contents are invalid.
    fn from_bytes(bytes: [u8; N]) -> Result<Self, ValidationError>;

    /// Convert this value into a fixed-size byte buffer.
    fn to_bytes(&self) -> [u8; N];
}

impl Raw<4> for u32 {
    fn from_bytes(bytes: [u8; 4]) -> Result<Self, ValidationError> {
        Ok(u32::from_le_bytes(bytes))
    }

    fn to_bytes(&self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
