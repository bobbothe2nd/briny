use crate::{traits::Pod, BrinyError};
use core::{mem, ptr, slice};

#[inline(always)]
pub fn slice_to_bytes<T: Pod>(slice: &[T]) -> &[u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    let ptr = slice.as_ptr().cast::<u8>();
    let len = mem::size_of_val(slice);
    unsafe { slice::from_raw_parts(ptr, len) }
}

#[inline(always)]
pub const fn slice_to_bytes_mut<T: Pod>(slice: &mut [T]) -> &mut [u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    let ptr = slice.as_mut_ptr().cast::<u8>();
    let len = size_of::<T>() * slice.len();
    unsafe { slice::from_raw_parts_mut(ptr, len) }
}

#[inline(always)]
pub const fn to_bytes<T: Pod>(input: &T) -> &[u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    unsafe { slice::from_raw_parts(ptr::from_ref::<T>(input).cast::<u8>(), size_of::<T>()) }
}

#[inline(always)]
pub const fn to_bytes_mut<T: Pod>(input: &mut T) -> &mut [u8] {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    unsafe { slice::from_raw_parts_mut(ptr::from_mut::<T>(input).cast::<u8>(), size_of::<T>()) }
}

#[inline(always)]
pub fn slice_from_bytes<T: Pod>(bytes: &[u8]) -> Result<&[T], BrinyError> {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    let mut err = BrinyError::RESERVED;

    let elem_size = size_of::<T>();

    if bytes.len() % elem_size != 0 {
        err = err.add(BrinyError::UNALIGNED_ACCESS);
    }

    let ptr = bytes.as_ptr();
    if (ptr as usize) % align_of::<T>() != 0 {
        err = err.add(BrinyError::UNALIGNED_ACCESS);
    }

    if err.is_err() {
        return Err(err);
    }

    let len = bytes.len() / elem_size;

    let t_ptr = ptr.cast();
    Ok(unsafe { slice::from_raw_parts(t_ptr, len) })
}

#[inline(always)]
pub fn from_bytes<T: Pod>(bytes: &[u8]) -> Result<T, BrinyError> {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    let mut err = BrinyError::RESERVED;
    if bytes.len() != size_of::<T>() {
        err = err.add(BrinyError::SIZE_BOUND_FAILURE);
    }
    if (bytes.as_ptr() as usize) % align_of::<T>() != 0 {
        err = err.add(BrinyError::UNALIGNED_ACCESS);
    }
    if err.is_err() {
        return Err(err);
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

#[inline(always)]
pub const fn from_bytes_unaligned<T: Pod>(bytes: &[u8]) -> Result<T, BrinyError> {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
    }

    if bytes.len() != mem::size_of::<T>() {
        return Err(BrinyError::SIZE_BOUND_FAILURE);
    }

    // copy bytes into temporary buffer
    let mut tmp = mem::MaybeUninit::<T>::uninit();
    unsafe {
        ptr::copy_nonoverlapping(
            bytes.as_ptr(),
            tmp.as_mut_ptr().cast::<u8>(),
            mem::size_of::<T>(),
        );

        Ok(tmp.assume_init())
    }
}

#[inline(always)]
pub const fn cast<T: Pod, U: Pod>(input: &T) -> U {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
        assert!(size_of::<T>() == size_of::<U>(), "cannot cast between types of different sizes");
    }

    let src_as_u = ptr::from_ref(input).cast::<U>();
    let val = unsafe { ptr::read_unaligned(src_as_u) };
    val
}

#[inline(always)]
pub const fn cast_mut<T: Pod, U: Pod>(input: &mut T) -> U {
    const {
        assert!(size_of::<T>() > 0, "cannot cast between ZSTs");
        assert!(size_of::<T>() == size_of::<U>(), "cannot cast between types of different sizes");
    }

    let src_as_u = ptr::from_ref(input).cast::<U>();
    let val = unsafe { ptr::read_unaligned(src_as_u) };
    val
}

#[inline(always)]
pub const fn cast_slice<T: Pod, U: Pod>(input: &[T]) -> &[U] {
    const {
        assert!(size_of::<T>() > 0 && size_of::<U>() > 0, "cannot cast between ZSTs");
        assert!(align_of::<T>() == align_of::<U>(), "cannot cast unaligned slices");
    }

    let len = size_of_val(input) / size_of::<U>();
    let src_as_u = input.as_ptr().cast::<U>();
    let val = unsafe { slice::from_raw_parts(src_as_u, len) };
    val
}

#[inline(always)]
pub const fn cast_slice_mut<T: Pod, U: Pod>(input: &mut [T]) -> &mut [U] {
    const {
        assert!(size_of::<T>() > 0 && size_of::<U>() > 0, "cannot cast between ZSTs");
        assert!(align_of::<T>() == align_of::<U>(), "cannot cast unaligned slices");
    }

    let len = size_of_val(input) / size_of::<U>();
    let src_as_u = input.as_mut_ptr().cast::<U>();
    let val = unsafe { slice::from_raw_parts_mut(src_as_u, len) };
    val
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
        // stack buffer: 12 bytes (to allow misalignment)
        let mut buf = [0u8; 12];

        // deliberately misalign by 1
        let misaligned_ptr = buf.as_mut_ptr().wrapping_add(1);

        // make a slice of 8 bytes from misaligned pointer
        let slice = unsafe { slice::from_raw_parts(misaligned_ptr, 8) };

        // should error because of misalignment
        let result = crate::raw::slice_from_bytes::<u32>(slice);
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
        // aligned bytes
        let val = 42u32;

        let bytes = val.to_le_bytes();
        let result = from_bytes_unaligned::<u32>(&bytes).unwrap();
        assert_eq!(result, val);

        // unaligned slice in a bigger array
        let mut buffer = [0u8; 8];
        buffer[1..5].copy_from_slice(&val.to_le_bytes());
        let slice = &buffer[1..5];
        let result = from_bytes_unaligned::<u32>(slice).unwrap();
        assert_eq!(result, val);
    }
}
