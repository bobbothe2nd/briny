//! Traits to abstract common characteristics among types.

use core::{
    cell::{Cell, LazyCell, OnceCell, RefCell, RefMut, UnsafeCell},
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Saturating, Wrapping,
    },
    ptr::NonNull,
    sync::atomic::{
        AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
        AtomicU64, AtomicU8, AtomicUsize,
    },
};

use crate::raw::MaybeNull;

/// A simple marker trait for types that have a consistent layout in memory.
///
/// # Safety
///
/// If this type does not have a stable layout, this is invalid. e.g., if the type
/// depends on something not listed in struct fields or assumes anything about the device.
pub unsafe trait StableLayout: Sized + 'static {}

unsafe impl StableLayout for () {}
unsafe impl StableLayout for u8 {}
unsafe impl StableLayout for i8 {}
unsafe impl StableLayout for u16 {}
unsafe impl StableLayout for i16 {}
unsafe impl StableLayout for u32 {}
unsafe impl StableLayout for i32 {}
unsafe impl StableLayout for u64 {}
unsafe impl StableLayout for i64 {}
unsafe impl StableLayout for u128 {}
unsafe impl StableLayout for i128 {}
unsafe impl StableLayout for usize {}
unsafe impl StableLayout for isize {}
unsafe impl StableLayout for f32 {}
unsafe impl StableLayout for f64 {}
unsafe impl StableLayout for bool {}
unsafe impl StableLayout for AtomicU8 {}
unsafe impl StableLayout for AtomicI8 {}
unsafe impl StableLayout for AtomicU16 {}
unsafe impl StableLayout for AtomicI16 {}
unsafe impl StableLayout for AtomicU32 {}
unsafe impl StableLayout for AtomicI32 {}
unsafe impl StableLayout for AtomicU64 {}
unsafe impl StableLayout for AtomicI64 {}
unsafe impl StableLayout for AtomicUsize {}
unsafe impl StableLayout for AtomicIsize {}
unsafe impl StableLayout for AtomicBool {}
unsafe impl StableLayout for NonZeroU8 {}
unsafe impl StableLayout for NonZeroI8 {}
unsafe impl StableLayout for NonZeroU16 {}
unsafe impl StableLayout for NonZeroI16 {}
unsafe impl StableLayout for NonZeroU32 {}
unsafe impl StableLayout for NonZeroI32 {}
unsafe impl StableLayout for NonZeroU64 {}
unsafe impl StableLayout for NonZeroI64 {}
unsafe impl StableLayout for NonZeroU128 {}
unsafe impl StableLayout for NonZeroI128 {}
unsafe impl StableLayout for NonZeroUsize {}
unsafe impl StableLayout for NonZeroIsize {}
unsafe impl<T: StableLayout, const N: usize> StableLayout for [T; N] {}
unsafe impl<T: StableLayout> StableLayout for MaybeUninit<T> {}
unsafe impl<T: 'static> StableLayout for *const T {}
unsafe impl<T: 'static> StableLayout for *mut T {}
unsafe impl<T: StableLayout> StableLayout for UnsafeCell<T> {}
unsafe impl<T: StableLayout> StableLayout for Cell<T> {}
unsafe impl<T: StableLayout> StableLayout for RefMut<'static, T> {}
unsafe impl<T: StableLayout> StableLayout for RefCell<T> {}
unsafe impl<T: StableLayout> StableLayout for OnceCell<T> {}
unsafe impl<T: StableLayout> StableLayout for LazyCell<T> {}
unsafe impl<T: StableLayout> StableLayout for ManuallyDrop<T> {}
unsafe impl<T: StableLayout> StableLayout for Wrapping<T> {}
unsafe impl<T: StableLayout> StableLayout for Saturating<T> {}
unsafe impl<T: 'static> StableLayout for PhantomData<T> {}
unsafe impl<T: StableLayout + NonNullable> StableLayout for MaybeNull<T> {}
unsafe impl<T: StableLayout> StableLayout for Option<T> {}

#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m128 {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m128bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m128d {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m128i {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m256 {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m256bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m256d {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m256i {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m512 {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m512bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m512d {}
#[cfg(target_arch = "x86_64")]
unsafe impl StableLayout for core::arch::x86_64::__m512i {}

