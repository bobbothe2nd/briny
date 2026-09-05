//! Cast allocations and efficiently zero them.

use alloc::vec::Vec;

use crate::traits::{Layout, StableLayout};

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

    core::mem::forget(input);

    unsafe { Vec::from_raw_parts(src_as_u, len, cap) }
}
