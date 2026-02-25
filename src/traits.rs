//! Traits to abstract common characteristics among types.

use core::{cell::{Cell, LazyCell, OnceCell, RefCell, RefMut, UnsafeCell}, marker::PhantomData, mem::{ManuallyDrop, MaybeUninit}, num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping}, pin::Pin, ptr::NonNull, sync::atomic::{AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize}};

/// A simple marker trait for types that have a consistent layout in memory.
pub unsafe trait StableLayout: 'static {}

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
unsafe impl<T: 'static> StableLayout for PhantomData<T> {}

/// Marker trait for types that are NOT interiorly mutable.
///
/// This complements [`Writable`] in the sense that any type which
/// cannot be mutated through a shared reference should implement this,
/// and every other type should implement it's complement. Anything
/// which either implements both or implements neither can be considered
/// a logic error (or undefined behavior in the case of the former).
pub unsafe trait InteriorImmutable {}

unsafe impl InteriorImmutable for u8 {}
unsafe impl InteriorImmutable for i8 {}
unsafe impl InteriorImmutable for u16 {}
unsafe impl InteriorImmutable for i16 {}
unsafe impl InteriorImmutable for u32 {}
unsafe impl InteriorImmutable for i32 {}
unsafe impl InteriorImmutable for u64 {}
unsafe impl InteriorImmutable for i64 {}
unsafe impl InteriorImmutable for u128 {}
unsafe impl InteriorImmutable for i128 {}
unsafe impl InteriorImmutable for isize {}
unsafe impl InteriorImmutable for usize {}
unsafe impl InteriorImmutable for f32 {}
unsafe impl InteriorImmutable for f64 {}
unsafe impl InteriorImmutable for bool {}
unsafe impl InteriorImmutable for NonZeroU8 {}
unsafe impl InteriorImmutable for NonZeroI8 {}
unsafe impl InteriorImmutable for NonZeroU16 {}
unsafe impl InteriorImmutable for NonZeroI16 {}
unsafe impl InteriorImmutable for NonZeroU32 {}
unsafe impl InteriorImmutable for NonZeroI32 {}
unsafe impl InteriorImmutable for NonZeroU64 {}
unsafe impl InteriorImmutable for NonZeroI64 {}
unsafe impl InteriorImmutable for NonZeroU128 {}
unsafe impl InteriorImmutable for NonZeroI128 {}
unsafe impl InteriorImmutable for NonZeroUsize {}
unsafe impl InteriorImmutable for NonZeroIsize {}
unsafe impl<T: InteriorImmutable, const N: usize> InteriorImmutable for [T; N] {}
unsafe impl<T: InteriorImmutable> InteriorImmutable for MaybeUninit<T> {}
unsafe impl<T: InteriorImmutable> InteriorImmutable for Pin<T> {}
unsafe impl<T: InteriorImmutable> InteriorImmutable for ManuallyDrop<T> {}
unsafe impl<T: InteriorImmutable> InteriorImmutable for Wrapping<T> {}
unsafe impl<T> InteriorImmutable for PhantomData<T> {}

/// Any type that is interiorly mutable.
///
/// This is the complement of [`InteriorImmutable`] as described.
pub unsafe trait Writable {}

unsafe impl Writable for AtomicU8 {}
unsafe impl Writable for AtomicI8 {}
unsafe impl Writable for AtomicU16 {}
unsafe impl Writable for AtomicI16 {}
unsafe impl Writable for AtomicU32 {}
unsafe impl Writable for AtomicI32 {}
unsafe impl Writable for AtomicU64 {}
unsafe impl Writable for AtomicI64 {}
unsafe impl Writable for AtomicUsize {}
unsafe impl Writable for AtomicIsize {}
unsafe impl Writable for AtomicBool {}
unsafe impl<T: Writable> Writable for [T] {}
unsafe impl<T: Writable, const N: usize> Writable for [T; N] {}
unsafe impl<T: Writable> Writable for MaybeUninit<T> {}
unsafe impl<T> Writable for UnsafeCell<T> {}
unsafe impl<T> Writable for Cell<T> {}
unsafe impl<T> Writable for RefMut<'_, T> {}
unsafe impl<T> Writable for RefCell<T> {}
unsafe impl<T> Writable for OnceCell<T> {}
unsafe impl<T> Writable for LazyCell<T> {}
unsafe impl<T: Writable> Writable for ManuallyDrop<T> {}
unsafe impl<T: Writable> Writable for Wrapping<T> {}
unsafe impl<T> Writable for NonNull<T> {}
unsafe impl<T> Writable for *mut T {}
unsafe impl<T> Writable for &mut T {}

