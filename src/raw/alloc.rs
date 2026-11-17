//! Cast allocations and efficiently zero them.

use core::mem::forget;

use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};

use crate::{
    raw::cast::cast_slice,
    traits::{Layout, NoUninit, Pod, ReprSafe, Zeroable},
};

/// Casts between two `Vec`s of different types.
#[must_use]
#[inline(always)]
pub fn cast_vec<T, U>(mut input: Vec<T>) -> Vec<U>
where
    T: NoUninit + Layout<U>,
    U: ReprSafe,
{
    const {
        assert!(
            size_of::<T>() != 0 && size_of::<U>() != 0,
            "cannot cast between ZSTs"
        );
        assert!(
            align_of::<T>() == align_of::<U>(),
            "original alignment must be at least as strict as cast"
        );

        assert!(
            T::HAS_NO_PADDING && U::HAS_NO_PADDING,
            "cannot cast between types with padding"
        );
        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
            stable_intr::assert_inhabited::<U>();
        }
    }

    let src_size = size_of_val(input.as_slice());

    let src_cap = input.capacity() * size_of::<T>();

    if src_cap == 0 {
        return Vec::new();
    }

    let dst_len = src_size.div_ceil(size_of::<U>());
    let dst_cap = src_cap.div_ceil(size_of::<U>());
    let src_as_u = input.as_mut_ptr().cast::<U>();

    forget(input);

    unsafe { Vec::from_raw_parts(src_as_u, dst_len, dst_cap) }
}

/// Casts between two [`Box`] pointers
#[must_use]
#[inline(always)]
pub fn cast_box<T, U>(input: Box<T>) -> Box<U>
where
    T: NoUninit + Layout<U>,
    U: ReprSafe,
{
    const {
        assert!(size_of::<T>() != 0, "cannot cast between ZSTs");
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
        assert!(
            align_of::<T>() == align_of::<U>(),
            "original alignment must be at least as strict as cast"
        );
    }

    let ptr = Box::into_raw(input).cast();

    unsafe { Box::from_raw(ptr) }
}

/// Casts between [`Arc`] pointers
#[must_use]
#[inline(always)]
pub fn cast_arc<T, U>(input: Arc<T>) -> Arc<U>
where
    T: NoUninit + Layout<U>,
    U: ReprSafe,
{
    const {
        assert!(
            size_of::<T>() != 0 && size_of::<U>() != 0,
            "cannot cast between ZSTs"
        );
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
        assert!(
            align_of::<T>() == align_of::<U>(),
            "original alignment must be at least as strict as cast"
        );

        assert!(
            T::HAS_NO_PADDING && U::HAS_NO_PADDING,
            "cannot cast between types with padding"
        );
        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
            stable_intr::assert_inhabited::<U>();
        }
    }

    let ptr = Arc::into_raw(input).cast();

    unsafe { Arc::from_raw(ptr) }
}

/// Casts between [`Rc`] pointers
#[must_use]
#[inline(always)]
pub fn cast_rc<T, U>(input: Rc<T>) -> Rc<U>
where
    T: NoUninit + Layout<U>,
    U: ReprSafe,
{
    const {
        assert!(
            size_of::<T>() != 0 && size_of::<U>() != 0,
            "cannot cast between ZSTs"
        );
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
        assert!(
            align_of::<T>() == align_of::<U>(),
            "original alignment must be at least as strict as cast"
        );

        assert!(
            T::HAS_NO_PADDING && U::HAS_NO_PADDING,
            "cannot cast between types with padding"
        );
        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
            stable_intr::assert_inhabited::<U>();
        }
    }

    let ptr = Rc::into_raw(input).cast();

    unsafe { Rc::from_raw(ptr) }
}

/// Creates a zeroed `Arc<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_arc<T: Zeroable>() -> Arc<T> {
    const {
        assert!(size_of::<T>() != 0, "cannot copy ZSTs");

        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
        }
    }

    unsafe { Arc::new_zeroed().assume_init() }
}

/// Creates a zeroed `Rc<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_rc<T: Zeroable>() -> Rc<T> {
    const {
        assert!(size_of::<T>() != 0, "cannot copy ZSTs");

        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
        }
    }

    unsafe { Rc::new_zeroed().assume_init() }
}

/// Creates a zeroed `Box<T>`
#[must_use]
#[inline(always)]
pub fn zeroed_box<T: Zeroable>() -> Box<T> {
    const {
        assert!(size_of::<T>() != 0, "cannot copy ZSTs");

        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
        }
    }

    unsafe { Box::new_zeroed().assume_init() }
}

/// Creates a zeroed `Arc<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_arc_slice<T: Zeroable>(len: usize) -> Arc<[T]> {
    const {
        assert!(size_of::<T>() != 0, "cannot copy ZSTs");

        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
        }
    }

    unsafe { Arc::new_zeroed_slice(len).assume_init() }
}

/// Creates a zeroed `Rc<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_rc_slice<T: Zeroable>(len: usize) -> Rc<[T]> {
    const {
        assert!(size_of::<T>() != 0, "cannot copy ZSTs");

        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
        }
    }

    unsafe { Rc::new_zeroed_slice(len).assume_init() }
}

/// Creates a zeroed `Box<[T]>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_box_slice<T: Zeroable>(len: usize) -> Box<[T]> {
    const {
        assert!(size_of::<T>() != 0, "cannot copy ZSTs");

        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
        }
    }

    unsafe { Box::new_zeroed_slice(len).assume_init() }
}

/// Creates a zeroed `Vec<T>` of length `len`
#[must_use]
#[inline(always)]
pub fn zeroed_vec<T: Zeroable>(len: usize) -> Vec<T> {
    zeroed_box_slice(len).into_vec()
}

/// Collects a slice of `Pod` types into a boxed slice of a different type
#[must_use]
#[inline(always)]
pub fn collect_reinterpret<T: Pod, U: Pod + Copy>(src: &[T]) -> Box<[U]> {
    const {
        assert!(
            size_of::<T>() != 0 && size_of::<U>() != 0,
            "cannot cast between ZSTs"
        );

        assert!(
            T::HAS_NO_PADDING && U::HAS_NO_PADDING,
            "cannot cast between types with padding"
        );
        #[cfg(feature = "stable_intr")]
        {
            stable_intr::assert_inhabited::<T>();
            stable_intr::assert_inhabited::<U>();
        }
    }

    let src_size = size_of_val(src);
    let dst_count = src_size.div_ceil(size_of::<U>());

    let mut dst = Box::<[U]>::new_uninit_slice(dst_count);

    unsafe {
        dst[dst.len() - 1]
            .as_mut_ptr()
            .write_bytes(0, size_of::<U>());
    }

    dst.write_copy_of_slice(cast_slice(src));
    unsafe { dst.assume_init() }
}
