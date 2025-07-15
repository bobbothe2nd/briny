//! Raw byte buffer abstraction for fixed-size binary data.
//!
//! This module provides:
//! - [`ByteBuf<T, N>`]: a generic wrapper around `[u8; N]` for safely handling raw bytes
//! - [`Raw<N>`]: a trait for parsing and serializing types to/from fixed-size byte arrays

use crate::prelude::UntrustedData;
use crate::trust::{Validate, ValidationError};
use core::{fmt::Debug, marker::PhantomData, str::FromStr};

#[must_use]
#[inline(always)]
fn map_ok<T, E, U>(res: Result<T, E>, f: fn(T) -> U) -> Result<U, E> {
    match res {
        Ok(v) => Ok(f(v)),
        Err(e) => Err(e),
    }
}

#[must_use]
#[inline(always)]
fn map_ok_option<T, U>(res: Result<T, ValidationError>, f: fn(T) -> U) -> Option<U> {
    match res {
        Ok(v) => Some(f(v)),
        Err(_) => None,
    }
}

#[must_use]
#[inline(always)]
fn and_then_ok<T, U, E, F: FnOnce(T) -> Result<U, E>>(res: Result<T, E>, f: F) -> Result<U, E> {
    match res {
        Ok(val) => f(val),
        Err(err) => Err(err),
    }
}

#[must_use]
#[inline(always)]
fn validation_to_unit<T>(res: Result<T, ValidationError>) -> Result<T, ()> {
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err(()),
    }
}

#[must_use]
#[inline(always)]
fn check_validation<T: Validate>(val: T) -> Result<T, ()> {
    if val.validate().is_ok() {
        Ok(val)
    } else {
        Err(())
    }
}

/// An iterator-like structure over byte slices that yields chunks of size `CHUNK`.
///
/// # Type Parameters
/// - `'a`: Lifetime of the underlying byte slice.
/// - `T`: Phantom type parameter, typically representing the element type logically associated with the chunked data.
/// - `CHUNK`: Constant generic representing the fixed size of each chunk.
///
/// # Fields
/// - `buf`: Reference to the underlying byte slice.
/// - `_phantom`: PhantomData to associate the generic type `T` without storing actual values.
pub struct Chunks<'a, T, const CHUNK: usize> {
    buf: &'a [u8],
    _phantom: PhantomData<T>,
}

impl<'a, T: Raw<CHUNK>, const CHUNK: usize> Iterator for Chunks<'a, T, CHUNK> {
    type Item = UntrustedData<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < CHUNK {
            return None;
        }
        let (head, tail) = self.buf.split_at(CHUNK);
        let mut tmp = [0u8; CHUNK];
        tmp.copy_from_slice(head);
        self.buf = tail;

        match T::from_bytes(tmp) {
            Ok(val) => Some(UntrustedData::new(val)),
            Err(_) => None,
        }
    }
}

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
    _phantom: PhantomData<T>,
}

impl<T, const N: usize> ByteBuf<T, N> {
    #[must_use]
    /// Construct from a `[u8; N]`
    #[inline(always)]
    pub const fn new(buf: [u8; N]) -> Self {
        Self {
            buf,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    /// Returns the inner bytes
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8; N] {
        &self.buf
    }

    #[must_use]
    /// Consumes the buffer and returns the bytes
    #[inline(always)]
    pub const fn into_bytes(self) -> [u8; N] {
        self.buf
    }

    #[must_use]
    /// Always returns `N`
    #[inline(always)]
    pub const fn len(&self) -> usize {
        N
    }

    #[must_use]
    /// Returns `true` if all bytes are zero
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        // rename to `is_zero`
        // make new `is_empty` for when `len()` returns `0`
        self.buf.iter().all(|&b| b == 0)
    }

    #[must_use]
    /// View buffer as `UntrustedData<T>`
    #[inline(always)]
    pub fn as_untrusted(&self) -> Result<UntrustedData<T>, ValidationError>
    where
        T: Raw<N>,
    {
        map_ok(T::from_bytes(self.buf), UntrustedData::new)
    }

