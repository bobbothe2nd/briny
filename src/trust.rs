//! Trust-boundary types for secure, zero-trust data handling.
//!
//! This module provides traits and wrappers for working with unvalidated
//! input (`UntrustedData`) and safely transitioning to validated, trusted
//! values (`TrustedData`) after explicit validation.
//!
//! # Trust Model
//! - External data is always treated as [`UntrustedData`].
//! - All validation logic is centralized in types that implement [`Validate`].
//! - Only data that passes validation can be wrapped in [`TrustedData`].
//! - Trusted data cannot be forged or constructed outside this module (sealed).
//!
//! # Purpose
//! This separation enables compile-time guarantees that:
//! - Trusted logic cannot operate on unchecked inputs
//! - Validation cannot be bypassed accidentally or maliciously
//! - All trust transitions are explicitly type-gated
//!
//! # ZTA Alignment
//! This follows core principles of Zero Trust Architecture:
//! - Never trust data by default — even internal inputs
//! - Enforce validation at every boundary (parse, deserialize, etc.)
//! - Isolate untrusted data until it has been verified
//! - Provide auditable trust transitions (e.g. `TrustedData::new()`)
//!
//! # Core Types
//! - [`UntrustedData<'a, T>`]: untrusted wrapper with a lifetime anchor
//! - [`TrustedData<'a, T>`]: post-validation wrapper, implements [`Trusted`]
//! - [`Validate`]: trait for enforcing invariants during trust transition
//! - [`TrustFrom`]: generic builder trait from untrusted inputs

use crate::pack::{Pack, PackRef, Unpack, UnpackBuf};
use core::{fmt::Debug, marker::PhantomData};

/// Marker trait for values that have passed validation and are considered safe.
///
/// Only crate-defined types (like `TrustedData`) can implement this trait,
/// due to the sealed trait pattern. This prevents external code from
/// arbitrarily marking data as trusted.
pub trait Trusted: private::Sealed + Validate {}

/// Marker trait for values that *have not* yet been validated.
///
/// Can be freely implemented by any type that carries untrusted or raw input.
/// Use this to gate logic that should never run on unchecked data.
pub trait Untrusted {}

/// Validation failure type.
///
/// Implementors of `Validate` return this to indicate invalid inputs.
///
/// # Binary Size Concerns
///
/// Implements `core::fmt::Display` and `core::error::Error`.
///
/// These implementations do **not** rely on `std`, and avoid formatting macros
/// to minimize binary size. In embedded builds, avoid calling `.to_string()`
/// or `format!()` with this type, it can increase binary size by several KiB.
pub struct ValidationError;

impl core::fmt::Display for ValidationError {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("\n    === validation failed")
    }
}

impl Debug for ValidationError {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("\n    === validation failed")
    }
}

impl core::error::Error for ValidationError {}

/// A trait for types that know how to validate their own internal invariants.
///
/// This trait is the gateway from `UntrustedData` → `TrustedData`. If
/// `validate()` fails, the data should be considered unusable.
pub trait Validate<C = ()> {
    /// Validator method to confirm trust on implementors.
    fn validate(&self) -> Result<(), ValidationError>;

    /// Context aware validator method.
    fn validate_with(&self, _ctx: &C) -> Result<(), ValidationError> {
        self.validate()
    }
}

impl<T: Clone, const N: usize> Validate for crate::raw::ByteBuf<T, N> {
    #[inline(always)]
    fn validate(&self) -> Result<(), ValidationError> {
        if self.clone().len() < 256 {
            Ok(())
        } else {
            Err(ValidationError)
        }
    }
}

/// Wrapper type for input from external/untrusted sources.
///
/// Holds a value of type `T` and a phantom lifetime `'a` that anchors the
/// untrusted data to some lifetime context (e.g., request/packet).
///
/// Use this to prevent accidental misuse of unsafe inputs.
pub struct UntrustedData<'a, T> {
    value: T,
    _marker: PhantomData<&'a ()>, // Binds `T`'s lifetime to `'a`
}

impl<'a, T> UntrustedData<'a, T> {
    #[must_use]
    /// Constructs a new `UntrustedData` wrapper.
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    #[must_use]
    /// Borrow the underlying data (still untrusted).
    #[inline(always)]
    pub const fn as_ref(&self) -> &T {
        &self.value
    }

