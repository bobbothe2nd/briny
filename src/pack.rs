//! Basic traits for easy binary serialization and optionally compression.
//!
//! Pain is not a prerequisite: no forced validation or complicated methods.

use crate::raw::ptr::ImpConst;

/// Trait for data that can be packed or compressed.
pub trait Pack {
    /// Method to pack data.
    fn pack<'a, T: Pack + Unpack>(&self) -> Packed<'a, T>;

    /// Method to compress data.
    fn compress<'a, T: Pack + Unpack>(&self) -> Packed<'a, T> {
        self.pack()
    }
}

/// Trait for data that can be unpacked or decompressed.
pub trait Unpack {
    /// Method to unpack data.
    fn unpack<'a, T: Unpack + Pack>(&self) -> Unpacked<'a, T>;

    /// Method to decompress data.
    fn decompress<'a, T: Unpack + Pack>(&self) -> Unpacked<'a, T> {
        self.unpack()
    }
}

/// A wrapper for packed data.
///
/// Holds a pointer to the inner value.
pub struct Packed<'a, T: Pack + Unpack> {
    ptr: ImpConst<'a, T>,
}

impl<'a, T: Pack + Unpack> Packed<'a, T> {
    /// Creates a new `Packed` structure with a pointer to the provided data.
    pub const fn new(data: &'a T) -> Self {
        let ptr = ImpConst::new(data);
        Self { ptr }
    }

    /// Provides a reference to the value the pointer is pointing to.
    #[must_use]
    pub const fn as_ref(&self) -> &T {
        self.ptr.as_ref()
    }
}

/// A wrapper for unpacked data.
///
/// Holds a pointer to the inner value.
pub struct Unpacked<'a, T: Pack> {
    data: ImpConst<'a, T>,
}

impl<'a, T: Pack + Unpack> Unpacked<'a, T> {
    /// Creates a new `Unpacked` structure with a pointer to the provided data.
    pub const fn new(data: &'a T) -> Self {
        let ptr = ImpConst::new(data);
        Self { data: ptr }
    }

    /// Provides a reference to the value the pointer is pointing to.
    #[must_use]
    pub const fn as_ref(&self) -> &T {
        self.data.as_ref()
    }
}
