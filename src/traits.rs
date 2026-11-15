//! Traits to abstract common characteristics among types.

use core::{
    cell::{Cell, UnsafeCell}, marker::{PhantomData, PhantomPinned}, mem::{ManuallyDrop, MaybeUninit}, num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize, Saturating, Wrapping,
    }, ptr::NonNull,
};

#[cfg(target_has_atomic = "8")]
use core::sync::atomic::{AtomicBool, AtomicI8, AtomicU8};
#[cfg(target_has_atomic = "16")]
use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};
#[cfg(target_has_atomic = "ptr")]
use core::sync::atomic::{AtomicPtr, AtomicIsize, AtomicUsize};

use stable_auto::{auto_leaf, auto_trait};

pub use stable_auto::Auto;

use crate::raw::MaybeNull;

/// Every bitpattern must be valid except the one pattern `check_valid` checks for.
///
/// # Safety
///
/// If the invalid bitpattern of [`Self::INVALID`] doesn't represent the valid type,
/// or if [`Self::check_valid`] returns `true` for that bitpattern, this is unsound
/// when used with [`NotPattern`]
pub unsafe trait InvalidPattern: Sized + 'static
where
    Self: Layout<Self::Valid>,
{
    /// The initialized type to coerce to.
    type Valid: ReprSafe;

    /// Defines invalid bitpattern as valid.
    const INVALID: Self;

    /// Checks for one invalid bitpattern.
    #[must_use]
    fn check_valid(self) -> bool;
}

