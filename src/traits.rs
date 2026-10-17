//! Traits to abstract common characteristics among types.

use core::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Saturating, Wrapping,
    },
    ptr::NonNull,
};

use crate::raw::MaybeNull;

/// Marker trait for types subject to the null pointer optimization.
///
/// # Safety
///
/// If zeroed is a valid bitpattern, undefined behavior will occur
/// when trying to optimize memory.
pub unsafe trait NonNullable {}

unsafe impl<T: CompilerAssumedNonNullable> NonNullable for T {}

/// Marker trait for types subject to the null pointer optimization by the compiler.
///
/// # Safety
///
/// If the compiler can safely assume the type can't represent a zeroed bitpattern,
/// this is safe. It isn't safe to implement on `Pod` types. The type must have valid
/// bitpatterns for every other type.
pub unsafe trait CompilerAssumedNonNullable: NonNullable {}

unsafe impl CompilerAssumedNonNullable for NonZeroU8 {}
unsafe impl CompilerAssumedNonNullable for NonZeroI8 {}
unsafe impl CompilerAssumedNonNullable for NonZeroU16 {}
unsafe impl CompilerAssumedNonNullable for NonZeroI16 {}
unsafe impl CompilerAssumedNonNullable for NonZeroU32 {}
unsafe impl CompilerAssumedNonNullable for NonZeroI32 {}
unsafe impl CompilerAssumedNonNullable for NonZeroU64 {}
unsafe impl CompilerAssumedNonNullable for NonZeroI64 {}
unsafe impl CompilerAssumedNonNullable for NonZeroU128 {}
unsafe impl CompilerAssumedNonNullable for NonZeroI128 {}
unsafe impl CompilerAssumedNonNullable for NonZeroUsize {}
unsafe impl CompilerAssumedNonNullable for NonZeroIsize {}
unsafe impl<T> CompilerAssumedNonNullable for NonNull<T> {}
unsafe impl<T> CompilerAssumedNonNullable for &T {}
unsafe impl<T> CompilerAssumedNonNullable for &mut T {}

/// Marker trait for types that are valid to be any bitpattern that is nonzero/nonnull.
///
/// # Safety
///
/// If other bitpatterns are invalid, implementing this trait is unsound.
pub unsafe trait AnyNonNull: CompilerAssumedNonNullable + 'static {}

unsafe impl AnyNonNull for NonZeroU8 {}
unsafe impl AnyNonNull for NonZeroI8 {}
unsafe impl AnyNonNull for NonZeroU16 {}
unsafe impl AnyNonNull for NonZeroI16 {}
unsafe impl AnyNonNull for NonZeroU32 {}
unsafe impl AnyNonNull for NonZeroI32 {}
unsafe impl AnyNonNull for NonZeroU64 {}
unsafe impl AnyNonNull for NonZeroI64 {}
unsafe impl AnyNonNull for NonZeroU128 {}
unsafe impl AnyNonNull for NonZeroI128 {}
unsafe impl AnyNonNull for NonZeroUsize {}
unsafe impl AnyNonNull for NonZeroIsize {}

/// POD marker trait for *Plain Old Data*.
///
/// # Safety
///
/// - All bit patterns of `T` must be valid
/// - `T` must have no padding or initialized padding
/// - `T` must implement [`StableLayout`] + [`RawConvert`]
///
/// Violating any of these constraints is bound to cause undefined behavior.
pub unsafe trait Pod: 'static {}

unsafe impl Pod for () {}
unsafe impl Pod for usize {}
unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for u128 {}
unsafe impl Pod for isize {}
unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for i128 {}
unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}
unsafe impl<T: Pod> Pod for ManuallyDrop<T> {}
unsafe impl<T: Pod> Pod for Wrapping<T> {}
unsafe impl<T: Pod> Pod for Saturating<T> {}
unsafe impl<T: CompilerAssumedNonNullable + AnyNonNull> Pod for Option<T> {}
unsafe impl<T: NonNullable + 'static> Pod for MaybeNull<T> {}
unsafe impl<T: Pod> Pod for Cell<T> {}
unsafe impl<T: Pod> Pod for UnsafeCell<T> {}

unsafe impl<T: 'static> Pod for PhantomData<T> {}
unsafe impl<T: 'static> Pod for MaybeUninit<T> {}

