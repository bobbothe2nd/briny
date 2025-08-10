//! The module which holds the non-allocating `Arc` alternative, `Naarc`. Unlike `Darc`, `Naarc` contains a raw pointer.
//!
//! It is by no means a drop-in replacement, between safety and completeness, it doesn't even scratch the surface of what `Arc` does. However, in `#![no_std]` environments where dynamic allocation is not available or too costly, `Naarc` can be a good substitute.
//!
//! ---
//!
//! Note: This can unexpectedly result in dangling pointers and cause dangerous undefined behavior - it occurs very often if not correctly constructed, therefore it is impossible to *safely* construct `Naarc`.
//! If safety is important, try `Darc` instead, which holds the actual inner value and not just a pointer - but that sacrifices some capabilities of `Naarc` which is closer to `alloc`'s `Arc`.

use crate::raw::cell::NotUnsafeCell;
use crate::raw::ptr::ImpConst;
use core::sync::atomic::{AtomicUsize, Ordering};

/// The type pointed to by a valid `Naarc`.
///
/// Does not support weak reference counting.
#[repr(C)]
#[derive(Debug)]
pub struct NaarcInner<T> {
    ref_count: AtomicUsize,
    data: NotUnsafeCell<T>,
}

impl<T> NaarcInner<T> {
    /// Method to create a new `NaarcInner` with the borrow counter at `ref_count` and `data` wrapped in a `NotUnsafeCell`.
    pub const fn new(ref_count: usize, data: T) -> Self {
        Self {
            ref_count: AtomicUsize::new(ref_count),
            data: NotUnsafeCell::new(data),
        }
    }

    /// Method to create a new `NaarcInner` with the borrow counter at `ref_count` and the data exactly as provided.
    pub const fn from_cell(ref_count: usize, data: NotUnsafeCell<T>) -> Self {
        Self {
            ref_count: AtomicUsize::new(ref_count),
            data,
        }
    }

    /// Method to check if the `NaarcInner` has yet to be dropped.
    pub fn is_dropped(&self) -> bool {
        self.ref_count.load(Ordering::Acquire) == 0
    }
}

/// A Non-Allocating Atomically Reference-Counted structure.
///
/// Holds a pointer to the inner value.
#[repr(C)]
#[derive(Debug)]
pub struct Naarc<'a, T> {
    inner: ImpConst<'a, NaarcInner<T>>,
}

unsafe impl<T: Send> Send for Naarc<'_, T> {}
unsafe impl<T: Sync> Sync for Naarc<'_, T> {}

impl<'a, T> Naarc<'a, T> {
    /// Create a new `Naarc` by initializing a `NaarcInner<T>` in place.
    ///
    /// # Safety
    ///
    /// The function is unsafe unless it can be guaranteed that `inner` outlives all clones of this `Naarc`.
    #[must_use]
    pub const unsafe fn new(
        inner: &'a mut core::mem::MaybeUninit<NaarcInner<T>>,
        value: T,
    ) -> Self {
        unsafe {
            inner.as_mut_ptr().write(NaarcInner {
                ref_count: AtomicUsize::new(1),
                data: NotUnsafeCell::new(value),
            });
            Self {
                inner: ImpConst::from_ptr(inner.as_ptr()),
            }
        }
    }

    /// Create a new `Naarc` from a mutable reference to a `NaarcInner<T>`.
    /// Initializes `ref_count` to 1.
    ///
    /// # Safety
    ///
    /// The function is unsafe unless it can be guaranteed that `inner` outlives all clones of this `Naarc`.
    pub unsafe fn from_inner(inner: &'a mut NaarcInner<T>) -> Self {
        inner.ref_count.store(1, Ordering::Relaxed);
        Self {
            inner: ImpConst::new(inner),
        }
    }

    /// Return a raw pointer to the inner value, consuming `self` in the process.
    #[must_use]
    pub const fn into_raw(self) -> *const NaarcInner<T> {
        let ptr = self.inner.as_ptr();
        core::mem::forget(self);
        ptr
    }

    /// Creates a new `Naarc` from a raw pointer to a `NaarcInner`.
    ///
    /// # Safety
    ///
    /// As long as the pointer is valid and `inner` outlives all existing clones, the constructor is safe.
    pub const unsafe fn from_raw(ptr: *const NaarcInner<T>) -> Self {
        Self {
            inner: unsafe { ImpConst::from_ptr(ptr) },
        }
    }