macro_rules! for_pod_ty {
    ($nonnull:ident $trait:ident $macro:path) => {
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
            [T: $trait, const N: usize] [T; N]
            [T: $trait] ManuallyDrop<T>
            [T: $trait] Wrapping<T>
            [T: $trait] Saturating<T>
            [T: $nonnull + ReprSafe] Option<T>
            [T: $nonnull + ReprSafe] MaybeNull<T>
            [T: $trait] Cell<T>
            [T: $trait] UnsafeCell<T>

            [T: $trait] PhantomData<T>
            [] PhantomPinned

            #[cfg(feature = "half")]
            [] half::f16
            #[cfg(feature = "half")]
            [] half::bf16

            #[cfg(feature = "nightly_float")]
            [] f16
            #[cfg(feature = "nightly_float")]
            [] f128

            #[cfg(target_family = "wasm")]
            [] core::arch::wasm32::v128
        }

        #[cfg(target_arch = "x86_64")]
        $macro! {
            [] core::arch::x86_64::__m128
            [] core::arch::x86_64::__m128bh
            [] core::arch::x86_64::__m128d
            [] core::arch::x86_64::__m128i
            [] core::arch::x86_64::__m256
            [] core::arch::x86_64::__m256bh
            [] core::arch::x86_64::__m256d
            [] core::arch::x86_64::__m256i
            [] core::arch::x86_64::__m512
            [] core::arch::x86_64::__m512bh
            [] core::arch::x86_64::__m512d
            [] core::arch::x86_64::__m512i
        }

        #[cfg(any(target_arch = "aarch64", target_arch = "arm64ec"))]
        $macro! {
            [] core::arch::aarch64::float32x2_t
            [] core::arch::aarch64::float32x2x2_t
            [] core::arch::aarch64::float32x2x3_t
            [] core::arch::aarch64::float32x2x4_t
            [] core::arch::aarch64::float32x4_t
            [] core::arch::aarch64::float32x4x2_t
            [] core::arch::aarch64::float32x4x3_t
            [] core::arch::aarch64::float32x4x4_t
            [] core::arch::aarch64::float64x1_t
            [] core::arch::aarch64::float64x1x2_t
            [] core::arch::aarch64::float64x1x3_t
            [] core::arch::aarch64::float64x1x4_t
            [] core::arch::aarch64::float64x2_t
            [] core::arch::aarch64::float64x2x2_t
            [] core::arch::aarch64::float64x2x3_t
            [] core::arch::aarch64::float64x2x4_t
            [] core::arch::aarch64::int16x4_t
            [] core::arch::aarch64::int16x4x2_t
            [] core::arch::aarch64::int16x4x3_t
            [] core::arch::aarch64::int16x4x4_t
            [] core::arch::aarch64::int16x8_t
            [] core::arch::aarch64::int16x8x2_t
            [] core::arch::aarch64::int16x8x3_t
            [] core::arch::aarch64::int16x8x4_t
            [] core::arch::aarch64::int32x2_t
            [] core::arch::aarch64::int32x2x2_t
            [] core::arch::aarch64::int32x2x3_t
            [] core::arch::aarch64::int32x2x4_t
            [] core::arch::aarch64::int32x4_t
            [] core::arch::aarch64::int32x4x2_t
            [] core::arch::aarch64::int32x4x3_t
            [] core::arch::aarch64::int32x4x4_t
            [] core::arch::aarch64::int64x1_t
            [] core::arch::aarch64::int64x1x2_t
            [] core::arch::aarch64::int64x1x3_t
            [] core::arch::aarch64::int64x1x4_t
            [] core::arch::aarch64::int64x2_t
            [] core::arch::aarch64::int64x2x2_t
            [] core::arch::aarch64::int64x2x3_t
            [] core::arch::aarch64::int64x2x4_t
            [] core::arch::aarch64::int8x16_t
            [] core::arch::aarch64::int8x16x2_t
            [] core::arch::aarch64::int8x16x3_t
            [] core::arch::aarch64::int8x16x4_t
            [] core::arch::aarch64::int8x8_t
            [] core::arch::aarch64::int8x8x2_t
            [] core::arch::aarch64::int8x8x3_t
            [] core::arch::aarch64::int8x8x4_t
            [] core::arch::aarch64::poly16x4_t
            [] core::arch::aarch64::poly16x4x2_t
            [] core::arch::aarch64::poly16x4x3_t
            [] core::arch::aarch64::poly16x4x4_t
            [] core::arch::aarch64::poly16x8_t
            [] core::arch::aarch64::poly16x8x2_t
            [] core::arch::aarch64::poly16x8x3_t
            [] core::arch::aarch64::poly16x8x4_t
            [] core::arch::aarch64::poly64x1_t
            [] core::arch::aarch64::poly64x1x2_t
            [] core::arch::aarch64::poly64x1x3_t
            [] core::arch::aarch64::poly64x1x4_t
            [] core::arch::aarch64::poly64x2_t
            [] core::arch::aarch64::poly64x2x2_t
            [] core::arch::aarch64::poly64x2x3_t
            [] core::arch::aarch64::poly64x2x4_t
            [] core::arch::aarch64::poly8x16_t
            [] core::arch::aarch64::poly8x16x2_t
            [] core::arch::aarch64::poly8x16x3_t
            [] core::arch::aarch64::poly8x16x4_t
            [] core::arch::aarch64::poly8x8_t
            [] core::arch::aarch64::poly8x8x2_t
            [] core::arch::aarch64::poly8x8x3_t
            [] core::arch::aarch64::poly8x8x4_t
            [] core::arch::aarch64::uint16x4_t
            [] core::arch::aarch64::uint16x4x2_t
            [] core::arch::aarch64::uint16x4x3_t
            [] core::arch::aarch64::uint16x4x4_t
            [] core::arch::aarch64::uint16x8_t
            [] core::arch::aarch64::uint16x8x2_t
            [] core::arch::aarch64::uint16x8x3_t
            [] core::arch::aarch64::uint16x8x4_t
            [] core::arch::aarch64::uint32x2_t
            [] core::arch::aarch64::uint32x2x2_t
            [] core::arch::aarch64::uint32x2x3_t
            [] core::arch::aarch64::uint32x2x4_t
            [] core::arch::aarch64::uint32x4_t
            [] core::arch::aarch64::uint32x4x2_t
            [] core::arch::aarch64::uint32x4x3_t
            [] core::arch::aarch64::uint32x4x4_t
            [] core::arch::aarch64::uint64x1_t
            [] core::arch::aarch64::uint64x1x2_t
            [] core::arch::aarch64::uint64x1x3_t
            [] core::arch::aarch64::uint64x1x4_t
            [] core::arch::aarch64::uint64x2_t
            [] core::arch::aarch64::uint64x2x2_t
            [] core::arch::aarch64::uint64x2x3_t
            [] core::arch::aarch64::uint64x2x4_t
            [] core::arch::aarch64::uint8x16_t
            [] core::arch::aarch64::uint8x16x2_t
            [] core::arch::aarch64::uint8x16x3_t
            [] core::arch::aarch64::uint8x16x4_t
            [] core::arch::aarch64::uint8x8_t
            [] core::arch::aarch64::uint8x8x2_t
            [] core::arch::aarch64::uint8x8x3_t
            [] core::arch::aarch64::uint8x8x4_t
        }
    };
}

/// Marker trait for types which can be safely zeroed.
#[auto_trait]
pub unsafe trait Zeroable {}

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

for_pod_ty!(CompilerAssumedNonNullable Zeroable impl_zero);

