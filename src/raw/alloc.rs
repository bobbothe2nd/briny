//! Cast allocations and efficiently zero them.

use core::mem::forget;

use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};

use crate::{
    raw::cast::{slice_to_bytes, slice_to_bytes_mut},
    traits::{Layout, Pod, StableLayout},
};

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
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
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

    unsafe { Box::from_raw(ptr) }
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

    unsafe { Arc::from_raw(ptr) }
}

/// Casts between [`Rc`] pointers
#[must_use]
#[inline(always)]
pub fn cast_rc<T: Layout<U>, U: StableLayout>(input: Rc<T>) -> Rc<U> {
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

    let ptr = Rc::into_raw(input).cast();

    unsafe { Rc::from_raw(ptr) }
}

/// Creates a zeroed `Arc<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_arc<T: Pod>() -> Arc<T> {
    unsafe { Arc::new_zeroed().assume_init() }
}

/// Creates a zeroed `Rc<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_rc<T: Pod>() -> Rc<T> {
    unsafe { Rc::new_zeroed().assume_init() }
}

/// Creates a zeroed `Box<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_box<T: Pod>() -> Box<T> {
    unsafe { Box::new_zeroed().assume_init() }
}

/// Creates a zeroed `Arc<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_arc_slice<T: Pod>(len: usize) -> Arc<[T]> {
    unsafe { Arc::new_zeroed_slice(len).assume_init() }
}

/// Creates a zeroed `Rc<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_rc_slice<T: Pod>(len: usize) -> Rc<[T]> {
    unsafe { Rc::new_zeroed_slice(len).assume_init() }
}

/// Creates a zeroed `Box<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_box_slice<T: Pod>(len: usize) -> Box<[T]> {
    unsafe { Box::new_zeroed_slice(len).assume_init() }
}

/// Creates a zeroed `Vec<T>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_vec<T: Pod>(len: usize) -> Vec<T> {
    zeroed_box_slice(len).into_vec()
}

/// Collects a slice of `Pod` types into a `Vec` of a different type
pub fn collect_reinterpret<T: Pod, U: Pod>(src: &[T]) -> Vec<U> {
    let src_size = size_of_val(src);

    let dst_pad = usize::from(!src_size.is_multiple_of(size_of::<U>()));
    let dst_count = src_size / size_of::<U>() + dst_pad;

    let mut dst: Vec<U> = zeroed_vec(dst_count);

    let src_bytes = slice_to_bytes(src);
    let dst_bytes = slice_to_bytes_mut(&mut dst[..]);

    dst_bytes[..src_size].copy_from_slice(src_bytes);

    dst
}
