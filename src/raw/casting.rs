//! Somewhat safe functions to cast very specific yet paartially arbitrary types `T` to `U`.
//!
//! Moat functions include alignment checks

use crate::{BrinyError, raw::Pod};
use core::mem::MaybeUninit;

/// Converts a slice `&[T]` to `&[u8]` (single bytes), implicitly guaranteeing alignment.
///
/// # Safety
///
/// Any valid data type can be converted to an array of `u8`, as it is but a single byte. Since bytes are the smallest unit computers can operate on, alignment is implicitly guaranteed.
#[inline(never)]
pub const fn slice_to_bytes<T: Copy + Pod>(slice: &[T]) -> &[u8] {
    let ptr = slice.as_ptr().cast::<u8>();
    let len = core::mem::size_of_val(slice);
    unsafe { core::slice::from_raw_parts(ptr, len) }
}

/// Converts `T` to `&[u8]` (byte slices), imp;icitly guaranteeing alignment.
///
/// # Safety
///
/// Any valid data type can be converted to an array of `u8`, as it is but a single byte. Since bytes are the smallest unit computers can operate on, alignment is implicitly guaranteed.
pub const fn to_bytes<T: Pod>(input: &T) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(core::ptr::from_ref::<T>(input).cast::<u8>(), size_of::<T>())
    }
}

/// Converts a slice `&[u8]` (byte slices) to `&[T]`, assuming they are properly aligned.
///
/// # Safety
///
/// Internally, this function uses `from_raw_parts`, an function. However, safety is guaranteed by asserting the following conditions:
///
/// - Length has been verified.
/// - &data ensures the memory lies within a valid single allocation.
/// - `&[u8]` was immutable, so creating `&[T]` from it is sound for Copy types if their bit-patterns are valid.
/// - But note: this doesn't guarantee validity of the bit pattern for all `T`. Thats why its restricted to `T: Copy + Pod`.
///
/// # Errors
///
/// A `BrinyError` is returned under the condition that data is not aligned to type `T`.
#[inline(never)]
pub fn slice_from_bytes<T: Pod>(bytes: &[u8]) -> Result<&[T], BrinyError> {
    let size = size_of::<T>();
    let align = align_of::<T>();

    if bytes.len() % size != 0 || (bytes.as_ptr() as usize) % align != 0 {
        return Err(BrinyError);
    }

    let len = bytes.len() / size;
    let ptr = bytes.as_ptr().cast::<T>();

    for i in 0..len {
        let raw = unsafe { core::slice::from_raw_parts(ptr.add(i).cast::<u8>(), size) };

        if !T::is_valid_bitpattern(raw) {
            return Err(BrinyError);
        }
    }

    Ok(unsafe { core::slice::from_raw_parts(ptr, len) })
}

/// Converts a slice `&[u8]` (byte slices) to `T`, assuming they are properly aligned.
///
/// # Safety
///
/// Internally, this function derenferences a raw pointer and dynamically casts types. However, safety is guaranteed by asserting the following conditions:
///
/// - Length has been verified.
/// - &data ensures the memory lies within a valid single allocation.
/// - `&[u8]` was immutable, so creating `T` from it is sound for `Copy` implementing types if their bit-patterns are valid.
/// - Padding was checked.
///
/// ...but this doesn't guarantee validity of the bit pattern for all `T`. Thats why its restricted to `T: Copy + Pod`.
///
/// However, even after all of that, it is assumed that the data is still unaligned and read safely with that assumption in mind at all times.
///
/// # Errors
///
/// A `BrinyError` is returned under the condition that data is not aligned to type `T` or the bitpatterns do not match.
#[inline(never)]
pub fn from_bytes<T: Pod>(bytes: &[u8]) -> Result<T, BrinyError> {
    if bytes.len() != size_of::<T>() || (bytes.as_ptr() as usize) % align_of::<T>() != 0 {
        return Err(BrinyError);
    }

    if !T::is_valid_bitpattern(bytes) {
        return Err(BrinyError);
    }

    let mut tmp = MaybeUninit::<T>::uninit();
    unsafe {
        core::ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            tmp.as_mut_ptr().cast::<u8>(),
            size_of::<T>(),
        );
        Ok(tmp.assume_init())
    }
}