#[cfg(feature = "half")]
unsafe impl Pod for half::f16 {}
#[cfg(feature = "half")]
unsafe impl Pod for half::bf16 {}

#[cfg(feature = "nightly_float")]
unsafe impl Pod for f16 {}
#[cfg(feature = "nightly_float")]
unsafe impl Pod for f128 {}

#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m128 {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m128bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m128d {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m128i {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m256 {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m256bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m256d {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m256i {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m512 {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m512bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m512d {}
#[cfg(target_arch = "x86_64")]
unsafe impl Pod for core::arch::x86_64::__m512i {}

/// Marker trait used to determine what types are safe to cast.
///
/// If a type is safe to cast to another type, but not any type, this should be used
/// instead of [`Pod`].
///
/// # Safety
///
/// This is less strict and is safe as long as it is safe to reinterpret the bytes
/// of any given `Self` as a `T`. It isn't required, however, that any `T` can be
/// reinterpreted as a `Self`. It is also completely unrelated to all other types.
pub unsafe trait Layout<T> {}

unsafe impl<T: Pod, U: Pod> Layout<U> for T {}

unsafe impl<T: AnyNonNull> Layout<bool> for T {}
unsafe impl<T: AnyNonNull> Layout<char> for T {}

unsafe impl<T: Pod> Layout<NonZeroI128> for T {}
unsafe impl<T: AnyNonNull> Layout<NonZeroU128> for T {}

unsafe impl<T: AnyNonNull> Layout<NonZeroI64> for T {}
unsafe impl<T: AnyNonNull> Layout<NonZeroU64> for T {}

unsafe impl<T: AnyNonNull> Layout<NonZeroI32> for T {}
unsafe impl<T: AnyNonNull> Layout<NonZeroU32> for T {}

unsafe impl<T: AnyNonNull> Layout<NonZeroI16> for T {}
unsafe impl<T: AnyNonNull> Layout<NonZeroU16> for T {}

unsafe impl<T: AnyNonNull> Layout<NonZeroI8> for T {}
unsafe impl<T: AnyNonNull> Layout<NonZeroU8> for T {}

unsafe impl<T: AnyNonNull> Layout<NonZeroUsize> for T {}
unsafe impl<T: AnyNonNull> Layout<NonZeroIsize> for T {}

unsafe impl<T: Pod> Layout<T> for NonZeroU128 {}
unsafe impl<T: Pod> Layout<T> for NonZeroI128 {}

unsafe impl<T: Pod> Layout<T> for NonZeroU64 {}
unsafe impl<T: Pod> Layout<T> for NonZeroI64 {}

unsafe impl<T: Pod> Layout<T> for NonZeroU32 {}
unsafe impl<T: Pod> Layout<T> for NonZeroI32 {}

unsafe impl<T: Pod> Layout<T> for NonZeroU16 {}
unsafe impl<T: Pod> Layout<T> for NonZeroI16 {}

unsafe impl<T: Pod> Layout<T> for NonZeroU8 {}
unsafe impl<T: Pod> Layout<T> for NonZeroI8 {}

unsafe impl<T: Pod> Layout<T> for NonZeroUsize {}
unsafe impl<T: Pod> Layout<T> for NonZeroIsize {}

unsafe impl<T: AnyNonNull, U> Layout<NonNull<U>> for T {}

/// Similar to [`Layout`], but that describes byte validity whereas this trait also requires copy safety.
///
/// # Safety
///
/// Slightly stricter than [`Layout`] and different from [`Pod`], it is safe to implement this trait on any type
/// that is safe to copy into a type of `T`.
///
/// This is always safe where `Self: Copy + Layout<T>, T: Copy`.
pub unsafe trait CopySafe<T>: Layout<T> {}

unsafe impl<T: Layout<U> + Copy, U: Copy> CopySafe<U> for T {}

/// Every bitpattern must be valid except the one pattern `check_valid` checks for.
///
/// # Safety
///
/// If the invalid bitpattern of [`Self::INVALID`] doesn't represent the valid type,
/// or if [`Self::check_valid`] returns `true` for that bitpattern, this is unsound
/// when used with [`NotPattern`]
pub unsafe trait InvalidPattern: Sized + 'static {
    /// The initialized type to coerce to.
    type Valid: Layout<Self>;

    /// Defines invalid bitpattern as valid.
    const INVALID: Self;

    /// Checks for one invalid bitpattern.
    #[must_use]
    fn check_valid(self) -> bool;
}

