//! A module in which lives safe wrappers for raw pointers like `ImpConst` and `ImpMut`.

/// Internal Memory Pointer (Constant)
#[derive(Debug, Copy)]
#[repr(transparent)]
pub struct ImpConst<'a, T> {
    ptr: *const T,
    _marker: core::marker::PhantomData<&'a T>,
}

unsafe impl<T: Send> Send for ImpConst<'_, T> {}
unsafe impl<T: Sync> Sync for ImpConst<'_, T> {}

impl<T> Clone for ImpConst<'_, T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.as_ptr(),
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a, T> ImpConst<'a, T> {
    /// Creates a new constant pointer to the data provided.
    pub const fn new(data: &'a T) -> Self {
        Self {
            ptr: core::ptr::from_ref(data),
            _marker: core::marker::PhantomData,
        }
    }

    /// Creates a new constant pointer with the provided pointer.
    ///
    /// # Safety
    ///
    /// The pointer cannot be guaranteed valid, it is up to the caller to ensure safety.
    pub const unsafe fn from_ptr(ptr: *const T) -> Self {
        Self {
            ptr,
            _marker: core::marker::PhantomData,
        }
    }

    /// Provides a reference to the data pointed at.
    #[must_use]
    pub const fn as_ref(&self) -> &T {
        // SAFETY: ptr was derived from a valid `&T` for lifetime `'a`
        unsafe { &*self.ptr }
    }

    /// Provides the constant pointer as stored.
    #[must_use]
    pub const fn as_ptr(&self) -> *const T {
        self.ptr
    }

    #[must_use]
    /// Casts the constant pointer as mutable.
    ///
    /// # Safety
    ///
    /// The pointer held may not offer safe mutation. This is similar to calling `.as_ptr().cast_mut()` which gives you a `*mut T`,
    /// but instead of that, the pointer is wrapped in `ImpMut<T>`.
    pub const unsafe fn cast_mut(&self) -> ImpMut<'_, T> {
        unsafe { ImpMut::from_ptr(self.ptr.cast_mut()) }
    }
}

/// Internal Memory Pointer (Mutable)
#[repr(transparent)]
#[derive(Debug, Copy)]
pub struct ImpMut<'a, T> {
    ptr: *mut T,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<T> Clone for ImpMut<'_, T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.as_mut_ptr(),
            _marker: core::marker::PhantomData,
        }
    }
}

impl<'a, T> ImpMut<'a, T> {
    /// Creates a new mutable pointer to the provided data.
    pub const fn new(data: &'a mut T) -> Self {
        Self {
            ptr: core::ptr::from_mut::<T>(data),
            _marker: core::marker::PhantomData,
        }
    }

    /// Creates a new constant pointer with the provided pointer.
    ///
    /// # Safety
    ///
    /// The pointer cannot be guaranteed valid, it is up to the caller to ensure safety.
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr,
            _marker: core::marker::PhantomData,
        }
    }

    /// Provides a reference to the data pointed at.
    #[must_use]
    pub const fn as_ref(&self) -> &T {
        // SAFETY: ptr was derived from a valid `&mut T` for lifetime `'a`
        unsafe { &*self.ptr }
    }

    /// Provides a mutable reference to the data pointed at.
    #[must_use]
    pub const fn as_mut(&mut self) -> &mut T {
        // SAFETY: ptr was derived from `&'a mut T`, so no aliasing
        unsafe { &mut *self.ptr }
    }

    /// Provides the raw pointer as a constant.
    #[must_use]
    pub const fn as_ptr(&self) -> *const T {
        self.ptr.cast_const()
    }

    /// Provides the raw pointer as stored.
    #[must_use]
    pub const fn as_mut_ptr(&self) -> *mut T {
        self.ptr
    }
}