    #[must_use]
    /// Tries to parse and validate into a trusted `T`
    #[inline(always)]
    pub fn try_unpack(&self) -> Result<T, ()>
    where
        T: Raw<N> + Validate,
    {
        let parsed = validation_to_unit(T::from_bytes(self.buf));
        and_then_ok(parsed, check_validation::<T>)
    }

    #[must_use]
    /// Interpret buffer as a sequence of untrusted `T`s
    #[inline(always)]
    pub fn chunks<U: Raw<M>, const M: usize>(&self) -> Result<Chunks<'_, U, M>, ValidationError> {
        if N % M != 0 {
            return Err(ValidationError);
        }
        Ok(Chunks {
            buf: &self.buf,
            _phantom: PhantomData,
        })
    }

    #[must_use]
    /// Peek first value of `U` from front of buffer
    #[inline(always)]
    pub fn peek<U: Raw<M>, const M: usize>(&self) -> Option<UntrustedData<U>> {
        if N < M {
            return None;
        }
        let mut temp = [0u8; M];
        temp.copy_from_slice(&self.buf[..M]);
        map_ok_option(U::from_bytes(temp), UntrustedData::new)
    }

    #[must_use]
    /// Pop first `U` from front, return value and tail as new buffer
    #[inline(always)]
    pub fn pop<U: Raw<M>, const M: usize>(&self) -> Option<(UntrustedData<U>, &[u8])> {
        if N < M {
            return None;
        }

        let mut temp = [0u8; M];
        temp.copy_from_slice(&self.buf[..M]);
        match U::from_bytes(temp) {
            Ok(val) => Some((UntrustedData::new(val), &self.buf[M..])),
            Err(_) => None,
        }
    }

    #[must_use]
    /// Rebuilds from a slice (must be exactly N bytes)
    #[inline(always)]
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        if bytes.len() != N {
            return Err(ValidationError);
        }
        let mut buf = [0u8; N];
        buf.copy_from_slice(bytes);
        Ok(Self::new(buf))
    }

    #[must_use]
    /// Parse to trusted `T`
    #[inline(always)]
    pub fn parse(self) -> Result<T, ValidationError>
    where
        T: Raw<N>,
    {
        T::from_bytes(self.buf)
    }
}

impl<T, const N: usize> FromStr for ByteBuf<T, N> {
    type Err = ValidationError;

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() > N {
            return Err(ValidationError);
        }
        let mut buf = [0u8; N];
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(Self::new(buf))
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
    #[must_use]
    fn from_bytes(bytes: [u8; N]) -> Result<Self, ValidationError>;

    /// Convert this value into a fixed-size byte buffer.
    #[must_use]
    fn to_bytes(&self) -> [u8; N];
}

impl Raw<4> for u32 {
    #[inline(always)]
    fn from_bytes(bytes: [u8; 4]) -> Result<Self, ValidationError> {
        Ok(u32::from_le_bytes(bytes))
    }

    #[inline(always)]
    fn to_bytes(&self) -> [u8; 4] {
        self.to_le_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{TrustedData, Unpack, UnpackBuf};
    use crate::trust::{Validate, ValidationError};

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Dummy(u32);

    impl Raw<4> for Dummy {
        fn from_bytes(bytes: [u8; 4]) -> Result<Self, ValidationError> {
            Ok(Dummy(u32::from_le_bytes(bytes)))
        }

        fn to_bytes(&self) -> [u8; 4] {
            self.0.to_le_bytes()
        }
    }

    impl Validate for Dummy {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.0 < 1000 {
                Ok(())
            } else {
                Err(ValidationError)
            }
        }
    }