    /// Attempt to convert the inner value to TrustedData.
    #[inline(always)]
    pub fn trust(self) -> Result<TrustedData<'a, T>, ValidationError>
    where
        T: Validate,
    {
        TrustedData::new(self.value)
    }
}

impl<'a, T> Untrusted for UntrustedData<'a, T> {}

impl<'a, T: Validate> Validate for UntrustedData<'a, T> {
    fn validate(&self) -> Result<(), ValidationError> {
        self.value.validate()
    }
}

/// Wrapper type for data that has passed validation and is safe to use.
///
/// This is the only public type that implements `Trusted`, and it cannot be
/// constructed outside of this module without passing validation.
pub struct TrustedData<'a, T> {
    inner: T,
    _marker: PhantomData<&'a ()>,
}

impl<'a, T> TrustedData<'a, T> {
    /// Validates the given value and, if successful, wraps it as `TrustedData`.
    #[inline(always)]
    pub fn new(value: T) -> Result<Self, ValidationError>
    where
        T: Validate,
    {
        value.validate()?;
        Ok(Self {
            inner: value,
            _marker: PhantomData,
        })
    }

    #[must_use]
    /// Borrow the trusted inner value.
    #[inline(always)]
    pub const fn as_ref(&self) -> &T {
        &self.inner // rename to `value()` for consistency
    }

    #[must_use]
    /// Consume the wrapper and return the trusted inner value.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Serialize the trusted value using its `Pack` implementation.
    #[inline(always)]
    pub fn pack(&self, out: PackRef<'_>) -> Result<(), ValidationError>
    where
        T: Pack,
    {
        self.inner.pack(out)
    }

    /// Deserialize and validate a value from the given input buffer.
    ///
    /// Returns an error if validation fails.
    #[inline(always)]
    pub fn unpack(input: UnpackBuf<'a>) -> Result<Self, ValidationError>
    where
        T: Unpack,
    {
        T::unpack_and_validate(input)
    }

    #[must_use]
    /// Re-check whether the value is still valid.
    ///
    /// This may be useful in long-lived or transformed data.
    #[inline(always)]
    pub fn is_valid(&self) -> bool
    where
        T: Validate,
    {
        self.inner.validate().is_ok()
    }

    /// Attempt to transform the value while preserving trust boundaries.
    ///
    /// The transformation must produce a new value that also validates.
    #[inline(always)]
    pub fn try_map<U, F>(self, f: F) -> Result<TrustedData<'a, U>, ValidationError>
    where
        F: FnOnce(T) -> U,
        U: Validate,
    {
        let result = f(self.inner);
        TrustedData::new(result)
    }
}

/// Trait for types that can be built from `UntrustedData` after validation.
///
/// Enables ergonomic `trust_from()` constructors without exposing internals.
pub trait TrustFrom<'a, T>: Sized + Validate {
    /// Quickly converts UntrustedData to TrustedData without enforcing validation.
    fn trust_from(input: UntrustedData<'a, T>) -> Result<TrustedData<'a, Self>, ValidationError>;
}

mod private {
    use crate::trust::Validate;

    #[doc(hidden)]
    pub trait Sealed {}
    impl<'a, T> Sealed for super::TrustedData<'a, T> where T: Validate {}
}

impl<'a, T: Validate> Trusted for TrustedData<'a, T> {}

impl<'a, T: Validate> Validate for TrustedData<'a, T> {
    fn validate(&self) -> Result<(), ValidationError> {
        self.inner.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyData([u8; 4]);

    impl Validate for MyData {
        fn validate(&self) -> Result<(), ValidationError> {
            if self.0[0] == 42 {
                Ok(())
            } else {
                Err(ValidationError)
            }
        }
    }

    #[test]
    fn test_trusted_data_validation() {
        let valid = MyData([42, 0, 0, 0]);
        let result = TrustedData::new(valid);
        assert!(result.is_ok());

        let invalid = MyData([0, 0, 0, 0]);
        assert!(TrustedData::new(invalid).is_err());
    }
}
