use crate::{private::Private, traits::NonNullable};
use core::mem::MaybeUninit;

/// Thin wrapper over `T` that safely checks when it is initialized that implements copy.
#[derive(Debug, Clone, Copy)]
pub struct MaybeNullCopy<T: NonNullable + Copy> {
    inner: MaybeUninit<T>,
}

impl<T: NonNullable + Copy> Private for MaybeNullCopy<T> {}

impl<T: NonNullable + Copy> MaybeNullCopy<T> {
    /// Creates a zeroed bitpattern the size of a value.
    #[inline(always)]
    #[must_use]
    pub const fn null() -> Self {
        Self {
            inner: MaybeUninit::zeroed(),
        }
    }

    /// Wraps a value.
    #[inline(always)]
    #[must_use]
    pub const fn new(val: T) -> Self {
        Self {
            inner: MaybeUninit::new(val),
        }
    }

    /// Returns true if it is null.
    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        let ptr = self.inner.as_ptr().cast::<u8>();
        let mut i = 0;

        while i < size_of::<T>() {
            if unsafe { ptr.add(i).read() } != 0 {
                return false;
            }

            i += 1;
        }

        true
    }

    /// Returns false if it is null.
    #[inline(always)]
    pub const fn is_init(&self) -> bool {
        let ptr = self.inner.as_ptr().cast::<u8>();
        let mut i = 0;

        while i < size_of::<T>() {
            if unsafe { ptr.add(i).read() } == 0 {
                return false;
            }

            i += 1;
        }

        true
    }

    /// Tries to convert this into an owned value.
    #[inline(always)]
    pub fn into_inner(self) -> Option<T> {
        if self.is_init() {
            unsafe { Some(self.into_inner_unchecked()) }
        } else {
            None
        }
    }

    /// Convert this into an owned value without checking if its initialized.
    ///
    /// # Safety
    ///
    /// This does not check if the value returned is valid and could be null.
    #[inline(always)]
    pub unsafe fn into_inner_unchecked(self) -> T {
        unsafe { self.inner.assume_init_read() }
    }

    /// Attempts to get a reference to the value.
    #[inline(always)]
    pub const fn get(&self) -> Option<&T> {
        if self.is_init() {
            unsafe { Some(self.get_unchecked()) }
        } else {
            None
        }
    }

    /// Attempts to get a mutable reference to the value.
    #[inline(always)]
    pub const fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_init() {
            unsafe { Some(self.get_mut_unchecked()) }
        } else {
            None
        }
    }

    /// Gets a reference without checking if its initialized.
    ///
    /// # Safety
    ///
    /// This does not check if the value returned is valid and could be null.
    #[inline(always)]
    pub const unsafe fn get_unchecked(&self) -> &T {
        unsafe { self.inner.assume_init_ref() }
    }

    /// Gets a mutable reference without checking if its initialized.
    ///
    /// # Safety
    ///
    /// This does not check if the value returned is valid and could be null.
    #[inline(always)]
    pub const unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        unsafe { self.inner.assume_init_mut() }
    }

    /// Gets a constant pointer to the value.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.inner.as_ptr()
    }

    /// Gets a mutable pointer to the value.
    #[inline(always)]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.inner.as_mut_ptr()
    }

    /// Attempts to set the value.
    ///
    /// Returns whether it successfully did or not.
    ///
    /// It fails if it is already initialized.
    #[inline(always)]
    pub const fn set(&mut self, val: T) -> bool {
        if self.is_null() {
            unsafe {
                self.inner.as_mut_ptr().write(val);
            }
            true
        } else {
            false
        }
    }

    /// Forces a change to the value.
    #[inline(always)]
    pub const fn force_set(&mut self, val: T) {
        unsafe {
            self.inner.as_mut_ptr().write(val);
        }
    }

    /// Sets the value to null.
    #[inline(always)]
    pub const fn nullify(&mut self) {
        if self.is_init() {
            unsafe {
                core::ptr::write_bytes(self.inner.as_mut_ptr().cast::<u8>(), 0, size_of::<T>());
            }
        }
    }

    /// Sets the value to null.
    #[inline(always)]
    pub const unsafe fn nullify_unchecked(&mut self) {
        unsafe {
            core::ptr::write_bytes(self.inner.as_mut_ptr().cast::<u8>(), 0, size_of::<T>());
        }
    }

    /// Matches the value over two callbacks.
    ///
    /// If initialized, call `if_init`. If null, call `if_null`.
    pub fn match_null_ref<R>(
        &self,
        if_init: impl FnOnce(&T) -> R,
        if_null: impl FnOnce() -> R,
    ) -> R {
        if self.is_init() {
            unsafe { if_init(self.get_unchecked()) }
        } else {
            if_null()
        }
    }

    /// Matches the value over two callbacks.
    ///
    /// If initialized, call `if_init`. If null, call `if_null`.
    pub fn match_null_mut<R>(
        &mut self,
        if_init: impl FnOnce(&mut T) -> R,
        if_null: impl FnOnce() -> R,
    ) -> R {
        if self.is_init() {
            unsafe { if_init(self.get_mut_unchecked()) }
        } else {
            if_null()
        }
    }

    /// Matches the value over two callbacks.
    ///
    /// If initialized, call `if_init`. If null, call `if_null`.
    pub fn match_null<R>(self, if_init: impl FnOnce(T) -> R, if_null: impl FnOnce() -> R) -> R {
        if self.is_init() {
            unsafe { if_init(self.into_inner_unchecked()) }
        } else {
            if_null()
        }
    }
}
