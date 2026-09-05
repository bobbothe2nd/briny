//! Casting primitive operations.

use crate::{
    traits::{Layout, RawConvert, StableLayout},
    BrinyError,
};
use core::{
    mem::{self, ManuallyDrop},
    ptr, slice,
};

/// Reinterpret the bytes of `T` as `U` *without copying* them.
///
/// This does NOT drop the value of `input`. Instead, it just reinterprets the bytes as type `U`.
#[inline(always)]
pub const fn reinterpret<T: Layout<U>, U: StableLayout>(input: T) -> U {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
    }

    unsafe { reinterpret_unchecked::<T, U>(input) }
}

/// Reinterpret the bytes of `T` as `U` *without copying* them.
///
/// This does NOT drop the value of `input`. Instead, it just reinterprets the bytes as type `U`.
#[inline(always)]
pub const unsafe fn reinterpret_unchecked<T, U>(input: T) -> U {
    union Reinterpret<T, U> {
        t: ManuallyDrop<T>,
        u: ManuallyDrop<U>,
    }

    let u = Reinterpret {
        t: ManuallyDrop::new(input),
    };

    unsafe { ManuallyDrop::into_inner(u.u) }
}

/// Converts any slice to bytes.
#[inline(always)]
pub const fn slice_to_bytes<T: RawConvert>(slice: &[T]) -> &[u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    let ptr = slice.as_ptr().cast::<u8>();
    let len = size_of_val(slice);
    unsafe { slice::from_raw_parts(ptr, len) }
}

/// Converts any mutable slice to bytes.
#[inline(always)]
pub const fn slice_to_bytes_mut<T: RawConvert>(slice: &mut [T]) -> &mut [u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    let ptr = slice.as_mut_ptr().cast::<u8>();
    let len = size_of_val(slice);
    unsafe { slice::from_raw_parts_mut(ptr, len) }
}

/// Converts any reference to a `RawConvert` type to bytes.
#[inline(always)]
pub const fn to_bytes<T: RawConvert>(input: &T) -> &[u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    unsafe { slice::from_raw_parts(ptr::from_ref::<T>(input).cast::<u8>(), size_of::<T>()) }
}

/// Converts any mutable reference to a `RawConvert` type to bytes.
#[inline(always)]
pub const fn to_bytes_mut<T: RawConvert>(input: &mut T) -> &mut [u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    unsafe { slice::from_raw_parts_mut(ptr::from_mut::<T>(input).cast::<u8>(), size_of::<T>()) }
}

/// Attempts to get a slice from raw bytes.
///
/// # Errors
///
/// Instead of causing undefined behavior or panicking, this function returns an error
/// when `bytes` is invalid (incorrect size or unaligned).
#[inline(always)]
pub const fn slice_from_bytes<T: RawConvert>(bytes: &[u8]) -> Result<&[T], BrinyError> {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    let elem_size = size_of::<T>();

    if !bytes.len().is_multiple_of(elem_size) {
        return Err(BrinyError::UNALIGNED_ACCESS);
    }

    let ptr = bytes.as_ptr();

    let len = bytes.len() / elem_size;

    let t_ptr = ptr.cast();
    Ok(unsafe { slice::from_raw_parts(t_ptr, len) })
}

/// Attempts to get a value from raw bytes.
///
/// # Errors
///
/// Instead of causing undefined behavior or panicking, this function returns an error
/// when `bytes` is invalid (incorrect size or unaligned).
#[inline(always)]
pub fn from_bytes<T: RawConvert>(bytes: &[u8]) -> Result<T, BrinyError> {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    if bytes.len() != size_of::<T>() {
        return Err(BrinyError::SIZE_BOUND_FAILURE);
    }

    let mut tmp = mem::MaybeUninit::<T>::uninit();
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            tmp.as_mut_ptr().cast::<u8>(),
            size_of::<T>(),
        );
        Ok(tmp.assume_init())
    }
}

/// Attempts to get a value from raw bytes without requiring alignment.
///
/// # Errors
///
/// Instead of causing undefined behavior or panicking, this function returns an error
/// when `bytes` is invalid (incorrect size).
#[inline(always)]
pub const fn from_bytes_unaligned<T: RawConvert>(bytes: &[u8]) -> Result<T, BrinyError> {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    if bytes.len() != size_of::<T>() {
        return Err(BrinyError::SIZE_BOUND_FAILURE);
    }

    let mut tmp = mem::MaybeUninit::<T>::uninit();
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            tmp.as_mut_ptr().cast::<u8>(),
            size_of::<T>(),
        );

        Ok(tmp.assume_init())
    }
}