/// Marker trait for types subject to the null pointer optimization.
///
/// # Safety
///
/// If `zeroed` is a valid bitpattern, undefined behavior will occur
/// when trying to optimize memory.
pub unsafe trait NonNullable {}

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

/// Marker trait for types that can be converted to/from bytes freely.
/// 
/// This doesn't mean any bitpattern would be valid for it, but it can be converted
/// to/from bytes without undefined behavior EVER occurring. different from POD by
/// enforcing that normal (non-atomic) operations are valid. This includes [`crate::ptr::read`],
/// [`crate::ptr::write`], and all the [`core`] counterparts.
///
/// # Safety
///
/// If the type isn't safe to write arbitrary bytes to or from it without atomics, it would
/// be unsound. Implementing this on a type that implements `Send` or `Sync` is usually a bad
/// idea unless it guarantees exclusivity even when not held.
pub unsafe trait RawConvert {}

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
unsafe impl<T: RawConvert> RawConvert for MaybeUninit<T> {}
unsafe impl<T: RawConvert> RawConvert for Option<T> {}
unsafe impl<T> RawConvert for &T {}
unsafe impl<T> RawConvert for &mut T {}
unsafe impl<T> RawConvert for *const T {}
unsafe impl<T> RawConvert for *mut T {}
unsafe impl<T: RawConvert> RawConvert for UnsafeCell<T> {}
unsafe impl<T: RawConvert> RawConvert for Cell<T> {}
unsafe impl<T: RawConvert> RawConvert for RefCell<T> {}
unsafe impl<T: RawConvert> RawConvert for RefMut<'_, T> {}
unsafe impl<T: RawConvert> RawConvert for LazyCell<T> {}
unsafe impl<T: RawConvert> RawConvert for OnceCell<T> {}
unsafe impl<T: RawConvert> RawConvert for ManuallyDrop<T> {}
unsafe impl<T: RawConvert> RawConvert for Wrapping<T> {}
unsafe impl<T> RawConvert for PhantomData<T> {}

/// Marker trait for types aligned exactly to one byte.
///
/// # Safety
///
/// If the type isn't aligned to one byte, undefined behavior
/// could theoretically occur - though it is still unlikely.
/// Albeit, implementing this on strictly aligned types is
/// considered unsound.
///
/// This can be done via the `repr(packed(1))` attribute on
/// structures. Note that `repr(align(1))` will NOT align the
/// structure to one byte, because Rust is free to change anything
/// that will make code safer if it isn't explicitly enforced.
pub unsafe trait Unaligned {}

unsafe impl Unaligned for u8 {}
unsafe impl Unaligned for i8 {}
unsafe impl Unaligned for bool {}
unsafe impl Unaligned for AtomicU8 {}
unsafe impl Unaligned for AtomicI8 {}
unsafe impl Unaligned for AtomicBool {}
unsafe impl Unaligned for NonZeroU8 {}
unsafe impl Unaligned for NonZeroI8 {}
unsafe impl<T: Unaligned> Unaligned for Option<T> {}
unsafe impl<T: Unaligned, const N: usize> Unaligned for [T; N] {}
unsafe impl<T: Unaligned> Unaligned for [T] {}
unsafe impl<T: Unaligned> Unaligned for MaybeUninit<T> {}
unsafe impl<T: Unaligned> Unaligned for UnsafeCell<T> {}
unsafe impl<T: Unaligned> Unaligned for Cell<T> {}
unsafe impl<T: Unaligned> Unaligned for ManuallyDrop<T> {}
unsafe impl<T: Unaligned> Unaligned for Wrapping<T> {}
unsafe impl<T> Unaligned for PhantomData<T> {}

/// POD marker trait for *Plain Old Data*.
///
/// # Safety
///
/// - All bit patterns of `T` must be valid
/// - `T` must have no padding or initialized padding
/// - `T` must implement [`StableLayout`] + [`RawConvert`]
///
/// Violating any of these constraints is bound to cause undefined behavior or
/// compiler errors (especially if derived).
pub unsafe trait Pod: StableLayout + RawConvert {}

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
unsafe impl<T: Pod> Pod for MaybeUninit<T> {}
unsafe impl<T: Pod> Pod for UnsafeCell<T> {}
unsafe impl<T: Pod> Pod for Cell<T> {}
unsafe impl<T: Pod> Pod for ManuallyDrop<T> {}
unsafe impl<T: Pod> Pod for Wrapping<T> {}
unsafe impl<T: 'static> Pod for PhantomData<T> {}
unsafe impl<T: 'static> Pod for *const T {}
unsafe impl<T: 'static> Pod for *mut T {}