#[cfg(feature = "half")]
unsafe impl StableLayout for half::f16 {}
#[cfg(feature = "half")]
unsafe impl StableLayout for half::bf16 {}

#[cfg(feature = "nightly_float")]
unsafe impl StableLayout for f16 {}
#[cfg(feature = "nightly_float")]
unsafe impl StableLayout for f128 {}

/// Marker trait for types subject to the null pointer optimization.
///
/// # Safety
///
/// If zeroed is a valid bitpattern, undefined behavior will occur
/// when trying to optimize memory.
pub unsafe trait NonNullable {}

unsafe impl NonNullable for () {}
unsafe impl NonNullable for NonZeroU8 {}
unsafe impl NonNullable for NonZeroI8 {}
unsafe impl NonNullable for NonZeroU16 {}
unsafe impl NonNullable for NonZeroI16 {}
unsafe impl NonNullable for NonZeroU32 {}
unsafe impl NonNullable for NonZeroI32 {}
unsafe impl NonNullable for NonZeroU64 {}
unsafe impl NonNullable for NonZeroI64 {}
unsafe impl NonNullable for NonZeroU128 {}
unsafe impl NonNullable for NonZeroI128 {}
unsafe impl NonNullable for NonZeroUsize {}
unsafe impl NonNullable for NonZeroIsize {}
unsafe impl<T> NonNullable for NonNull<T> {}
unsafe impl<T> NonNullable for &T {}
unsafe impl<T> NonNullable for &mut T {}

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

/// Marker trait for types that can be converted to/from bytes freely.
///
/// This doesn't mean any bitpattern would be valid for it, but it can be converted
/// to/from bytes without undefined behavior EVER occurring. different from POD by
/// enforcing that normal (non-atomic) operations are valid. This includes [`crate::ptr::read`]
/// and [`crate::ptr::write`].
///
/// # Safety
///
/// If the type isn't safe to write arbitrary bytes to or from it without atomics, it would
/// be unsound. Implementing this on a type that implements `Send` or `Sync` is usually a bad
/// idea unless it guarantees exclusivity even when not held.
pub unsafe trait RawConvert {}

unsafe impl RawConvert for () {}
unsafe impl RawConvert for u8 {}
unsafe impl RawConvert for i8 {}
unsafe impl RawConvert for u16 {}
unsafe impl RawConvert for i16 {}
unsafe impl RawConvert for u32 {}
unsafe impl RawConvert for i32 {}
unsafe impl RawConvert for u64 {}
unsafe impl RawConvert for i64 {}
unsafe impl RawConvert for u128 {}
unsafe impl RawConvert for i128 {}
unsafe impl RawConvert for isize {}
unsafe impl RawConvert for usize {}
unsafe impl RawConvert for f32 {}
unsafe impl RawConvert for f64 {}
unsafe impl RawConvert for bool {}
unsafe impl RawConvert for NonZeroU8 {}
unsafe impl RawConvert for NonZeroI8 {}
unsafe impl RawConvert for NonZeroU16 {}
unsafe impl RawConvert for NonZeroI16 {}
unsafe impl RawConvert for NonZeroU32 {}
unsafe impl RawConvert for NonZeroI32 {}
unsafe impl RawConvert for NonZeroU64 {}
unsafe impl RawConvert for NonZeroI64 {}
unsafe impl RawConvert for NonZeroU128 {}
unsafe impl RawConvert for NonZeroI128 {}
unsafe impl RawConvert for NonZeroUsize {}
unsafe impl RawConvert for NonZeroIsize {}
unsafe impl<T> RawConvert for NonNull<T> {}
unsafe impl<T: RawConvert, const N: usize> RawConvert for [T; N] {}
unsafe impl<T: RawConvert> RawConvert for Option<T> {}
unsafe impl<T: RawConvert> RawConvert for UnsafeCell<T> {}
unsafe impl<T: RawConvert> RawConvert for Cell<T> {}
unsafe impl<T: RawConvert> RawConvert for ManuallyDrop<T> {}
unsafe impl<T: RawConvert> RawConvert for Wrapping<T> {}
unsafe impl<T: RawConvert> RawConvert for Saturating<T> {}
unsafe impl<T> RawConvert for PhantomData<T> {}
unsafe impl<T: RawConvert + NonNullable> RawConvert for MaybeNull<T> {}