    /// Attempt to get a mutable reference to the inner value.
    ///
    /// # Safety
    ///
    /// Assuming `Naarc` was correctly constructed and the pointer is valid, this function is safe.
    ///
    /// If it wasn't correctly constructed, undefined behavior is bound to occur.
    ///
    /// # Errors
    ///
    /// If the strong count is not equal to 1, `None` is returned.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.strong_count() == 1 {
            unsafe { Some(&mut *(*self.inner.as_ptr()).data.get().cast_mut()) }
        } else {
            None
        }
    }

    /// Force `Naarc` into `&mut T` and panic on failure.
    ///
    /// # Safety
    ///
    /// Assuming `Naarc` was correctly constructed and the pointer is valid, this function is safe.
    ///
    /// If it wasn't correctly constructed, undefined behavior is bound to occur.
    ///
    /// # Panics
    ///
    /// When the strong count is not 0, a hard panic occurs.
    pub fn make_mut(&mut self) -> &mut T
    where
        T: Clone,
    {
        assert!(
            self.strong_count() == 1,
            "make_mut not supported for Naarc without heap allocation"
        );
        #[allow(clippy::unwrap_used)]
        self.get_mut().unwrap()
    }

    /// Provides a pointer to the data stored inside.
    ///
    /// # Safety
    ///
    /// Assuming `Naarc` was correctly constructed and the pointer is valid, this function is safe.
    ///
    /// If it wasn't correctly constructed, undefined behavior is bound to occur.
    ///
    /// # Safety
    ///
    /// Assuming `Naarc` was correctly constructed and the pointer is valid, this function is safe.
    ///
    /// If it wasn't correctly constructed, undefined behavior is bound to occur.
    #[must_use]
    pub const fn as_ptr(&self) -> *const T {
        unsafe { (*self.inner.as_ptr()).data.get() }
    }

    /// Read the inner data and return the results.
    ///
    /// # Errors
    ///
    /// If the strong count is not precisely equal to 1, it is utterly unsafe to unwrap.
    ///
    /// # Safety
    ///
    /// Assuming `Naarc` was correctly constructed and the pointer is valid, this function is safe.
    ///
    /// If it wasn't correctly constructed, undefined behavior is bound to occur.
    pub fn try_unwrap(self) -> Result<T, Self> {
        if self.strong_count() == 1 {
            // SAFETY: sole owner, can consume data
            let inner = unsafe { &*self.inner.as_ptr() };
            // move data out, avoiding double drop
            // careful: NotUnsafeCell, use ptr::read to avoid double drop
            let data = unsafe { core::ptr::read(inner.data.get()) };
            core::mem::forget(self);
            Ok(data)
        } else {
            Err(self)
        }
    }

    /// Load the reference counter as a non-atomic.
    ///
    /// # Safety
    ///
    /// Assuming `Naarc` was correctly constructed and the pointer is valid, this function is safe.
    ///
    /// If it wasn't correctly constructed, undefined behavior is bound to occur.
    #[must_use]
    pub fn strong_count(&self) -> usize {
        unsafe { (*self.inner.as_ptr()).ref_count.load(Ordering::Acquire) }
    }
}

impl<T> Clone for Naarc<'_, T> {
    fn clone(&self) -> Self {
        let inner = self.inner.as_ptr();

        unsafe {
            let count = &(*inner).ref_count;
            let old = count.fetch_add(1, Ordering::Relaxed);
            debug_assert!(old > 0, "cloning Naarc with invalid ref count");
        }

        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Drop for Naarc<'_, T> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.inner.as_ptr();
            let count = &(*inner).ref_count;
            let prev = count.fetch_sub(1, Ordering::Release);

            if prev == 1 {
                core::sync::atomic::fence(Ordering::Acquire);
                drop(core::ptr::read(inner));
            } else if prev == 0 {
                // This is UB and caused your stack overflows
                panic!("dropping Naarc with non-positive ref count");
            }
        }
    }
}

impl<T> core::ops::Deref for Naarc<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: assumes valid pointer and initialized data
        unsafe { &*(*self.inner.as_ptr()).data.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo {
        x: usize,
    }

    fn init_naarc() -> Naarc<'static, Foo> {
        use core::mem::MaybeUninit;
        static mut INNER: MaybeUninit<NaarcInner<Foo>> = MaybeUninit::uninit();

        // SAFETY: we're never aliasing or dropping the same memory twice
        unsafe {
            let arc = Naarc::new((&raw mut INNER).as_mut().unwrap(), Foo { x: 42 });
            arc
        }
    }

    #[test]
    fn clone_and_refcount() {
        let a = init_naarc();
        assert_eq!(a.strong_count(), 1);
        let b = a.clone();
        assert_eq!(a.strong_count(), 2);
        let c = b.clone();
        assert_eq!(a.strong_count(), 3);
        drop(c);
        assert_eq!(a.strong_count(), 2);
        drop(b);
        assert_eq!(a.strong_count(), 1);
    }

    #[test]
    fn mut_borrow() {
        let mut a = init_naarc();
        {
            let r = a.get_mut().unwrap();
            r.x = 42;
        }
        assert_eq!(a.get_mut().unwrap().x, 42);
    }

    #[test]
    fn mut_then_shared_fails() {
        let mut a = init_naarc();
        let _r = a.get_mut().unwrap();
        assert!(a.get_mut().is_some());
    }

    #[test]
    fn multiple_clones_borrow_should_fail() {
        let mut a = init_naarc();
        let b = a.clone();
        let _c = b.clone();

        assert!(a.get_mut().is_none());
    }

    #[test]
    fn drop_last_ref_fence_only() {
        let a = init_naarc();
        let b = a.clone();
        assert_eq!(a.strong_count(), 2);
        drop(b);
        assert_eq!(a.strong_count(), 1);
        drop(a);
        // Cannot verify memory free (by design), but no UB
    }
}