/// Casts between two references to `Pod` types.
#[inline(always)]
pub const fn cast<T: Layout<U>, U: StableLayout>(input: &T) -> U {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
        assert!(
            align_of::<T>() >= align_of::<U>(),
            "cannot cast unaligned types"
        );
    }

    let src_as_u = ptr::from_ref(input).cast::<U>();
    unsafe { ptr::read_unaligned(src_as_u) }
}

/// Casts between two mutable references to `Pod` types.
#[inline(always)]
pub const fn cast_mut<T: Layout<U>, U: StableLayout>(input: &mut T) -> U {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
        assert!(
            size_of::<T>() == size_of::<U>(),
            "cannot cast between types of different sizes"
        );
        assert!(
            align_of::<T>() >= align_of::<U>(),
            "cannot cast unaligned types"
        );
    }

    let src_as_u = ptr::from_ref(input).cast::<U>();
    unsafe { ptr::read_unaligned(src_as_u) }
}

/// Casts between two immutable slices of different types.
#[inline(always)]
pub const fn cast_slice<T: Layout<U>, U: StableLayout>(input: &[T]) -> &[U] {
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

    let len = size_of_val(input) / size_of::<U>();
    let src_as_u = input.as_ptr().cast::<U>();
    unsafe { slice::from_raw_parts(src_as_u, len) }
}

/// Casts between two mutable slices of different types.
#[inline(always)]
pub const fn cast_slice_mut<T: Layout<U>, U: StableLayout>(input: &mut [T]) -> &mut [U] {
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

    let len = size_of_val(input) / size_of::<U>();
    let src_as_u = input.as_mut_ptr().cast::<U>();
    unsafe { slice::from_raw_parts_mut(src_as_u, len) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C, align(8))]
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct ThePod {
        a: u32,
        b: u32,
    }

    unsafe impl crate::traits::RawConvert for ThePod {}
    unsafe impl crate::traits::StableLayout for ThePod {}
    unsafe impl crate::traits::Pod for ThePod {}

    #[test]
    fn stack_misaligned_slice_from_bytes() {
        let buf = [0u8; 11];

        let result = slice_from_bytes::<u32>(&buf);
        assert!(result.is_err());
    }

    #[test]
    fn to_bytes_roundtrip() {
        let val = 0x12345678u32;
        let bytes = slice_to_bytes(slice::from_ref(&val));
        let restored = from_bytes::<u32>(bytes).unwrap();
        assert_eq!(val, restored);
    }

    #[test]
    fn slice_from_bytes_valid() {
        let arr = [1u32, 2, 3];
        let bytes = to_bytes(&arr);
        let restored = slice_from_bytes::<u32>(bytes).unwrap();
        assert_eq!(restored, &arr);
    }

    #[test]
    fn cast_between_same_size_types() {
        let original: u32 = 0xDEADBEEF;
        let casted = cast::<u32, f32>(&original);
        let restored = cast::<f32, u32>(&casted);
        assert_eq!(restored, original);
    }

    #[test]
    fn reinterpret_same_size_types() {
        let original: u32 = 0xDEADBEEF;
        let casted = reinterpret::<u32, f32>(original);
        let restored = reinterpret::<f32, u32>(casted);
        assert_eq!(restored, original);
    }

    #[test]
    fn custom_struct_bytes_roundtrip() {
        let pod = ThePod {
            a: 0xABCD,
            b: 0x12345678,
        };
        let bytes = slice_to_bytes(slice::from_ref(&pod));
        let restored: ThePod = from_bytes(bytes).unwrap();
        assert_eq!(pod, restored);
    }

    #[test]
    fn cast_struct_to_u64_and_back() {
        assert_eq!(align_of::<ThePod>(), 8);

        let pod = ThePod {
            a: 0x1122,
            b: 0x33445566,
        };
        let raw: u64 = cast(&pod);
        let back: ThePod = cast(&raw);
        assert_eq!(pod, back);
    }

    #[test]
    fn from_bytes_unaligned_safety() {
        let val = 42u32;

        let bytes = val.to_le_bytes();
        let result = from_bytes_unaligned::<u32>(&bytes).unwrap();
        assert_eq!(result, val);

        let mut buffer = [0u8; 8];
        buffer[1..5].copy_from_slice(&val.to_le_bytes());
        let slice = &buffer[1..5];
        let result = from_bytes_unaligned::<u32>(slice).unwrap();
        assert_eq!(result, val);
    }
}
