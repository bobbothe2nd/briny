//! Cast allocations and efficiently zero them.

use core::mem::forget;

use alloc::{boxed::Box, sync::Arc, vec::Vec};

use crate::traits::{Layout, Pod, StableLayout};

/// Casts between two immutable slices of different types.
#[must_use]
#[inline(always)]
pub fn cast_vec<T: Layout<U>, U: StableLayout>(mut input: Vec<T>) -> Vec<U> {
    const {
        assert!(
            size_of::<T>() > 0 && size_of::<U>() > 0,
            "cannot cast between ZSTs"
        );
        assert!(
            align_of::<T>() >= align_of::<U>(),
            "original alignment must be at least as strict as cast"
        );
    }

    let input_len = size_of_val(input.as_slice());
    let input_cap = input.capacity() * size_of::<T>();

    if input_cap == 0 {
        return Vec::new();
    }

    let len = input_len / size_of::<U>();
    let cap = input_cap / size_of::<U>();
    let src_as_u = input.as_mut_ptr().cast::<U>();

    forget(input);

    unsafe { Vec::from_raw_parts(src_as_u, len, cap) }
}

/// Casts between two [`Box`] pointers
#[must_use]
#[inline(always)]
pub fn cast_box<T: Layout<U>, U: StableLayout>(input: Box<T>) -> Box<U> {
    const {
        assert!(
            size_of::<T>() > 0,
            "cannot cast between ZSTs"
        );
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
        assert!(
            align_of::<T>() >= align_of::<U>(),
            "original alignment must be at least as strict as cast"
        );
    }

    let ptr = Box::into_raw(input).cast();

    unsafe {
        Box::from_raw(ptr)
    }
}

    /// Casts between [`Arc`] pointers
#[must_use]
#[inline(always)]
pub fn cast_arc<T: Layout<U>, U: StableLayout>(input: Arc<T>) -> Arc<U> {
    const {
        assert!(
            size_of::<T>() > 0 && size_of::<U>() > 0,
            "cannot cast between ZSTs"
        );
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
        assert!(
            align_of::<T>() >= align_of::<U>(),
            "original alignment must be at least as strict as cast"
        );
    }

    let ptr = Arc::into_raw(input).cast();

    unsafe {
        Arc::from_raw(ptr)
    }
}

/// Creates a zeroed `Arc<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_arc<T: Pod>() -> Arc<T> {
    unsafe {
        Arc::new_zeroed().assume_init()
    }
}

/// Creates a zeroed `Box<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_box<T: Pod>() -> Box<T> {
    unsafe {
        Box::new_zeroed().assume_init()
    }
}

/// Creates a zeroed `Arc<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_arc_slice<T: Pod>(len: usize) -> Arc<[T]> {
    unsafe {
        Arc::new_zeroed_slice(len).assume_init()
    }
}

/// Creates a zeroed `Box<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_box_slice<T: Pod>(len: usize) -> Box<[T]> {
    unsafe {
        Box::new_zeroed_slice(len).assume_init()
    }
}

/// Creates a zeroed `Vec<T>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_vec<T: Pod>(len: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(len);

    let ptr: *mut T = vec.as_mut_ptr();

    unsafe {
        ptr.write_bytes(0, len * size_of::<T>());

        vec.set_len(len);
    }

    vec
}
