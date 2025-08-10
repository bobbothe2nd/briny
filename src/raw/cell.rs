//! A safe wrapper over `UnsafeCell` called `NotUnsafeCell`.

use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicIsize, Ordering},
};

use crate::BrinyError;

#[derive(Debug)]
/// A non-unsafe cell that still allows interior mutable access.
///
/// This struct is *only* available on systems that support atomic operations.
pub struct NotUnsafeCell<T> {
    value: UnsafeCell<T>,
    borrow: AtomicIsize,
}

unsafe impl<T: Sync> Sync for NotUnsafeCell<T> {}
unsafe impl<T: Send> Send for NotUnsafeCell<T> {}

impl<T> NotUnsafeCell<T> {
    /// Create a new `NotUnsafeCell`
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            borrow: AtomicIsize::new(0),
        }
    }

    /// Provide the inner value, consuming `self` in the process.
    ///
    /// # Safety
    ///
    /// Since the cell is consumed, there is nothing to borrow and hence no unsafety possible.
    pub fn into_inner(self) -> T {
        unsafe { core::ptr::read(self.value.get()) }
    }

    /// Try to immutably borrow the inner value.
    ///
    /// # Errors
    ///
    /// `BrinyError` is thrown such a case that the borrow counter is less than 0.
    pub fn try_borrow(&self) -> Result<NotUnsafeRef<'_, T>, BrinyError> {
        let result = self
            .borrow
            .fetch_update(Ordering::Acquire, Ordering::Relaxed, |count| {
                if count >= 0 { Some(count + 1) } else { None }
            });

        match result {
            Ok(_) => Ok(NotUnsafeRef { cell: self }),
            Err(_) => Err(BrinyError),
        }
    }

    /// Try to mutably borrow the inner value.
    ///
    /// # Errors
    ///
    /// If writing to the borrow counter fails, `BrinyError` is returned.
    pub fn try_borrow_mut(&self) -> Result<NotUnsafeRefMut<'_, T>, BrinyError> {
        match self
            .borrow
            .compare_exchange(0, -1, Ordering::Acquire, Ordering::Relaxed)
        {
            Ok(_) => Ok(NotUnsafeRefMut { cell: self }),
            Err(_) => Err(BrinyError),
        }
    }

    /// Force an immutable borrow out of the inner value.
    ///
    /// # Panics
    ///
    /// If borrowing fails, i.e. borrow counter is less than 0, a hard panic occurs with a static message.
    #[must_use]
    pub fn borrow(&self) -> NotUnsafeRef<'_, T> {
        #[allow(clippy::expect_used)]
        self.try_borrow().expect("already mutably borrowed")
    }

    /// Force a mutable borrow out of the inner value.
    ///
    /// # Panics
    ///
    /// If borrowing fails, i.e. writing to the atomic borrow counter, a hard panic occurs with a static message.
    #[must_use]
    pub fn borrow_mut(&self) -> NotUnsafeRefMut<'_, T> {
        #[allow(clippy::expect_used)]
        self.try_borrow_mut().expect("already borrowed")
    }

    /// Get a safe shared reference without borrow checks.
    ///
    /// # Safety
    ///
    /// This is always safe because it's a shared reference.
    /// It does NOT enforce borrow rules, so use carefully.
    pub fn get_ref(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    /// Returns a constant pointer to the inner pointer within the `UnsafeCell`.
    ///
    /// # Safety
    ///
    /// An immutable pointer can't violate Rust aliasing rules unless cast to a mutable pointer..
    pub const fn get(&self) -> *const T {
        self.value.get()
    }

    /// Unsafe escape hatch to obtain a mutable pointer within the `UnsafeCell`.
    ///
    /// # Safety
    ///
    /// To guarantee safety when calling this function, the following must be true:
    ///
    /// - Never use-after-free, out-of-bounds, or dangling pointer access
    /// - Never circumvents borrow tracking logic
    /// - Doesn't violate Rust aliasing rules in general
    pub const unsafe fn get_mut(&self) -> *mut T {
        self.value.get()
    }

    /// Get the current state of the borrow
    pub(crate) fn debug_borrow_state(&self) -> isize {
        self.borrow.load(Ordering::Acquire)
    }
}

impl<T> Drop for NotUnsafeCell<T> {
    fn drop(&mut self) {
        let borrow_count = self.debug_borrow_state();
        assert!(
            borrow_count == 0,
            "NotUnsafeCell dropped while borrowed: count = {borrow_count}"
        );
    }
}