#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m128 {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m128bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m128d {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m128i {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m256 {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m256bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m256d {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m256i {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m512 {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m512bh {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m512d {}
#[cfg(target_arch = "x86_64")]
unsafe impl RawConvert for core::arch::x86_64::__m512i {}

#[cfg(feature = "half")]
unsafe impl RawConvert for half::f16 {}
#[cfg(feature = "half")]
unsafe impl RawConvert for half::bf16 {}

#[cfg(feature = "nightly_float")]
unsafe impl RawConvert for f16 {}
#[cfg(feature = "nightly_float")]
unsafe impl RawConvert for f128 {}

/// POD marker trait for *Plain Old Data*.
///
/// # Safety
///
/// - All bit patterns of `T` must be valid
/// - `T` must have no padding or initialized padding
/// - `T` must implement [`StableLayout`] + [`RawConvert`]
///
/// Violating any of these constraints is bound to cause undefined behavior.
pub unsafe trait Pod: StableLayout + RawConvert {}

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
unsafe impl<T: 'static> Pod for PhantomData<T> {}
unsafe impl<T: CompilerAssumedNonNullable + RawConvert + StableLayout> Pod for Option<T> {}

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

/// Internal trait used to determine what types are safe to cast.
///
/// If a type is safe to cast to another type, but not any type, this should be used
/// instead of [`Pod`].
///
/// # Safety
///
/// This is less strict and is safe as long as it is safe to reinterpret the bytes
/// of any given `T` as a `U`. It isn't required, however, that any `U` can be
/// reinterpreted as a `T`. It is also completely unrelated to all other types.
pub unsafe trait Layout<T: StableLayout>: StableLayout {}

unsafe impl<T: Pod, U: Pod> Layout<U> for T {}

unsafe impl Layout<NonZeroI128> for NonZeroU128 {}
unsafe impl Layout<NonZeroU128> for NonZeroI128 {}

unsafe impl Layout<NonZeroI64> for NonZeroU64 {}
unsafe impl Layout<NonZeroU64> for NonZeroI64 {}

unsafe impl Layout<NonZeroI32> for NonZeroU32 {}
unsafe impl Layout<NonZeroU32> for NonZeroI32 {}

unsafe impl Layout<NonZeroI16> for NonZeroU16 {}
unsafe impl Layout<NonZeroU16> for NonZeroI16 {}

unsafe impl Layout<NonZeroI8> for NonZeroU8 {}
unsafe impl Layout<NonZeroU8> for NonZeroI8 {}

unsafe impl Layout<NonZeroUsize> for NonZeroIsize {}
unsafe impl Layout<NonZeroIsize> for NonZeroUsize {}

#[cfg(target_pointer_width = "64")]
unsafe impl Layout<NonZeroI64> for NonZeroUsize {}
#[cfg(target_pointer_width = "64")]
unsafe impl Layout<NonZeroI64> for NonZeroIsize {}

#[cfg(target_pointer_width = "64")]
unsafe impl Layout<NonZeroU64> for NonZeroUsize {}
#[cfg(target_pointer_width = "64")]
unsafe impl Layout<NonZeroU64> for NonZeroIsize {}

#[cfg(target_pointer_width = "32")]
unsafe impl Layout<NonZeroI32> for NonZeroUsize {}
#[cfg(target_pointer_width = "32")]
unsafe impl Layout<NonZeroI32> for NonZeroIsize {}

#[cfg(target_pointer_width = "32")]
unsafe impl Layout<NonZeroU32> for NonZeroUsize {}
#[cfg(target_pointer_width = "32")]
unsafe impl Layout<NonZeroU32> for NonZeroIsize {}

#[cfg(target_pointer_width = "16")]
unsafe impl Layout<NonZeroI16> for NonZeroUsize {}
#[cfg(target_pointer_width = "16")]
unsafe impl Layout<NonZeroI16> for NonZeroIsize {}

#[cfg(target_pointer_width = "16")]
unsafe impl Layout<NonZeroU16> for NonZeroUsize {}
#[cfg(target_pointer_width = "16")]
unsafe impl Layout<NonZeroU16> for NonZeroIsize {}
