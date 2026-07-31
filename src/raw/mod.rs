//! Core primitives, byte casting, and safe wrappers for handling raw bytes.
//!
//! # Safety
//!
//! This is the only module containing unsafe code, but it has a lot of it! The unsafe code in this module is for good reason though - It allows for casting between arbitrary types and making safe abstractions over unsafe ones.
//!
//! Traits like `Pod` provide useful methods to handle this data safely.

pub mod cast;

pub mod nonzero;