impl_zero! {
    #[cfg(target_has_atomic = "8")]
    [] AtomicBool
    #[cfg(target_has_atomic = "8")]
    [] AtomicI8
    #[cfg(target_has_atomic = "8")]
    [] AtomicU8

    #[cfg(target_has_atomic = "16")]
    [] AtomicI16
    #[cfg(target_has_atomic = "16")]
    [] AtomicU16

    #[cfg(target_has_atomic = "32")]
    [] AtomicI32
    #[cfg(target_has_atomic = "32")]
    [] AtomicU32

    #[cfg(target_has_atomic = "64")]
    [] AtomicI64
    #[cfg(target_has_atomic = "64")]
    [] AtomicU64

    #[cfg(target_has_atomic = "ptr")]
    [T] AtomicPtr<T>
    #[cfg(target_has_atomic = "ptr")]
    [] AtomicIsize
    #[cfg(target_has_atomic = "ptr")]
    [] AtomicUsize

    [] bool
    [] char
    [] str

    [T] MaybeUninit<T>
}

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
/// this is safe. It isn't safe to implement on `Pod` types. The type must have valid
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

/// Marker trait for types that are valid to be any bitpattern that is nonZero/nonnull.
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
/// - `T` must be inhabited and initialized
#[auto_trait]
pub unsafe trait Pod: ReprSafe + NoPadding + 'static {}

macro_rules! impl_pod {
    ($($(#[$attr:meta])*[$($generics:tt)*]$ty:ty)*) => {
        auto_leaf! {$(
            $(#[$attr])*
            unsafe impl<$($generics)*> Pod for $ty {}
        )*}
    };
}

for_pod_ty!(AnyNonNull Pod impl_pod);

/// Marker trait describing directional bit-pattern validity.
///
/// # Safety
///
/// Every valid bit pattern of `Self` must be a valid bit pattern of `T`.
///
/// This trait does not guarantee equal size, alignment, or absence of padding.
/// Those properties must be checked separately.
pub unsafe trait Layout<T: ReprSafe + ?Sized>: ReprSafe {}

unsafe impl<S: ReprSafe, D: Pod> Layout<D> for S {}

/// Similar to [`Layout`], but that describes byte validity whereas this trait also requires copy safety.
///
/// # Safety
///
/// Slightly stricter than [`Layout`] and different from [`Pod`], it is safe to implement this trait on any type
/// that is safe to copy into a type of `T`.
///
/// This is always safe where `Self: Layout<T>, T: Copy`.
pub unsafe trait CopySafe<T: ReprSafe>: Layout<T> {}

unsafe impl<T: Layout<U>, U: ReprSafe + Copy> CopySafe<U> for T {}

/// Marker trait for types with no uninitialized padding bytes.
///
/// The associated constant `HAS_NO_PADDING` is what decides if it is safe.
pub unsafe trait NoPadding {
    /// `true` iff `Self` has no padding, else `false`
    const HAS_NO_PADDING: bool;
}

unsafe impl<T> NoPadding for T
where
    T: Auto,
    T::Fields: Sized,
{
    const HAS_NO_PADDING: bool = size_of::<T::Fields>() == size_of::<T>();
}

/// Types whose initialized representations may be safely reinterpreted.
///
/// # Safety
///
/// No guarantee of bitpattern validity, but reading bytes from this type is safe.
/// Safe as source (reads), but not destination (writes).
///
/// This has nothing to do with FFI or `repr(C)`!
#[auto_trait]
pub unsafe trait ReprSafe {}

macro_rules! impl_repr_safe {
    ($($(#[$attr:meta])*[$($generics:tt)*]$ty:ty)*) => {
        auto_leaf! {$(
            $(#[$attr])*
            unsafe impl<$($generics)*> ReprSafe for $ty {}
        )*}
    };
}

for_pod_ty!(CompilerAssumedNonNullable ReprSafe impl_repr_safe);

impl_repr_safe! {
    #[cfg(target_has_atomic = "8")]
    [] AtomicBool
    #[cfg(target_has_atomic = "8")]
    [] AtomicI8
    #[cfg(target_has_atomic = "8")]
    [] AtomicU8

    #[cfg(target_has_atomic = "16")]
    [] AtomicI16
    #[cfg(target_has_atomic = "16")]
    [] AtomicU16

    #[cfg(target_has_atomic = "32")]
    [] AtomicI32
    #[cfg(target_has_atomic = "32")]
    [] AtomicU32

    #[cfg(target_has_atomic = "64")]
    [] AtomicI64
    #[cfg(target_has_atomic = "64")]
    [] AtomicU64

    #[cfg(target_has_atomic = "ptr")]
    [T] AtomicPtr<T>
    #[cfg(target_has_atomic = "ptr")]
    [] AtomicIsize
    #[cfg(target_has_atomic = "ptr")]
    [] AtomicUsize

    [] NonZeroI8
    [] NonZeroU8
    [] NonZeroI16
    [] NonZeroU16
    [] NonZeroI32
    [] NonZeroU32
    [] NonZeroI64
    [] NonZeroU64
    [] NonZeroI128
    [] NonZeroU128
    [] NonZeroIsize
    [] NonZeroUsize

    [T] *const T
    [T] *mut T
    [T] NonNull<T>

    [] bool
    [] char
    [] str

    [T] MaybeUninit<T>
}
