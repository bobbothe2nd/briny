//! Core primitives, byte casting, and (mostly) safe wrappers for handling raw bytes.
//!
//! # Safety
//!
//! This is the only module containing unsafe code, but it has a lot of it! The unsafe code in this module is for good reason though - It allows for casting between arbitrary types and making safe abstractions over unsafe ones.
//!
//! Traits like `Pod` provide useful methods to handle this data safely.

mod cast;
pub use cast::{
    cast, cast_mut, from_bytes, from_bytes_unaligned, slice_from_bytes,
    slice_to_bytes, slice_to_bytes_mut, to_bytes, to_bytes_mut,
    cast_slice, cast_slice_mut,
};