/// Every byte must be "initialized" at all times
///
/// # Safety
///
/// Rust considers it undefined behavior to read padding bytes of any type.
/// These padding bytes are uninitialized with values unspecified.
#[deprecated = "just use Pod"]
pub unsafe trait NoUninitPadding {}

#[allow(deprecated)]
mod uninit {
    use super::{
        AnyNonNull, Cell, CompilerAssumedNonNullable, ManuallyDrop, MaybeNull, MaybeUninit,
        NoUninitPadding, NonNull, NonNullable, NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64,
        NonZeroI8, NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8,
        NonZeroUsize, PhantomData, Saturating, UnsafeCell, Wrapping,
    };

    unsafe impl NoUninitPadding for () {}
    unsafe impl NoUninitPadding for usize {}
    unsafe impl NoUninitPadding for u8 {}
    unsafe impl NoUninitPadding for u16 {}
    unsafe impl NoUninitPadding for u32 {}
    unsafe impl NoUninitPadding for u64 {}
    unsafe impl NoUninitPadding for u128 {}
    unsafe impl NoUninitPadding for isize {}
    unsafe impl NoUninitPadding for i8 {}
    unsafe impl NoUninitPadding for i16 {}
    unsafe impl NoUninitPadding for i32 {}
    unsafe impl NoUninitPadding for i64 {}
    unsafe impl NoUninitPadding for i128 {}
    unsafe impl NoUninitPadding for f32 {}
    unsafe impl NoUninitPadding for f64 {}
    unsafe impl<T: NoUninitPadding, const N: usize> NoUninitPadding for [T; N] {}
    unsafe impl<T: NoUninitPadding> NoUninitPadding for ManuallyDrop<T> {}
    unsafe impl<T: NoUninitPadding> NoUninitPadding for Wrapping<T> {}
    unsafe impl<T: NoUninitPadding> NoUninitPadding for Saturating<T> {}
    unsafe impl<T: CompilerAssumedNonNullable + AnyNonNull> NoUninitPadding for Option<T> {}
    unsafe impl<T: NonNullable + 'static> NoUninitPadding for MaybeNull<T> {}
    unsafe impl<T: NoUninitPadding> NoUninitPadding for Cell<T> {}
    unsafe impl<T: NoUninitPadding> NoUninitPadding for UnsafeCell<T> {}

    unsafe impl<T: 'static> NoUninitPadding for PhantomData<T> {}
    unsafe impl<T: 'static> NoUninitPadding for MaybeUninit<T> {}

    #[cfg(feature = "half")]
    unsafe impl NoUninitPadding for half::f16 {}
    #[cfg(feature = "half")]
    unsafe impl NoUninitPadding for half::bf16 {}

    #[cfg(feature = "nightly_float")]
    unsafe impl NoUninitPadding for f16 {}
    #[cfg(feature = "nightly_float")]
    unsafe impl NoUninitPadding for f128 {}

    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m128 {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m128bh {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m128d {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m128i {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m256 {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m256bh {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m256d {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m256i {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m512 {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m512bh {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m512d {}
    #[cfg(target_arch = "x86_64")]
    unsafe impl NoUninitPadding for core::arch::x86_64::__m512i {}

    unsafe impl NoUninitPadding for NonZeroU8 {}
    unsafe impl NoUninitPadding for NonZeroI8 {}
    unsafe impl NoUninitPadding for NonZeroU16 {}
    unsafe impl NoUninitPadding for NonZeroI16 {}
    unsafe impl NoUninitPadding for NonZeroU32 {}
    unsafe impl NoUninitPadding for NonZeroI32 {}
    unsafe impl NoUninitPadding for NonZeroU64 {}
    unsafe impl NoUninitPadding for NonZeroI64 {}
    unsafe impl NoUninitPadding for NonZeroU128 {}
    unsafe impl NoUninitPadding for NonZeroI128 {}
    unsafe impl NoUninitPadding for NonZeroUsize {}
    unsafe impl NoUninitPadding for NonZeroIsize {}
    unsafe impl<T> NoUninitPadding for NonNull<T> {}
    unsafe impl<T> NoUninitPadding for &T {}
    unsafe impl<T> NoUninitPadding for &mut T {}
}