/// Casts data of type `T` to type `U` without changing the value.
///
/// # Safety
///
/// Internally, raw pointers are derenferenced and cast between types, but safety is guaranteed because the following conditions are mets:
///
/// - `T` and `U` are verified to have compatible bitpatterns
/// - `T` and `U` have the same alignment
/// - `T` cannot be smaller than `U`
/// - `T` and `U` implement `Copy + Pod`
///
/// However, even after all of that, it is assumed that the data is still unaligned and read safely with that assumption in mind at all times.
///
/// # Errors
///
/// In such a case that `T` is not aligned to `U`, they cannot be cast directly, and so `BrinyError` is returned from this function.
#[inline(never)]
pub fn cast<T: Pod, U: Pod>(input: &T) -> Result<U, BrinyError> {
    if size_of::<T>() != size_of::<U>() || align_of::<T>() < align_of::<U>() {
        return Err(BrinyError);
    }

    let input_bytes = unsafe {
        core::slice::from_raw_parts(core::ptr::from_ref::<T>(input).cast::<u8>(), size_of::<T>())
    };

    if !U::is_valid_bitpattern(input_bytes) {
        return Err(BrinyError);
    }

    let mut tmp = MaybeUninit::<U>::uninit();
    unsafe {
        core::ptr::copy_nonoverlapping(
            input_bytes.as_ptr(),
            tmp.as_mut_ptr().cast::<u8>(),
            size_of::<U>(),
        );
        Ok(tmp.assume_init())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C, align(8))]
    #[derive(Copy, Clone, Debug, PartialEq)]
    struct ThePod {
        a: u16,
        b: u32,
    }

    unsafe impl Pod for ThePod {}

    impl crate::SafeMemory for ThePod {
        const _ASSERTIONS: () = {
            assert!(align_of::<ThePod>() == 8);
            assert!(size_of::<ThePod>() == size_of::<u64>());
        };
    }

    #[test]
    fn static_mut_slice_from_bytes_misaligned() {
        static mut BUF: [u8; 8 + core::mem::align_of::<u32>()] = [0; 8 + 4];

        let ptr = unsafe {
            #[allow(static_mut_refs)]
            BUF.as_mut_ptr().add(1)
        }; // misalign by 1
        let slice = unsafe { core::slice::from_raw_parts(ptr, 8) };
        let result = slice_from_bytes::<u32>(slice);
        assert!(result.is_err());
    }

    #[test]
    fn to_bytes_roundtrip() {
        let val = 0x12345678u32;
        let bytes = slice_to_bytes(core::slice::from_ref(&val));
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
    fn slice_from_bytes_invalid() {
        let arr = [0u8; 7]; // intentionally misaligned
        assert!(slice_from_bytes::<u32>(&arr).is_err());
    }

    #[test]
    fn slice_from_bytes_misaligned() {
        let buf = [0u8; 12];
        let misaligned = &buf[1..]; // misalign by 1 byte
        assert!(slice_from_bytes::<u32>(misaligned).is_err());
    }

    #[test]
    fn from_bytes_invalid_length() {
        let arr = [0xFFu8; 3]; // not aligned to any common type
        assert!(from_bytes::<u32>(&arr).is_err());
    }

    #[test]
    fn cast_between_same_size_types() {
        let original: u32 = 0xDEADBEEF;
        let casted = cast::<u32, f32>(&original).unwrap();
        let restored = cast::<f32, u32>(&casted).unwrap();
        assert_eq!(restored, original);
    }

    #[test]
    fn custom_struct_bytes_roundtrip() {
        let pod = ThePod {
            a: 0xABCD,
            b: 0x12345678,
        };
        let bytes = slice_to_bytes(core::slice::from_ref(&pod));
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
        let raw: u64 = cast(&pod).unwrap();
        let back: ThePod = cast(&raw).unwrap();
        assert_eq!(pod, back);
    }

    #[test]
    fn invalid_cast_size_mismatch() {
        let val = 0x1234u16;
        let result = cast::<u16, u32>(&val);
        assert!(result.is_err());
    }
}