    impl Unpack for Dummy {
        fn unpack_and_validate(
            buf: UnpackBuf<'_>,
        ) -> Result<TrustedData<'_, Self>, ValidationError> {
            let raw: [u8; 4] = buf.try_into_array().map_err(|_| ValidationError)?;
            let d = Dummy(u32::from_le_bytes(raw));
            d.validate()?;
            TrustedData::new(d)
        }
    }

    #[test]
    fn test_new_and_access() {
        let buf = ByteBuf::<Dummy, 4>::new([1, 2, 3, 4]);
        assert_eq!(buf.as_bytes(), &[1, 2, 3, 4]);
        assert_eq!(buf.into_bytes(), [1, 2, 3, 4]);
        assert_eq!(buf.len(), 4);
        assert!(!buf.is_empty());
    }

    #[test]
    fn test_zeroed_is_empty() {
        let buf = ByteBuf::<Dummy, 4>::new([0; 4]);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_parse() {
        let d = Dummy(123);
        let raw = d.to_bytes();
        let buf = ByteBuf::<Dummy, 4>::new(raw);
        assert_eq!(buf.parse().unwrap(), d);
    }

    #[test]
    fn test_try_unpack_valid() {
        let d = Dummy(42);
        let buf = ByteBuf::<Dummy, 4>::new(d.to_bytes());
        let result = buf.try_unpack();
        assert_eq!(result, Ok(d));
    }

    #[test]
    fn test_try_unpack_invalid_validation() {
        let d = Dummy(1234); // invalid per validate()
        let buf = ByteBuf::<Dummy, 4>::new(d.to_bytes());
        assert_eq!(buf.try_unpack(), Err(()));
    }

    #[test]
    fn test_from_str_truncates_and_pads() {
        let input = "AB";
        let buf = ByteBuf::<Dummy, 4>::from_str(input).unwrap();
        assert_eq!(buf.as_bytes(), b"AB\0\0");
    }

    #[test]
    fn test_from_str_too_long() {
        let input = "HELLO"; // > 4 bytes
        let err = ByteBuf::<Dummy, 4>::from_str(input).unwrap_err();
        assert_eq!(err, ValidationError);
    }

    #[test]
    fn test_chunks() {
        let d1 = Dummy(1);
        let d2 = Dummy(2);
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&d1.to_bytes());
        bytes[4..].copy_from_slice(&d2.to_bytes());

        let buf = ByteBuf::<Dummy, 8>::new(bytes);
        let chunks: Vec<_> = buf.chunks::<Dummy, 4>().unwrap().collect();
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].value(), &d1);
        assert_eq!(chunks[1].value(), &d2);
    }

    #[test]
    fn test_chunks_misaligned() {
        let buf = ByteBuf::<Dummy, 5>::new([0; 5]);
        let result = buf.chunks::<Dummy, 4>();
        assert!(
            result.is_err(),
            "Expected ValidationError due to misalignment"
        );
    }

    #[test]
    fn test_peek_ok() {
        let d = Dummy(99);
        let buf = ByteBuf::<Dummy, 4>::new(d.to_bytes());
        let peeked = buf.peek::<Dummy, 4>().unwrap();
        assert_eq!(peeked.value(), &d);
    }

    #[test]
    fn test_peek_too_small() {
        let buf = ByteBuf::<Dummy, 2>::new([1, 2]);
        assert!(buf.peek::<Dummy, 4>().is_none());
    }

    #[test]
    fn test_pop_valid() {
        let d1 = Dummy(55);
        let d2 = Dummy(88);

        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&d1.to_bytes());
        bytes[4..].copy_from_slice(&d2.to_bytes());

        let buf = ByteBuf::<Dummy, 8>::new(bytes);
        let (val, rest) = buf.pop::<Dummy, 4>().unwrap();
        assert_eq!(val.value(), &d1);
        assert_eq!(rest, &d2.to_bytes());
    }

    #[test]
    fn test_pop_insufficient_bytes() {
        let buf = ByteBuf::<Dummy, 2>::new([1, 2]);
        assert!(buf.pop::<Dummy, 4>().is_none());
    }

    #[test]
    fn test_chunks_custom() {
        let d1 = Dummy(1);
        let d2 = Dummy(2);

        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&d1.to_bytes());
        bytes[4..].copy_from_slice(&d2.to_bytes());

        let buf = ByteBuf::<Dummy, 8>::new(bytes);
        let chunks: Vec<_> = buf.chunks::<Dummy, 4>().unwrap().collect();
        assert_eq!(chunks[0].value(), &d1);
        assert_eq!(chunks[1].value(), &d2);
    }
}
