//! Traits to abstract common characteristics among types.

use core::{
    cell::{Cell, UnsafeCell},
    marker::{PhantomData, PhantomPinned},
    mem::{ManuallyDrop, MaybeUninit},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Saturating, Wrapping,
    },
    ptr::NonNull,
};

use stable_auto::{auto_leaf, auto_trait};

pub use stable_auto::Auto;

use crate::raw::MaybeNull;

macro_rules! for_pod_ty {
    ($nonnull:ident $macro:path) => {
        $macro! {
            [] ()
            [] usize
            [] u8
            [] u16
            [] u32
            [] u64
            [] u128
            [] isize
            [] i8
            [] i16
            [] i32
            [] i64
            [] i128
            [] f32
            [] f64
            [T: ImplicitPod, const N: usize] [T; N]
            [T: ImplicitPod] ManuallyDrop<T>
            [T: ImplicitPod] Wrapping<T>
            [T: ImplicitPod] Saturating<T>
            [T: $nonnull] Option<T>
            [T: $nonnull] MaybeNull<T>
            [T: ImplicitPod] Cell<T>
            [T: ImplicitPod] UnsafeCell<T>

            [T: 'static] PhantomData<T>
            [] PhantomPinned

            #[cfg(feature = "half")]
            [] half::f16
            #[cfg(feature = "half")]
            [] half::bf16

            #[cfg(feature = "nightly_float")]
            [] f16
            #[cfg(feature = "nightly_float")]
            [] f128

            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m128
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m128bh
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m128d
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m128i
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m256
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m256bh
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m256d
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m256i
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m512
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m512bh
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m512d
            #[cfg(target_arch = "x86_64")]
            [] core::arch::x86_64::__m512i
        }
    };
}

/// Marker trait for types which can be safely zeroed.
#[auto_trait]
pub unsafe trait Zeroable: NoPadding {}

macro_rules! impl_zero {
    ($($(#[$attr:meta])*[$($generics:tt)*]$ty:ty)*) => {
        auto_leaf! {
            $(
                $(#[$attr])*
                unsafe impl<$($generics)*> Zeroable for $ty {}
            )*
        }
    };
}

for_pod_ty!(CompilerAssumedNonNullable impl_zero);
impl_zero!([T] MaybeUninit<T>);

/// Marker trait for types subject to the null pointer optimization.
///
/// # Safety
///
/// If zeroed is a valid bitpattern, undefined behavior will occur
/// when trying to optimize memory.
#[auto_trait]
pub unsafe trait NonNullable: NoPadding {}

auto_leaf! {
    unsafe impl<T: CompilerAssumedNonNullable + ?Sized> NonNullable for T {}
}

/// Marker trait for types subject to the null pointer optimization by the compiler.
///
/// # Safety
///
/// If the compiler can safely assume the type can't represent a zeroed bitpattern,
/// this is safe. It isn't safe to implement on `ImplicitPod` types. The type must have valid
/// bitpatterns for every other type.
pub unsafe trait CompilerAssumedNonNullable: NonNullable + NoPadding {}

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
#[auto_trait]
pub unsafe trait AnyNonNull: CompilerAssumedNonNullable + 'static {}

auto_leaf! {
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
}

/// POD marker trait for *Plain Old Data*.
///
/// # Safety
///
/// - All bit patterns of `T` must be valid
/// - `T` must have no padding or initialized padding
/// - `T` must implement [`StableLayout`] + [`RawConvert`]
///
/// Violating any of these constraints is bound to cause undefined behavior.
#[auto_trait]
pub unsafe trait ImplicitPod: NoPadding + 'static {}

auto_leaf! {
    unsafe impl<T: ExplicitPod> ImplicitPod for T {}
}

/// POD marker trait for *Plain Old Data*.
///
/// # Safety
///
/// - All bit patterns of `T` must be valid
/// - `T` must have no padding or initialized padding
/// - `T` must implement [`StableLayout`] + [`RawConvert`]
///
/// Violating any of these constraints is bound to cause undefined behavior.
pub unsafe trait ExplicitPod: NoPadding + 'static {}

macro_rules! impl_pod {
    ($($(#[$attr:meta])*[$($generics:tt)*]$ty:ty)*) => {
        $(
            $(#[$attr])*
            unsafe impl<$($generics)*> ExplicitPod for $ty {}
        )*
    };
}

for_pod_ty!(AnyNonNull impl_pod);

unsafe impl<T: ImplicitPod> ExplicitPod for crate::raw::PodCell<T> {}

/// Marker trait used to determine what types are safe to cast.
///
/// If a type is safe to cast to another type, but not any type, this should be used
/// instead of [`ImplicitPod`].
///
/// # Safety
///
/// This is less strict and is safe as long as it is safe to reinterpret the bytes
/// of any given `Self` as a `T`. It isn't required, however, that any `T` can be
/// reinterpreted as a `Self`. It is also completely unrelated to all other types.
pub unsafe trait Layout<T: ?Sized> {}

unsafe impl<T: ExplicitPod + ?Sized, U: ExplicitPod + ?Sized> Layout<U> for T {}

unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroI128> for T {}
unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroU128> for T {}

unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroI64> for T {}
unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroU64> for T {}

unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroI32> for T {}
unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroU32> for T {}

unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroI16> for T {}
unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroU16> for T {}

unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroI8> for T {}
unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroU8> for T {}

unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroUsize> for T {}
unsafe impl<T: AnyNonNull + ?Sized> Layout<NonZeroIsize> for T {}

unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroU128 {}
unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroI128 {}

unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroU64 {}
unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroI64 {}

unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroU32 {}
unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroI32 {}

unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroU16 {}
unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroI16 {}

unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroU8 {}
unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroI8 {}

unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroUsize {}
unsafe impl<T: ExplicitPod + ?Sized> Layout<T> for NonZeroIsize {}

unsafe impl<T: AnyNonNull, U> Layout<NonNull<U>> for T {}

unsafe impl<T, U> Layout<MaybeUninit<T>> for U {}

/// Similar to [`Layout`], but that describes byte validity whereas this trait also requires copy safety.
///
/// # Safety
///
/// Slightly stricter than [`Layout`] and different from [`ImplicitPod`], it is safe to implement this trait on any type
/// that is safe to copy into a type of `T`.
///
/// This is always safe where `Self: Layout<T>, T: Copy`.
pub unsafe trait CopySafe<T>: Layout<T> {}

unsafe impl<T: Layout<U>, U: Copy> CopySafe<U> for T {}

/// Marker trait for types with no uninitialized padding bytes.
pub unsafe trait NoPadding {
    #[doc(hidden)]
    const HAS_NO_PADDING: bool;
}

unsafe impl<T> NoPadding for T
where 
    T: Auto,
    T::Fields: Sized,
{
    const HAS_NO_PADDING: bool = size_of::<T::Fields>() == size_of::<T>();
}

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
