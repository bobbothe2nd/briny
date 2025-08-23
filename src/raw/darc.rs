//! The module which holds the non-allocating, reference-based `Arc` alternative, `Darc`. Unlike `Darc`, `Darc` contains a reference.
//!
//! This makes it safer in what it is capable of, but less versatile in what it can do in general.

use crate::raw::cell::NotUnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

/// The type held within a valid `Darc`.
///
/// Does not support weak reference counting.
#[derive(Debug)]
#[repr(C)]
pub struct DarcInner<T> {
    ref_count: AtomicUsize,
    data: NotUnsafeCell<T>,
}

impl<T> DarcInner<T> {
    /// Method to create a new `DarcInner` with the data wrapped in a `NotUnsafeCell`.
    pub const fn new(data: T) -> Self {
        Self {
            ref_count: AtomicUsize::new(1),
            data: NotUnsafeCell::new(data),
        }
    }

    /// Method to create a new `DarcInner` with the data exactly as provided.
    pub const fn from_cell(data: NotUnsafeCell<T>) -> Self {
        Self {
            ref_count: AtomicUsize::new(1),
            data,
        }
    }
}

#[derive(Debug)]
/// A Direct Atomically Reference-Counted structure.
///
/// Holds a reference to the inner value.
pub struct Darc<'a, T> {
    inner: &'a DarcInner<T>,
}

unsafe impl<T: Send + Sync> Send for Darc<'_, T> {}
unsafe impl<T: Send + Sync> Sync for Darc<'_, T> {}

impl<'a, T> Darc<'a, T> {
    /// A constructor that takes `MaybeUninit<DarcInner>` and constructs `Darc`.
    ///
    /// # Safety
    ///
    /// The function is unsafe unless it can be guaranteed that `inner` outlives all clones of this `Naarc`.
    ///
    /// The constructor is often less safe than `from_raw` (or at least as unsafe), taking a `MaybeUninit` instead of `DarcInner`, adding another layer of possible unsafety.
    pub const unsafe fn new(inner: &'a mut core::mem::MaybeUninit<DarcInner<T>>, value: T) -> Self {
        unsafe {
            let ptr = inner.as_mut_ptr();
            ptr.write(DarcInner::new(value));
            Self::from_raw(ptr)
        }
    }

    /// Create a new `Darc` from a `DarcInner` and initialize `ref_count` to 1.
    pub fn from_inner(inner: &'a DarcInner<T>) -> Self {
        inner.ref_count.store(1, Ordering::Relaxed);
        Self { inner }
    }

    /// Create a new `Darc` from a pointer to a `DaarcInner`
    ///
    /// # Safety
    ///
    /// The pointer must be valid and properly reference counted.
    pub const unsafe fn from_raw(ptr: *const DarcInner<T>) -> Self {
        Self {
            inner: unsafe { &*ptr },
        }
    }

    /// Return a raw pointer to the inner value, consuming `self` in the process.
    #[must_use]
    pub const fn into_raw(self) -> *const DarcInner<T> {
        let ptr = self.inner as *const DarcInner<T>;
        core::mem::forget(self);
        ptr
    }

    /// Attempt to get the inner data mutably.
    ///
    /// # Errors
    ///
    /// When the strong reference counter is not at 1, a `None` is returned.
    ///
    /// # Safety
    ///
    /// By Rusts ownership model, this is unsafe unless there is no other `Darc` alive.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.strong_count() == 1 {
            unsafe { Some(&mut *self.inner.data.get_mut()) }
        } else {
            None
        }
    }

    #[must_use]
    /// Return the atomic value stored in `ref_count` as `isize`.
    pub fn strong_count(&self) -> usize {
        self.inner.ref_count.load(Ordering::Acquire)
    }

    /// Attempt to unwrap and return the inner value.
    ///
    /// # Errors
    ///
    /// Softly fails and returns `self` when the strong count is not equal to 1.
    pub fn try_unwrap(self) -> Result<T, Self> {
        if self.strong_count() == 1 {
            let inner = self.inner;
            // SAFETY: sole owner, can move out the data
            let data = unsafe { core::ptr::read(inner.data.get()) };
            core::mem::forget(self);
            Ok(data)
        } else {
            Err(self)
        }
    }
}

impl<T> Clone for Darc<'_, T> {
    fn clone(&self) -> Self {
        self.inner.ref_count.fetch_add(1, Ordering::Relaxed);
        Self { inner: self.inner }
    }
}

impl<T> Drop for Darc<'_, T> {
    fn drop(&mut self) {
        let prev = self.inner.ref_count.fetch_sub(1, Ordering::Release);
        debug_assert!(prev > 0);
        if prev == 1 {
            core::sync::atomic::fence(Ordering::Acquire);
        }
    }
}

impl<T> core::ops::Deref for Darc<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.data.get() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo {
        x: usize,
    }

    fn init_darc() -> Darc<'static, Foo> {
        use core::mem::MaybeUninit;
        static mut INNER: MaybeUninit<DarcInner<Foo>> = MaybeUninit::uninit();

        // SAFETY: we're never aliasing or dropping the same memory twice
        unsafe {
            let arc = Darc::new((&raw mut INNER).as_mut().unwrap(), Foo { x: 42 });
            arc
        }
    }

    #[test]
    fn clone_and_refcount() {
        let a = init_darc();
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
        let mut a = init_darc();
        {
            let r = a.get_mut().unwrap();
            r.x = 42;
        }
        assert_eq!(a.get_mut().unwrap().x, 42);
    }

    #[test]
    fn mut_then_shared_fails() {
        let mut a = init_darc();
        let _r = a.get_mut().unwrap();
        assert!(a.get_mut().is_some());
    }

    #[test]
    fn multiple_clones_borrow_should_fail() {
        let mut a = init_darc();
        let b = a.clone();
        let _c = b.clone();

        assert!(a.get_mut().is_none());
    }

    #[test]
    fn drop_last_ref_fence_only() {
        let a = init_darc();
        let b = a.clone();
        assert_eq!(a.strong_count(), 2);
        drop(b);
        assert_eq!(a.strong_count(), 1);
        drop(a);
        // Cannot verify memory free (by design), but no UB
    }
}
