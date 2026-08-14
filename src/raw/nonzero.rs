//! `Option` alternative that exploits zeroed bitpatterns for memory efficiency.

use core::mem::{ManuallyDrop, MaybeUninit};
use crate::traits::NonNullable;

/// Thin wrapper over `T` that safely checks when it is initialized.
pub struct MaybeNull<T: NonNullable> {
    inner: MaybeUninit<T>,
}

impl<T: NonNullable> MaybeNull<T> {
    /// Creates a zeroed bitpattern the size of a value.
    #[inline(always)]
    pub const fn null() -> Self {
        Self {
            inner: MaybeUninit::zeroed(),
        }
    }

    /// Wraps a value.
    #[inline(always)]
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
            unsafe {
                Some(self.into_inner_unchecked())
            }
        } else {
            None
        }
    }

    /// Convert this into an owned value without checking if its initialized.
    #[inline(always)]
    pub unsafe fn into_inner_unchecked(self) -> T {
        unsafe {
            ManuallyDrop::new(self).inner.assume_init_read()
        }
    }

    /// Attempts to get a reference to the value.
    #[inline(always)]
    pub const fn get(&self) -> Option<&T> {
        if self.is_init() {
            unsafe {
                Some(self.get_unchecked())
            }
        } else {
            None
        }
    }

    /// Attempts to get a mutable reference to the value.
    #[inline(always)]
    pub const fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_init() {
            unsafe {
                Some(self.get_mut_unchecked())
            }
        } else {
            None
        }
    }

    /// Gets a reference without checking if its initialized.
    #[inline(always)]
    pub const unsafe fn get_unchecked(&self) -> &T {
        unsafe {
            self.inner.assume_init_ref()
        }
    }

    /// Gets a mutable reference without checking if its initialized.
    #[inline(always)]
    pub const unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        unsafe {
            self.inner.assume_init_mut()
        }
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
    pub fn set(&mut self, val: T) -> bool {
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
    ///
    /// If it is already initialized, it will drop the value first.
    #[inline(always)]
    pub fn force_set(&mut self, val: T) {
        if self.is_init() {
            unsafe {
                self.drop_unchecked();
            }
        }

        unsafe {
            self.inner.as_mut_ptr().write(val);
        }
    }

    /// Sets the value to null.
    #[inline(always)]
    pub fn nullify(&mut self) {
        if self.is_init() {
            unsafe {
                self.drop_unchecked();
            }

            unsafe {
                core::ptr::write_bytes(
                    self.inner.as_mut_ptr().cast::<u8>(),
                    0,
                    size_of::<T>()
                );
            }
        }
    }

    /// Sets the value to null.
    /// 
    /// # Safety
    ///
    /// Does not drop the value if it is initialized and does not check if it is already zeroed.
    /// An unsafe (and constant) variant of [`Self::nullify`].
    #[inline(always)]
    pub const unsafe fn nullify_unchecked(&mut self) {
        unsafe {
            core::ptr::write_bytes(
                self.inner.as_mut_ptr().cast::<u8>(),
                0,
                size_of::<T>()
            );
        }
    }

    /// Drops the value.
    ///
    /// # Safety
    ///
    /// Does not check if it is initialized and does not nullify it. An unsafe variant of [`Self::nullify`].
    #[inline(always)]
    pub unsafe fn drop_unchecked(&mut self) {
        unsafe {
            self.inner.assume_init_drop();
        }
    }

    /// Matches the value over two callbacks.
    ///
    /// If initialized, call `if_init`. If null, call `if_null`.
    pub fn match_null_ref<R>(&self, if_init: impl FnOnce(&T) -> R, if_null: impl FnOnce() -> R) -> R {
        if self.is_init() {
            unsafe {
                if_init(self.get_unchecked())
            }
        } else {
            if_null()
        }
    }

    /// Matches the value over two callbacks.
    ///
    /// If initialized, call `if_init`. If null, call `if_null`.
    pub fn match_null_mut<R>(&mut self, if_init: impl FnOnce(&mut T) -> R, if_null: impl FnOnce() -> R) -> R {
        if self.is_init() {
            unsafe {
                if_init(self.get_mut_unchecked())
            }
        } else {
            if_null()
        }
    }

    /// Matches the value over two callbacks.
    ///
    /// If initialized, call `if_init`. If null, call `if_null`.
    pub fn match_null<R>(self, if_init: impl FnOnce(T) -> R, if_null: impl FnOnce() -> R) -> R {
        if self.is_init() {
            unsafe {
                if_init(self.into_inner_unchecked())
            }
        } else {
            if_null()
        }
    }
}

impl<T: NonNullable> Drop for MaybeNull<T> {
    #[inline(always)]
    fn drop(&mut self) {
        if self.is_init() {
            unsafe {
                self.drop_unchecked();
            }
        }
    }
}

/// Matches a `MaybeNull` value.
///
/// Usage:
///
/// ```rust
/// let num = core::num::NonZeroU8::new(123).unwrap();
/// let maybe_null = briny::raw::nonzero::MaybeNull::new(num);
///
/// let string = briny::match_null!(
///     match &maybe_null {
///         Init(val) => { format!("{val}") }
///         Null => { "null".to_string() }
///     }
/// );
///
/// assert_eq!(string, "123");
/// ```
#[macro_export]
macro_rules! match_null {
    (
        match $maybe_null:ident {
            Init($init_val:ident) => $if_init:block
            Null => $if_null:block
        }
    ) => {
        if $maybe_null.is_init() {
            let $init_val = unsafe { $maybe_null.into_inner_unchecked() };
            $if_init
        } else $if_null
    };

    (
        match $maybe_null:ident {
            Null => $if_null:block
            Init($init_val:ident) => $if_init:block
        }
    ) => {
        if $maybe_null.is_init() {
            let $init_val = unsafe { $maybe_null.into_inner_unchecked() };
            $if_init
        } else $if_null
    };

    (
        match &$maybe_null:ident {
            Init($init_val:ident) => $if_init:block
            Null => $if_null:block
        }
    ) => {
        if $maybe_null.is_init() {
            let $init_val = unsafe { $maybe_null.get_unchecked() };
            $if_init
        } else $if_null
    };

    (
        match &$maybe_null:ident {
            Null => $if_null:block
            Init($init_val:ident) => $if_init:block
        }
    ) => {
        if $maybe_null.is_init() {
            let $init_val = unsafe { $maybe_null.get_unchecked() };
            $if_init
        } else $if_null
    };

    (
        match &mut $maybe_null:ident {
            Init($init_val:ident) => $if_init:block
            Null => $if_null:block
        }
    ) => {
        if $maybe_null.is_init() {
            let $init_val = unsafe { $maybe_null.get_mut_unchecked() };
            $if_init
        } else $if_null
    };

    (
        match &mut $maybe_null:ident {
            Null => $if_null:block
            Init($init_val:ident) => $if_init:block
        }
    ) => {
        if $maybe_null.is_init() {
            let $init_val = unsafe { $maybe_null.get_mut_unchecked() };
            $if_init
        } else $if_null
    };
}