/// A shared borrow of a `NotUnsafeCell<T>`
pub struct NotUnsafeRef<'a, T> {
    cell: &'a NotUnsafeCell<T>,
}

impl<'a, T> NotUnsafeRef<'a, T> {
    /// Cast `T` to `U` via the provided closure.
    pub fn map<U>(self, f: impl FnOnce(&T) -> &U) -> NotUnsafeRef<'a, U> {
        let _inner_ref = f(&*self);
        let cell = (core::ptr::from_ref::<NotUnsafeCell<T>>(self.cell)).cast::<NotUnsafeCell<U>>();
        // forget self to avoid double-decrementing borrow count
        core::mem::forget(self);
        NotUnsafeRef {
            cell: unsafe { &*cell },
        }
    }
}

impl<T> core::ops::Deref for NotUnsafeRef<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.cell.value.get() }
    }
}

impl<T> Drop for NotUnsafeRef<'_, T> {
    fn drop(&mut self) {
        let prev = self.cell.borrow.fetch_sub(1, Ordering::Release);
        assert!(prev > 0);
    }
}

/// A mutable borrow of a `NotUnsafeCell<T>`
pub struct NotUnsafeRefMut<'a, T> {
    cell: &'a NotUnsafeCell<T>,
}

impl<'a, T> NotUnsafeRefMut<'a, T> {
    /// Casts `T` to `U` via the provided closure.
    pub fn map<U>(self, f: impl FnOnce(&mut T) -> &mut U) -> NotUnsafeRefMut<'a, U> {
        let _ptr = f(unsafe { &mut *self.cell.value.get() });
        let new_cell = NotUnsafeRefMut {
            cell: unsafe {
                &*(core::ptr::from_ref::<NotUnsafeCell<T>>(self.cell).cast::<NotUnsafeCell<U>>())
            },
        };
        core::mem::forget(self);
        new_cell
    }
}

impl<T> core::ops::Deref for NotUnsafeRefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.cell.value.get() }
    }
}

impl<T> core::ops::DerefMut for NotUnsafeRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.cell.value.get() }
    }
}

impl<T> Drop for NotUnsafeRefMut<'_, T> {
    fn drop(&mut self) {
        let prev = self.cell.borrow.swap(0, Ordering::Release);
        assert_eq!(prev, -1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Foo {
        value: usize,
    }

    #[test]
    fn shared_borrow() {
        let cell = NotUnsafeCell::new(42);
        let r1 = cell.try_borrow().unwrap();
        let r2 = cell.try_borrow().unwrap();
        assert_eq!(*r1, 42);
        assert_eq!(*r2, 42);
        assert_eq!(cell.debug_borrow_state(), 2);
        drop(r1);
        assert_eq!(cell.debug_borrow_state(), 1);
        drop(r2);
        assert_eq!(cell.debug_borrow_state(), 0);
    }

    #[test]
    fn mut_borrow() {
        let cell = NotUnsafeCell::new(42);
        {
            let mut r = cell.try_borrow_mut().unwrap();
            *r = 69;
        }
        assert_eq!(cell.debug_borrow_state(), 0);
        assert_eq!(*cell.try_borrow().unwrap(), 69);
    }

    #[test]
    fn shared_then_mut_borrow_fails() {
        let cell = NotUnsafeCell::new(42);
        let _r1 = cell.try_borrow().unwrap();
        assert!(cell.try_borrow_mut().is_err());
    }

    #[test]
    fn mut_then_shared_borrow_fails() {
        let cell = NotUnsafeCell::new(42);
        let _r = cell.try_borrow_mut().unwrap();
        assert!(cell.try_borrow().is_err());
    }

    #[test]
    fn map_shared() {
        let cell = NotUnsafeCell::new(Foo { value: 10 });
        let r = cell.try_borrow().unwrap();
        let mapped = r.map(|foo| &foo.value);
        assert_eq!(*mapped, 10);
    }

    #[test]
    fn map_mut() {
        let cell = NotUnsafeCell::new(Foo { value: 5 });
        {
            let r = cell.try_borrow_mut().unwrap();
            let mut mut_mapped = r.map(|f| &mut f.value);
            *mut_mapped = 99;
        }
        assert_eq!(cell.try_borrow().unwrap().value, 99);
    }

    #[test]
    fn get_mut_ptr_safety() {
        let cell = NotUnsafeCell::new(1234);
        let ptr = unsafe { cell.get_mut() };
        unsafe {
            assert_eq!(*ptr, 1234);
            *ptr = 5678;
        }
        assert_eq!(*cell.try_borrow().unwrap(), 5678);
    }
}
