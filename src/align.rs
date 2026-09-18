//! Constants and alignment functions to help manage memory.

/// 1 KiB, 1024 bytes
pub const KIB: u64 = 1024;
/// 1 MiB, 1024 [`KIB`]
pub const MIB: u64 = 1024 * 1024;
/// 1 GiB, 1024 [`MIB`]
pub const GIB: u64 = 1024 * 1024 * 1024;
/// 1 TiB, 1024 [`GIB`]
pub const TIB: u64 = 1024 * 1024 * 1024 * 1024;

/// 1 KB, 1000 bytes
pub const KB: u64 = 1000;
/// 1 MB, 1000 [`KB`]
pub const MB: u64 = 1000 * 1000;
/// 1 GB, 1000 [`MB`]
pub const GB: u64 = 1000 * 1000 * 1000;
/// 1 TB, 1000 [`GB`].
pub const TB: u64 = 1000 * 1000 * 1000 * 1000;

macro_rules! make_align_fn {
    ($align_up:ident, $align_down:ident, $type:ty) => {
        /// Aligns an address up such that `out > addr`.
        #[inline(always)]
        #[must_use]
        pub const fn $align_up(addr: $type, align: $type) -> $type {
            (addr | (align - 1)) + 1
        }

        /// Aligns an address down such that `out <= addr`.
        #[inline(always)]
        #[must_use]
        pub const fn $align_down(addr: $type, align: $type) -> $type {
            addr & (align - 1)
        }
    };
}

make_align_fn!(align_up, align_down, usize);
make_align_fn!(align_up_u128, align_down_u128, u128);
make_align_fn!(align_up_u64, align_down_u64, u64);
make_align_fn!(align_up_u32, align_down_u32, u32);
make_align_fn!(align_up_u16, align_down_u16, u16);
make_align_fn!(align_up_u8, align_down_u8, u8);

/// Aligns an immutable pointer up such that `out <= addr`.
#[inline(always)]
#[must_use]
pub fn align_up_ptr<T>(addr: *const T, align: usize) -> *const T {
    const {
        assert!(align_of::<T>() == 1);
    }

    addr.with_addr(align_up(addr.addr(), align))
}

/// Aligns an immutable pointer down such that `out <= addr`.
#[inline(always)]
#[must_use]
pub fn align_down_ptr<T>(addr: *const T, align: usize) -> *const T {
    const {
        assert!(align_of::<T>() == 1);
    }

    addr.with_addr(align_down(addr.addr(), align))
}

/// Takes an unaligned immutable pointer and produces an aligned one such that `out > addr`.
///
/// This is probably not a valid object.
#[inline(always)]
#[must_use]
pub fn align_up_ptr_aligned<T>(addr: *const T) -> *const T {
    addr.with_addr(align_up(addr.addr(), align_of::<T>()))
}

/// Takes an unaligned immutable pointer and produces an aligned one such that `out <= addr`.
///
/// This is probably not a valid object.
#[inline(always)]
#[must_use]
pub fn align_down_ptr_aligned<T>(addr: *const T) -> *const T {
    addr.with_addr(align_down(addr.addr(), align_of::<T>()))
}

/// Aligns a mutable pointer up such that `out <= addr`.
#[inline(always)]
#[must_use]
pub fn align_up_mut_ptr<T>(addr: *mut T, align: usize) -> *mut T {
    const {
        assert!(align_of::<T>() == 1);
    }

    addr.with_addr(align_up(addr.addr(), align))
}

/// Aligns a mutable pointer down such that `out <= addr`.
#[inline(always)]
#[must_use]
pub fn align_down_mut_ptr<T>(addr: *mut T, align: usize) -> *mut T {
    const {
        assert!(align_of::<T>() == 1);
    }

    addr.with_addr(align_down(addr.addr(), align))
}

/// Takes an unaligned mutable pointer and produces an aligned one such that `out > addr`.
///
/// This is probably not a valid object.
#[inline(always)]
#[must_use]
pub fn align_up_ptr_mut_aligned<T>(addr: *mut T) -> *mut T {
    addr.with_addr(align_up(addr.addr(), align_of::<T>()))
}

/// Takes an unaligned mutable pointer and produces an aligned one such that `out <= addr`.
///
/// This is probably not a valid object.
#[inline(always)]
#[must_use]
pub fn align_down_ptr_mut_aligned<T>(addr: *mut T) -> *mut T {
    addr.with_addr(align_down(addr.addr(), align_of::<T>()))
}
