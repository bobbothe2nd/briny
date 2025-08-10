//! A small module with traits to improve soundness in any codebase.
//!
//! # Validate
//!
//! Generic parameters: `C` (default: `()`)
//!
//! Methods: `validate(&self) -> Result<Self, BrinyError>`, `validate_with(&self, ctx: C) -> Result<Self, BrinyError>`, `try_validate(&self) -> bool`
//!
//! # Prepared
//!
//! Generic parameters: `T` (no defaults)
//!
//! Methods: `prepare(&mut self)`, `finalize(&self) -> T`

use crate::BrinyError;

/// A trait to validate and confirm trust.
///
/// When implementing, there is an optional type parameter `C` that enables use of a powerful `validate_with` method that takes `C` as an argument, e.g.
///
/// ```rust
/// use core::fmt::Debug;
/// use briny::{valid::Validate, BrinyError};
///
/// pub struct Foo;
///
/// impl<C: Debug + PartialEq<usize>> Validate<C> for Foo {
///     fn validate_with(&self, ctx: C) -> Result<Self, BrinyError>
///     where
///         Self: Sized,
///     {
///         assert_ne!(ctx, 0);
///
///         Ok(Self)
///     }
/// }
/// ```
pub trait Validate<C = ()> {
    /// Simple validator method to confirm trust upon the caller.
    ///
    /// # Errors
    ///
    /// When validation fails, `BrinyError` is returned. This can occur in innumerable cases of which cannot be covered.
    fn validate(&self) -> Result<Self, BrinyError>
    where
        Self: Sized,
    {
        Err(BrinyError)
    }

    /// Advanced validator method to confirm trust upon the caller. A context aware validator method, per se.
    ///
    /// In several cases, having no context in the validation isn't very helpful. If context is required, it is suggested that this function is used because the context is passed as a generic argument directly to the function - no need to manually replicate this behavior with the regular `validate` method.
    ///
    /// # Errors
    ///
    /// When validation fails, `BrinyError` is returned. This can occur in innumerable cases of which cannot be covered.
    fn validate_with(&self, _ctx: C) -> Result<Self, BrinyError>
    where
        Self: Sized,
    {
        self.validate()
    }

    /// Naive validator method to confirm trust upon the caller.
    ///
    /// Call it directly in statements to get a clear boolean value that is usually the equivalent of `self.validate().is_err()` as it is by default.
    ///
    /// Sometimes, it can be helpful to simplify validation to improve performance, that's precisely why this function exists; so you don't have to sacrifice security for performance.
    fn try_validate(&self) -> bool
    where
        Self: Sized,
    {
        self.validate().is_err()
    }
}

/// A trait for types that may not always be ready for arbitrary changes.
///
/// Provides methods to ensure soundness even in tight scenarios.
pub trait Prepared<T = ()> {
    /// Borrow mutably to change metadata or raw memory, preparing for finalization.
    fn prepare(&mut self);

    /// Perform necessary calculations to get the final data from assumed prepared data.
    fn finalize(&self) -> T;
}

/// A trait to get data from arbitrary sources.
pub trait Source<T> {
    /// Fetches a reference from the data source.
    fn fetch(&self) -> &T;

    /// Initializes the data source.
    fn init(&mut self) {}

    /// Fetches a reference from the data source given specific context.
    fn fetch_with<U>(&self, _ctx: U) -> &T {
        self.fetch()
    }
}
